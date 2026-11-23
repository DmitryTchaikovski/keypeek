use super::MAX_LAYERS;
use kanata_keyberon::action::Action;
use kanata_parser::custom_action::{Btn, CustomAction, MWheelDirection, MoveDirection};
use keypeek_core::{CustomBinding, CustomKind, KeySpec, LayerActivation, MouseAction, MouseButton};
use std::fmt::Debug;

pub(super) fn action_to_key_spec<'a>(action: Action<'a, &'a CustomAction>, id: usize) -> KeySpec {
    match action {
        Action::NoOp => KeySpec::None,
        Action::Trans => KeySpec::Transparent,
        Action::Layer(layer) if layer < MAX_LAYERS => KeySpec::Layer {
            layer: layer as u8,
            activation: LayerActivation::Momentary,
        },
        Action::DefaultLayer(layer) if layer < MAX_LAYERS => KeySpec::Layer {
            layer: layer as u8,
            activation: LayerActivation::Default,
        },
        Action::Custom(CustomAction::MoveMouse {
            direction,
            distance,
            ..
        }) => mouse_move_spec(*direction, *distance),
        Action::Custom(CustomAction::MoveMouseAccel {
            direction,
            max_distance,
            ..
        }) => mouse_move_spec(*direction, *max_distance),
        Action::Custom(CustomAction::MWheel {
            direction,
            distance,
            ..
        }) => mouse_scroll_spec(*direction, *distance),
        Action::Custom(CustomAction::MWheelNotch { direction }) => mouse_scroll_spec(*direction, 1),
        Action::Custom(CustomAction::Mouse(button) | CustomAction::MouseTap(button)) => {
            KeySpec::Mouse(MouseAction::Press(mouse_button(*button)))
        }
        _ => KeySpec::Custom(CustomBinding {
            kind: CustomKind::Raw,
            id: id as u32,
            name: Some(action_label(&action)),
            param1: None,
            param2: None,
        }),
    }
}

fn mouse_scroll_spec(direction: MWheelDirection, distance: u16) -> KeySpec {
    let distance = distance as i16;
    let (x, y) = match direction {
        MWheelDirection::Up => (0, distance),
        MWheelDirection::Down => (0, -distance),
        MWheelDirection::Left => (-distance, 0),
        MWheelDirection::Right => (distance, 0),
    };
    KeySpec::Mouse(MouseAction::Scroll { x, y })
}

fn mouse_button(button: Btn) -> MouseButton {
    match button {
        Btn::Left => MouseButton::Left,
        Btn::Right => MouseButton::Right,
        Btn::Mid => MouseButton::Middle,
        Btn::Forward => MouseButton::Button4,
        Btn::Backward => MouseButton::Button5,
    }
}

fn mouse_move_spec(direction: MoveDirection, distance: u16) -> KeySpec {
    let distance = distance as i16;
    let (x, y) = match direction {
        MoveDirection::Up => (0, -distance),
        MoveDirection::Down => (0, distance),
        MoveDirection::Left => (-distance, 0),
        MoveDirection::Right => (distance, 0),
    };
    KeySpec::Mouse(MouseAction::Move { x, y })
}

fn action_label<'a, T: Debug + 'a>(action: &Action<'a, T>) -> String {
    match action {
        Action::NoOp => String::new(),
        Action::Trans => "_".to_string(),
        Action::KeyCode(key) => keycode_label(key),
        Action::MultipleKeyCodes(keys) => {
            keys.iter().map(keycode_label).collect::<Vec<_>>().join("+")
        }
        Action::MultipleActions(_) => "Multi".to_string(),
        Action::Layer(layer) => format!("MO L{layer}"),
        Action::DefaultLayer(layer) => format!("DF L{layer}"),
        Action::Sequence { .. } | Action::RepeatableSequence { .. } => "Sequence".to_string(),
        Action::CancelSequences => "Cancel".to_string(),
        Action::ReleaseState(_) => "Release".to_string(),
        Action::HoldTap(_) => "Tap/Hold".to_string(),
        Action::Custom(_) => "Custom".to_string(),
        Action::OneShot(_) => "OneShot".to_string(),
        Action::OneShotIgnoreEventsTicks(_) => "OneShot".to_string(),
        Action::TapDance(_) => "TapDance".to_string(),
        Action::Chords(_) => "Chord".to_string(),
        Action::Repeat => "Repeat".to_string(),
        Action::Fork(_) | Action::Switch(_) => "Switch".to_string(),
        Action::Src => "Src".to_string(),
    }
}

fn keycode_label(key: &impl Debug) -> String {
    match format!("{key:?}").as_str() {
        "Kb1" => "1".to_string(),
        "Kb2" => "2".to_string(),
        "Kb3" => "3".to_string(),
        "Kb4" => "4".to_string(),
        "Kb5" => "5".to_string(),
        "Kb6" => "6".to_string(),
        "Kb7" => "7".to_string(),
        "Kb8" => "8".to_string(),
        "Kb9" => "9".to_string(),
        "Kb0" => "0".to_string(),
        "BSpace" => "Bksp".to_string(),
        "SColon" => ";".to_string(),
        "LBracket" => "[".to_string(),
        "RBracket" => "]".to_string(),
        "Bslash" => "\\".to_string(),
        "LCtrl" => "Ctrl".to_string(),
        "RCtrl" => "RCtrl".to_string(),
        "LShift" => "Shift".to_string(),
        "RShift" => "RShift".to_string(),
        "LAlt" => "Alt".to_string(),
        "RAlt" => "RAlt".to_string(),
        "LGui" => "Meta".to_string(),
        "RGui" => "RMeta".to_string(),
        name => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::config::parse_config;
    use super::*;
    use kanata_parser::keys::OsCode;

    #[test]
    fn mouse_movement_actions_are_recognized() {
        let source = r#"
            (defsrc a b c d e)
            (deflayer base
                (movemouse-up 1 3)
                (movemouse-down 1 4)
                (movemouse-left 1 5)
                (movemouse-right 1 6)
                (movemouse-accel-up 1 10 2 7))
        "#;
        let config =
            kanata_parser::cfg::new_from_str(source, std::iter::empty().collect()).unwrap();
        let (_, snapshot, _) = parse_config(config).unwrap();
        let key_specs = &snapshot.actions[0][0];

        assert_eq!(
            key_specs[usize::from(OsCode::KEY_A)],
            Some(KeySpec::Mouse(MouseAction::Move { x: 0, y: -3 }))
        );
        assert_eq!(
            key_specs[usize::from(OsCode::KEY_B)],
            Some(KeySpec::Mouse(MouseAction::Move { x: 0, y: 4 }))
        );
        assert_eq!(
            key_specs[usize::from(OsCode::KEY_C)],
            Some(KeySpec::Mouse(MouseAction::Move { x: -5, y: 0 }))
        );
        assert_eq!(
            key_specs[usize::from(OsCode::KEY_D)],
            Some(KeySpec::Mouse(MouseAction::Move { x: 6, y: 0 }))
        );
        assert_eq!(
            key_specs[usize::from(OsCode::KEY_E)],
            Some(KeySpec::Mouse(MouseAction::Move { x: 0, y: -7 }))
        );
    }

    #[test]
    fn mouse_button_actions_are_recognized() {
        let source = r#"
            (defsrc a b c d e f g h i j)
            (deflayer base
                mlft mrgt mmid mfwd mbck
                mltp mrtp mmtp mftp mbtp)
        "#;
        let config =
            kanata_parser::cfg::new_from_str(source, std::iter::empty().collect()).unwrap();
        let (_, snapshot, _) = parse_config(config).unwrap();
        let key_specs = &snapshot.actions[0][0];
        let expected = [
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::Button4,
            MouseButton::Button5,
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::Button4,
            MouseButton::Button5,
        ];

        for (key, button) in [
            OsCode::KEY_A,
            OsCode::KEY_B,
            OsCode::KEY_C,
            OsCode::KEY_D,
            OsCode::KEY_E,
            OsCode::KEY_F,
            OsCode::KEY_G,
            OsCode::KEY_H,
            OsCode::KEY_I,
            OsCode::KEY_J,
        ]
        .into_iter()
        .zip(expected)
        {
            assert_eq!(
                key_specs[usize::from(key)],
                Some(KeySpec::Mouse(MouseAction::Press(button)))
            );
        }
    }

    #[test]
    fn mouse_scroll_actions_are_recognized() {
        let source = r#"
            (defsrc a b c d e f g h i)
            (deflayer base
                (mwheel-up 1 3)
                (mwheel-down 1 4)
                (mwheel-left 1 5)
                (mwheel-right 1 6)
                mwu mwd mwl mwr
                (mwheel-accel-up 1 10 2 0.5))
        "#;
        let config =
            kanata_parser::cfg::new_from_str(source, std::iter::empty().collect()).unwrap();
        let (_, snapshot, _) = parse_config(config).unwrap();
        let key_specs = &snapshot.actions[0][0];
        let expected = [
            (OsCode::KEY_A, 0, 3),
            (OsCode::KEY_B, 0, -4),
            (OsCode::KEY_C, -5, 0),
            (OsCode::KEY_D, 6, 0),
            (OsCode::KEY_E, 0, 1),
            (OsCode::KEY_F, 0, -1),
            (OsCode::KEY_G, -1, 0),
            (OsCode::KEY_H, 1, 0),
            (OsCode::KEY_I, 0, 1),
        ];

        for (key, x, y) in expected {
            assert_eq!(
                key_specs[usize::from(key)],
                Some(KeySpec::Mouse(MouseAction::Scroll { x, y }))
            );
        }
    }
}
