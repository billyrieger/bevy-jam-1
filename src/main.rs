use bevy::prelude::*;

mod game;
mod setup;

const KEY_CODE_UP: KeyCode = KeyCode::Up;
const KEY_CODE_DOWN: KeyCode = KeyCode::Down;
const KEY_CODE_LEFT: KeyCode = KeyCode::Left;
const KEY_CODE_RIGHT: KeyCode = KeyCode::Right;
const KEY_CODE_ACTION: KeyCode = KeyCode::Space;
const KEY_CODE_SECONDARY: KeyCode = KeyCode::LShift;

const BG_WIDTH: f32 = 480.;
const BG_HEIGHT: f32 = 270.;

pub fn default<T: Default>() -> T {
    Default::default()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: BG_WIDTH,
            height: BG_HEIGHT,
            scale_factor_override: Some(3.),
            // resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugin(setup::SetupPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(AppState::Loading)
        .run();
}
