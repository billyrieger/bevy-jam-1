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
mod resource;
mod setup;
mod spawn;
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

struct PrimaryKeyPress;

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
struct WorldSprite {
    base: Vec2,
    custom_scale: f32,
}

impl Default for WorldSprite {
    fn default() -> Self {
        Self {
            base: Vec2::ZERO,
            custom_scale: 1.0,
        }
    }
}

#[derive(Component)]
struct SyncWorldPosition;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct UiCamera;

#[derive(Component)]
struct Shadow {
    parent: Entity,
}

// Player components

#[derive(Component, Clone)]
enum PlayerState {
    Idle,
    Run,
    Swing,
}

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

// ====== Components ======

#[derive(Component)]
struct SpriteAnimation {
    frames: Vec<SpriteAnimationFrame>,
    timer: Timer,
}

struct SpriteAnimationFrame {
    sprite_index: usize,
    duration: Duration,
}

#[derive(Component, Clone)]
struct NextPlayerState(PlayerState);

impl SpriteAnimation {
    fn new<const N: usize>(indices: [usize; N], durations: [f32; N], repeating: bool) -> Self {
        let frames = indices
            .into_iter()
            .zip(durations)
            .map(|(index, duration)| SpriteAnimationFrame {
                sprite_index: index,
                duration: Duration::from_secs_f32(duration),
            })
            .collect();
        Self {
            frames,
            timer: Timer::from_seconds(durations.iter().sum(), repeating),
        }
    }

    fn player_serve() -> Self {
        Self::new([0, 1, 2, 3], [1.0, 0.3, 0.2, 0.2], false)
    }

    fn player_idle() -> Self {
        Self::new([4, 5, 6, 7], [0.3, 0.1, 0.2, 0.1], true)
    }

    fn player_run() -> Self {
        // The spritesheet frames are off by one for this animation.
        Self::new([9, 10, 11, 8], [0.2, 0.2, 0.2, 0.2], true)
    }

    fn player_swing() -> Self {
        Self::new([12, 13, 14], [0.3, 0.3, 0.3], false)
    }
}

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
