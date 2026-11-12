mod codec;
mod config;
mod discovery;
mod driver;
mod geometry;
mod presenter;
mod profile;

pub use discovery::KanataScanner;
pub use driver::KanataProtocol;
pub use presenter::KanataPresenter;
pub use profile::KanataEditorProfile;

use crate::device_discovery::DeviceDriverScanner;
use crate::firmware::FirmwareBundle;
use crate::protocols::{ConnectionSpec, DeviceError, KeyboardProtocol};
use keypeek_core::keymap_editor::EditorProfile;
use keypeek_core::KeyPresenter;
use std::sync::Arc;

const MAX_LAYERS: usize = 32;

pub struct KanataBundle;

impl FirmwareBundle for KanataBundle {
    fn create_presenter(&self) -> Arc<dyn KeyPresenter> {
        Arc::new(KanataPresenter)
    }

    fn create_profile(&self) -> Arc<dyn EditorProfile> {
        Arc::new(KanataEditorProfile)
    }

    fn scanners(&self) -> Vec<Box<dyn DeviceDriverScanner>> {
        vec![Box::new(KanataScanner)]
    }

    fn connect(&self, spec: &ConnectionSpec) -> Result<Box<dyn KeyboardProtocol>, DeviceError> {
        match spec {
            ConnectionSpec::Kanata {
                config_path,
                address,
            } => Ok(Box::new(KanataProtocol::connect(config_path, address)?)),
            _ => Err(DeviceError::Unsupported(
                "Unsupported spec for Kanata bundle".to_string(),
            )),
        }
    }
}
