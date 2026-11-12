use crate::device_discovery::{DeviceDriverScanner, DiscoveredDevice};
use crate::protocols::ConnectionSpec;

const DEFAULT_ADDRESS: &str = "127.0.0.1:5829";

pub struct KanataScanner;

impl DeviceDriverScanner for KanataScanner {
    fn scan(&self, _ctx: &mut crate::device_discovery::DiscoveryContext) -> Vec<DiscoveredDevice> {
        vec![DiscoveredDevice {
            base_name: "Kanata".to_string(),
            vid: 0,
            pid: 0,
            driver_id: "kanata",
            protocol_label: "TCP",
            requires_layout_file: true,
            spec: ConnectionSpec::Kanata {
                config_path: String::new(),
                address: DEFAULT_ADDRESS.to_string(),
            },
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_offers_a_config_backed_tcp_connection() {
        let devices = KanataScanner.scan(&mut crate::device_discovery::DiscoveryContext::default());
        assert_eq!(devices.len(), 1);
        assert!(devices[0].requires_layout_file);
        assert!(matches!(
            devices[0].spec,
            ConnectionSpec::Kanata { ref config_path, ref address }
                if config_path.is_empty() && address == DEFAULT_ADDRESS
        ));
    }
}
