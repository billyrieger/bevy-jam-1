#![feature(bool_to_option)]
#![feature(try_blocks)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_easings::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

const BG_WIDTH: f32 = 272.0;
const BG_HEIGHT: f32 = 256.0;
const PX_SCALE: f32 = 2.0;

mod animation;
mod game;
mod input;
mod spawn;
mod resource;
mod world;

const WORLD_SCALE: f32 = 10.0;

// ====== State ======

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    InGame,
}

// ====== Resources ======

#[derive(Default)]
struct ResourceHandles(Vec<HandleUntyped>);

#[derive(Default)]
struct MousePosition(Option<Vec2>);

// ====== Events ======

struct PrimaryButtonPress;

struct MovePlayerEvent {
    direction: Vec3,
}

#[derive(Default)]
struct SpawnTennisBall {
    position: RigidBodyPosition,
    velocity: RigidBodyVelocity,
}

struct SpawnCourtEvent;

struct SpawnPlayerEvent {
    position: WorldPosition,
    invert_controls: bool,
}

// ====== Components ======

#[derive(Component, Clone, Copy, Debug, Default)]
struct WorldPosition(Vec3);

#[derive(Component)]
struct SyncWorldPosition;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct UiCamera;

// Player components

#[derive(Component)]
struct Player;

#[derive(Component)]
struct UserControlled;

#[derive(Component)]
struct CpuControlled;

// Court components

#[derive(Component)]
struct Court;

#[derive(Component)]
struct GrassSurface;

// Tennis ball components

#[derive(Component)]
struct TennisBall;

#[derive(Component)]
struct TennisBallShadow;

// Animation components

#[derive(Component)]
struct SpriteAnimation {
    frames: Vec<SpriteAnimationFrame>,
    timer: Timer,
}

struct SpriteAnimationFrame {
    sprite_index: usize,
    duration: Duration,
}

#[derive(Component)]
struct InvertControls;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: BG_WIDTH * PX_SCALE,
            height: BG_HEIGHT * PX_SCALE,
            resizable: false,
            cursor_visible: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(EasingsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(resource::ResourcePlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(spawn::SpawnPlugin)
        .add_plugin(animation::AnimationPlugin)
        .add_state(GameState::Loading)
        .run();
}
