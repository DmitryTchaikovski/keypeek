use kanata_parser::keys::OsCode;

pub(super) fn fallback_position(index: usize) -> (f32, f32, f32, f32) {
    ((index % 12) as f32, 9.0 + (index / 12) as f32, 1.0, 1.0)
}

pub(super) fn ansi_position(key: OsCode) -> Option<(f32, f32, f32, f32)> {
    use OsCode::*;

    let unit = |x, y| Some((x, y, 1.0, 1.0));
    match key {
        KEY_ESC => unit(0.0, 0.0),
        KEY_F1 => unit(2.0, 0.0),
        KEY_F2 => unit(3.0, 0.0),
        KEY_F3 => unit(4.0, 0.0),
        KEY_F4 => unit(5.0, 0.0),
        KEY_F5 => unit(7.0, 0.0),
        KEY_F6 => unit(8.0, 0.0),
        KEY_F7 => unit(9.0, 0.0),
        KEY_F8 => unit(10.0, 0.0),
        KEY_F9 => unit(12.0, 0.0),
        KEY_F10 => unit(13.0, 0.0),
        KEY_F11 => unit(14.0, 0.0),
        KEY_F12 => unit(15.0, 0.0),
        KEY_1 => unit(1.0, 2.0),
        KEY_2 => unit(2.0, 2.0),
        KEY_3 => unit(3.0, 2.0),
        KEY_4 => unit(4.0, 2.0),
        KEY_5 => unit(5.0, 2.0),
        KEY_6 => unit(6.0, 2.0),
        KEY_7 => unit(7.0, 2.0),
        KEY_8 => unit(8.0, 2.0),
        KEY_9 => unit(9.0, 2.0),
        KEY_0 => unit(10.0, 2.0),
        KEY_MINUS => unit(11.0, 2.0),
        KEY_EQUAL => unit(12.0, 2.0),
        KEY_BACKSPACE => Some((13.0, 2.0, 2.0, 1.0)),
        KEY_TAB => Some((0.0, 3.0, 1.5, 1.0)),
        KEY_Q => unit(1.5, 3.0),
        KEY_W => unit(2.5, 3.0),
        KEY_E => unit(3.5, 3.0),
        KEY_R => unit(4.5, 3.0),
        KEY_T => unit(5.5, 3.0),
        KEY_Y => unit(6.5, 3.0),
        KEY_U => unit(7.5, 3.0),
        KEY_I => unit(8.5, 3.0),
        KEY_O => unit(9.5, 3.0),
        KEY_P => unit(10.5, 3.0),
        KEY_LEFTBRACE => unit(11.5, 3.0),
        KEY_RIGHTBRACE => unit(12.5, 3.0),
        KEY_ENTER => Some((13.5, 3.0, 1.5, 2.0)),
        KEY_CAPSLOCK => Some((0.0, 4.0, 1.75, 1.0)),
        KEY_A => unit(1.75, 4.0),
        KEY_S => unit(2.75, 4.0),
        KEY_D => unit(3.75, 4.0),
        KEY_F => unit(4.75, 4.0),
        KEY_G => unit(5.75, 4.0),
        KEY_H => unit(6.75, 4.0),
        KEY_J => unit(7.75, 4.0),
        KEY_K => unit(8.75, 4.0),
        KEY_L => unit(9.75, 4.0),
        KEY_SEMICOLON => unit(10.75, 4.0),
        KEY_APOSTROPHE => unit(11.75, 4.0),
        KEY_LEFTSHIFT => Some((0.0, 5.0, 2.25, 1.0)),
        KEY_Z => unit(2.25, 5.0),
        KEY_X => unit(3.25, 5.0),
        KEY_C => unit(4.25, 5.0),
        KEY_V => unit(5.25, 5.0),
        KEY_B => unit(6.25, 5.0),
        KEY_N => unit(7.25, 5.0),
        KEY_M => unit(8.25, 5.0),
        KEY_COMMA => unit(9.25, 5.0),
        KEY_DOT => unit(10.25, 5.0),
        KEY_SLASH => unit(11.25, 5.0),
        KEY_RIGHTSHIFT => Some((12.25, 5.0, 2.75, 1.0)),
        KEY_LEFTCTRL => Some((0.0, 6.0, 1.25, 1.0)),
        KEY_LEFTMETA => Some((1.25, 6.0, 1.25, 1.0)),
        KEY_LEFTALT => Some((2.5, 6.0, 1.25, 1.0)),
        KEY_SPACE => Some((3.75, 6.0, 6.25, 1.0)),
        KEY_RIGHTALT => Some((10.0, 6.0, 1.25, 1.0)),
        KEY_RIGHTMETA => Some((11.25, 6.0, 1.25, 1.0)),
        KEY_COMPOSE => Some((12.5, 6.0, 1.25, 1.0)),
        KEY_RIGHTCTRL => Some((13.75, 6.0, 1.25, 1.0)),
        KEY_INSERT => unit(16.0, 2.0),
        KEY_HOME => unit(17.0, 2.0),
        KEY_PAGEUP => unit(18.0, 2.0),
        KEY_DELETE => unit(16.0, 3.0),
        KEY_END => unit(17.0, 3.0),
        KEY_PAGEDOWN => unit(18.0, 3.0),
        KEY_UP => unit(17.0, 5.0),
        KEY_LEFT => unit(16.0, 6.0),
        KEY_DOWN => unit(17.0, 6.0),
        KEY_RIGHT => unit(18.0, 6.0),
        KEY_NUMLOCK => unit(20.0, 2.0),
        KEY_KP7 => unit(21.0, 2.0),
        KEY_KP8 => unit(22.0, 2.0),
        KEY_KP9 => unit(23.0, 2.0),
        KEY_KPMINUS => unit(24.0, 2.0),
        KEY_KP4 => unit(21.0, 3.0),
        KEY_KP5 => unit(22.0, 3.0),
        KEY_KP6 => unit(23.0, 3.0),
        KEY_KPPLUS => Some((24.0, 3.0, 1.0, 2.0)),
        KEY_KP1 => unit(21.0, 4.0),
        KEY_KP2 => unit(22.0, 4.0),
        KEY_KP3 => unit(23.0, 4.0),
        KEY_KPENTER => Some((24.0, 5.0, 1.0, 2.0)),
        KEY_KP0 => Some((21.0, 5.0, 2.0, 1.0)),
        KEY_KPDOT => unit(23.0, 5.0),
        KEY_KPASTERISK => unit(25.0, 2.0),
        KEY_KPSLASH => unit(26.0, 2.0),
        KEY_KPEQUAL => unit(25.0, 3.0),
        key if (KEY_F13 as u16..=KEY_F24 as u16).contains(&(key as u16)) => {
            let offset = usize::from(key) - usize::from(KEY_F13);
            unit((offset % 12) as f32, 8.0 + (offset / 12) as f32)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_preserves_ansi_and_fallback_positions() {
        assert_eq!(ansi_position(OsCode::KEY_A), Some((1.75, 4.0, 1.0, 1.0)));
        assert_eq!(fallback_position(0), (0.0, 9.0, 1.0, 1.0));
        assert_eq!(fallback_position(11), (11.0, 9.0, 1.0, 1.0));
        assert_eq!(fallback_position(12), (0.0, 10.0, 1.0, 1.0));
    }
}
