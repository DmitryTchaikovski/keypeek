use super::config::parse_config;
use crate::protocols::{DeviceError, DeviceEvent, EventSubscription, KeyboardProtocol, StopSignal};
use kanata_tcp_protocol::ServerMessage;
use keypeek_core::{KeyboardDefinition, KeymapSnapshot};
use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MAX_TCP_MESSAGE_SIZE: usize = 64 * 1024;

pub struct KanataProtocol {
    definition: KeyboardDefinition,
    snapshot: KeymapSnapshot,
    layer_names: Vec<String>,
    initial_layer: String,
    reader: Option<BufReader<TcpStream>>,
    stop_signal: StopSignal,
}

impl KanataProtocol {
    pub(super) fn connect(config_path: &str, address: &str) -> Result<Self, DeviceError> {
        let path = config_path.trim();
        if path.is_empty() {
            return Err(DeviceError::Protocol(
                "Select a Kanata configuration file before connecting".to_string(),
            ));
        }

        let config = kanata_parser::cfg::new_from_file(Path::new(path)).map_err(|e| {
            DeviceError::Protocol(format!("Failed to parse Kanata configuration: {e}"))
        })?;
        let (definition, snapshot, layer_names) = parse_config(config)?;

        let socket_address: SocketAddr = address.parse().map_err(|e| {
            DeviceError::Protocol(format!("Invalid Kanata TCP address '{address}': {e}"))
        })?;
        let stream =
            TcpStream::connect_timeout(&socket_address, Duration::from_secs(3)).map_err(|e| {
                DeviceError::Transport(format!(
                    "Could not connect to Kanata TCP server at {address}: {e}. \
                     Start Kanata with `--port 5829` and verify the configured address."
                ))
            })?;
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| DeviceError::Transport(format!("Failed to configure Kanata TCP: {e}")))?;

        let mut reader = BufReader::new(stream);
        let mut greeting = Vec::new();
        reader.read_until(b'\n', &mut greeting).map_err(|e| {
            DeviceError::Transport(format!(
                "Failed to read the initial Kanata TCP event from {address}: {e}"
            ))
        })?;
        if greeting.len() > MAX_TCP_MESSAGE_SIZE {
            return Err(DeviceError::Protocol(
                "Kanata TCP initial event exceeded the maximum message size".to_string(),
            ));
        }
        let initial_layer =
            match serde_json::from_slice::<ServerMessage>(&greeting).map_err(|e| {
                DeviceError::Protocol(format!("Invalid initial Kanata TCP message: {e}"))
            })? {
                ServerMessage::LayerChange { new } => new,
                _ => {
                    return Err(DeviceError::Protocol(
                        "Kanata TCP server did not send its current layer on connect".to_string(),
                    ));
                }
            };
        if !layer_names.iter().any(|name| name == &initial_layer) {
            return Err(DeviceError::Protocol(format!(
                "Kanata reports active layer '{initial_layer}', which is missing from the selected configuration"
            )));
        }

        Ok(Self {
            definition,
            snapshot,
            layer_names,
            initial_layer,
            reader: Some(reader),
            stop_signal: StopSignal::new(),
        })
    }
}

impl KeyboardProtocol for KanataProtocol {
    fn get_layout_definition(&self) -> &KeyboardDefinition {
        &self.definition
    }

    fn read_keymap(&self) -> Result<KeymapSnapshot, DeviceError> {
        Ok(self.snapshot.clone())
    }

    fn subscribe_events(&mut self) -> Result<EventSubscription, DeviceError> {
        let reader = self.reader.take().ok_or_else(|| {
            DeviceError::Protocol("Already subscribed to Kanata events".to_string())
        })?;

        let (event_tx, event_rx) = mpsc::channel();
        let initial_event = layer_change_event(&self.initial_layer, &self.layer_names)?;
        event_tx.send(initial_event).map_err(|e| {
            DeviceError::Transport(format!("Failed to queue initial Kanata layer: {e}"))
        })?;

        let reader_stop = self.stop_signal.clone();
        let layer_names = self.layer_names.clone();
        thread::spawn(move || {
            read_kanata_events(reader, event_tx, layer_names, reader_stop);
        });

        Ok(EventSubscription::new(event_rx, self.stop_signal.clone()))
    }

    fn disconnect(&mut self) {
        self.stop_signal.stop();
    }
}

impl Drop for KanataProtocol {
    fn drop(&mut self) {
        self.stop_signal.stop();
    }
}

fn layer_change_event(name: &str, layer_names: &[String]) -> Result<DeviceEvent, DeviceError> {
    let index = layer_names
        .iter()
        .position(|layer_name| layer_name == name)
        .ok_or_else(|| {
            DeviceError::Protocol(format!(
                "Kanata reports unknown layer '{name}'; reload the Kanata configuration in KeyPeek"
            ))
        })?;
    let layer_mask = 1u32.checked_shl(index as u32).ok_or_else(|| {
        DeviceError::Unsupported(format!("Kanata layer index {index} is not supported"))
    })?;
    Ok(DeviceEvent::LayersChanged {
        active_layers: if index == 0 { 0 } else { layer_mask },
        default_layers: 1,
    })
}

fn read_kanata_events(
    mut reader: BufReader<TcpStream>,
    event_tx: mpsc::Sender<DeviceEvent>,
    layer_names: Vec<String>,
    stop_signal: StopSignal,
) {
    if let Err(e) = reader
        .get_ref()
        .set_read_timeout(Some(Duration::from_millis(100)))
    {
        let _ = event_tx.send(DeviceEvent::Disconnected(format!(
            "Failed to configure Kanata TCP event reader: {e}"
        )));
        return;
    }

    let mut pending = Vec::new();
    while !stop_signal.is_stopped() {
        match reader.read_until(b'\n', &mut pending) {
            Ok(0) => {
                let _ = event_tx.send(DeviceEvent::Disconnected(
                    "Kanata TCP connection closed".to_string(),
                ));
                break;
            }
            Ok(_) if pending.last() == Some(&b'\n') => {
                let message = match serde_json::from_slice::<ServerMessage>(&pending) {
                    Ok(message) => message,
                    Err(e) => {
                        let _ = event_tx.send(DeviceEvent::Disconnected(format!(
                            "Invalid Kanata TCP event: {e}"
                        )));
                        break;
                    }
                };
                pending.clear();
                match message {
                    ServerMessage::LayerChange { new } => {
                        match layer_change_event(&new, &layer_names) {
                            Ok(event) => {
                                if event_tx.send(event).is_err() {
                                    break;
                                }
                            }
                            Err(e) => {
                                let _ = event_tx.send(DeviceEvent::Disconnected(e.to_string()));
                                break;
                            }
                        }
                    }
                    ServerMessage::ConfigFileReload { .. } => {
                        let _ = event_tx.send(DeviceEvent::Disconnected(
                            "Kanata configuration reloaded; reconnecting to refresh the overlay"
                                .to_string(),
                        ));
                        break;
                    }
                    ServerMessage::Error { msg } => {
                        let _ = event_tx.send(DeviceEvent::Disconnected(format!(
                            "Kanata TCP server error: {msg}"
                        )));
                        break;
                    }
                    _ => {}
                }
            }
            Ok(_) => {}
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                if pending.len() > MAX_TCP_MESSAGE_SIZE {
                    let _ = event_tx.send(DeviceEvent::Disconnected(
                        "Kanata TCP event exceeded the maximum message size".to_string(),
                    ));
                    break;
                }
            }
            Err(e) => {
                let _ = event_tx.send(DeviceEvent::Disconnected(format!(
                    "Kanata TCP event reader failed: {e}"
                )));
                break;
            }
        }
        if pending.len() > MAX_TCP_MESSAGE_SIZE {
            let _ = event_tx.send(DeviceEvent::Disconnected(
                "Kanata TCP event exceeded the maximum message size".to_string(),
            ));
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_kanata_layer_is_represented_without_fake_key_presses() {
        let layers = vec!["base".to_string(), "nav".to_string()];
        assert_eq!(
            layer_change_event("nav", &layers).unwrap(),
            DeviceEvent::LayersChanged {
                active_layers: 2,
                default_layers: 1,
            }
        );
        assert!(layer_change_event("missing", &layers).is_err());
    }

    #[test]
    fn tcp_layer_change_notifications_are_forwarded_as_device_events() {
        use std::io::Write;
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .write_all(
                    &ServerMessage::LayerChange {
                        new: "nav".to_string(),
                    }
                    .as_bytes(),
                )
                .unwrap();
        });

        let stream = TcpStream::connect(address).unwrap();
        let (event_tx, event_rx) = mpsc::channel();
        let reader = thread::spawn(move || {
            read_kanata_events(
                BufReader::new(stream),
                event_tx,
                vec!["base".to_string(), "nav".to_string()],
                StopSignal::new(),
            );
        });

        assert_eq!(
            event_rx.recv_timeout(Duration::from_secs(1)).unwrap(),
            DeviceEvent::LayersChanged {
                active_layers: 2,
                default_layers: 1,
            }
        );
        assert!(matches!(
            event_rx.recv_timeout(Duration::from_secs(1)).unwrap(),
            DeviceEvent::Disconnected(_)
        ));
        server.join().unwrap();
        reader.join().unwrap();
    }
}
