use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum InputButton {
    Up,
    Down,
    Left,
    Right,
    Primary,
    Secondary,
}

struct KeyCodeMap(HashMap<InputButton, Vec<KeyCode>>);

impl KeyCodeMap {
}

impl Default for KeyCodeMap {
    fn default() -> Self {
        use InputButton::*;
        Self(HashMap::from_iter([
            (Up, vec![KeyCode::Up, KeyCode::W]),
            (Down, vec![KeyCode::Down, KeyCode::S]),
            (Left, vec![KeyCode::Left, KeyCode::A]),
            (Right, vec![KeyCode::Right, KeyCode::D]),
            (Primary, vec![KeyCode::Space]),
            (Secondary, vec![KeyCode::LShift, KeyCode::RShift]),
        ]))
    }
}
