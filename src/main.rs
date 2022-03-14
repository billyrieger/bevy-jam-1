#![feature(array_windows, try_blocks)]

use bevy::prelude::*;

mod prelude {
    pub use crate::game::world::*;
    pub use crate::{default, AppState, WORLD_SCALE};
    pub use bevy::prelude::*;
    pub use bevy_prototype_lyon::prelude::*;
    pub use bevy_rapier3d::prelude::*;
}

mod game;
mod setup;

const KEY_CODE_UP: KeyCode = KeyCode::Up;
const KEY_CODE_DOWN: KeyCode = KeyCode::Down;
const KEY_CODE_LEFT: KeyCode = KeyCode::Left;
const KEY_CODE_RIGHT: KeyCode = KeyCode::Right;
const KEY_CODE_ACTION: KeyCode = KeyCode::Space;

const PLAYER_SPEED: f32 = 30.;
const PLAYER_CHARGING_SPEED_FACTOR: f32 = 0.4;
const PLAYER_SWING_COOLDOWN_SECS: f32 = 0.5;

const BG_WIDTH: f32 = 320. *1.5;
const BG_HEIGHT: f32 = 180. *1.5;
pub const WORLD_SCALE: f32 = 16.;

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
