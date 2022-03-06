#![feature(array_windows, try_blocks)]

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

mod game;
mod setup;

const KEY_CODE_UP: KeyCode = KeyCode::Up;
const KEY_CODE_DOWN: KeyCode = KeyCode::Down;
const KEY_CODE_LEFT: KeyCode = KeyCode::Left;
const KEY_CODE_RIGHT: KeyCode = KeyCode::Right;
const KEY_CODE_ACTION: KeyCode = KeyCode::Space;

const PLAYER_SPEED: f32 = 15.;
const PLAYER_CHARGING_SPEED_FACTOR: f32 = 0.4;
const PLAYER_SWING_COOLDOWN_SECS: f32 = 0.5;

const BG_WIDTH: f32 = 272.;
const BG_HEIGHT: f32 = 256.;
const WORLD_SCALE: f32 = 5.;

fn default<T: Default>() -> T {
    Default::default()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    InGame,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: BG_WIDTH,
            height: BG_HEIGHT,
            scale_factor_override: Some(1.),
            // resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugin(setup::SetupPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(AppState::Loading)
        .run();
}
