use super::codec::action_to_key_spec;
use super::geometry::{ansi_position, fallback_position};
use super::MAX_LAYERS;
use crate::protocols::DeviceError;
use kanata_parser::cfg::Cfg;
use kanata_parser::keys::OsCode;
use keypeek_core::{Key, KeyboardDefinition, KeyboardLayout, KeymapSnapshot, LayerInfo};

pub(super) fn parse_config(
    config: Cfg,
) -> Result<(KeyboardDefinition, KeymapSnapshot, Vec<String>), DeviceError> {
    let layer_names: Vec<String> = config
        .layer_info
        .iter()
        .map(|layer| layer.name.clone())
        .collect();
    if layer_names.is_empty() {
        return Err(DeviceError::Protocol(
            "Kanata configuration does not define any layers".to_string(),
        ));
    }
    if layer_names.len() > MAX_LAYERS {
        return Err(DeviceError::Unsupported(format!(
            "Kanata configuration defines {} layers; KeyPeek supports at most {MAX_LAYERS}",
            layer_names.len()
        )));
    }

    let process_unmapped_keys = config.options.process_unmapped_keys;
    let mut source_keys: Vec<OsCode> = config
        .mapped_keys
        .iter()
        .copied()
        .filter(|key| !process_unmapped_keys || ansi_position(*key).is_some())
        .collect();
    source_keys.sort_by_key(|key| usize::from(*key));
    if source_keys.is_empty() {
        return Err(DeviceError::Protocol(
            "Kanata configuration does not define any source keys".to_string(),
        ));
    }

    let cols = source_keys
        .iter()
        .map(|key| usize::from(*key))
        .max()
        .unwrap_or_default()
        + 1;
    let layout = config.layout.b();
    let layout_layers = layout.layers;
    if layout_layers.len() < layer_names.len() {
        return Err(DeviceError::Protocol(format!(
            "Kanata parser returned {} keymap layers for {} layer definitions",
            layout_layers.len(),
            layer_names.len()
        )));
    }

    let mut actions = vec![vec![vec![None; cols]; 1]; layer_names.len()];
    for (layer_index, layer_actions) in actions.iter_mut().enumerate() {
        let key_actions = &layout_layers[layer_index][0];
        for key in &source_keys {
            let col = usize::from(*key);
            if let Some(action) = key_actions.get(col) {
                layer_actions[0][col] = Some(action_to_key_spec(*action, col));
            }
        }
    }

    let mut fallback_index = 0;
    let keys = source_keys
        .iter()
        .map(|key| {
            let (x, y, w, h) = ansi_position(*key).unwrap_or_else(|| {
                let index = fallback_index;
                fallback_index += 1;
                fallback_position(index)
            });
            Key {
                row: 0,
                col: usize::from(*key),
                x,
                y,
                w,
                h,
                r: 0.0,
            }
        })
        .collect();
    let definition = KeyboardDefinition {
        vid: 0,
        pid: 0,
        rows: 1,
        cols,
        layouts: vec![KeyboardLayout {
            name: "ANSI".to_string(),
            keys,
        }],
    };
    let snapshot = KeymapSnapshot {
        layers: layer_names
            .iter()
            .enumerate()
            .map(|(index, name)| LayerInfo {
                id: index as u32,
                name: Some(name.clone()),
            })
            .collect(),
        actions,
    };

    Ok((definition, snapshot, layer_names))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test_config() -> Cfg {
        let source = r#"
            (defsrc a b c)
            (deflayer base a b c)
            (deflayer nav _ b c)
        "#;
        kanata_parser::cfg::new_from_str(source, std::iter::empty().collect()).unwrap()
    }

    #[test]
    fn config_parser_builds_geometry_and_all_layers() {
        let (definition, snapshot, names) = parse_config(parse_test_config()).unwrap();

        assert_eq!(names, ["base", "nav"]);
        assert_eq!(definition.layouts[0].name, "ANSI");
        assert_eq!(definition.rows, 1);
        assert_eq!(definition.layouts[0].keys.len(), 3);
        assert_eq!(snapshot.layers.len(), 2);
        assert_eq!(snapshot.actions.len(), 2);
        assert!(snapshot.actions[0][0][usize::from(OsCode::KEY_A)].is_some());
        assert!(snapshot.actions[1][0][usize::from(OsCode::KEY_B)].is_some());
    }
}
