#![feature(try_blocks)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_easings::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

mod game;
mod setup;

const KEY_CODE_UP: KeyCode = KeyCode::Up;
const KEY_CODE_DOWN: KeyCode = KeyCode::Down;
const KEY_CODE_LEFT: KeyCode = KeyCode::Left;
const KEY_CODE_RIGHT: KeyCode = KeyCode::Right;
const KEY_CODE_ACTION: KeyCode = KeyCode::Space;

const PLAYER_SPEED: f32 = 0.25;
const PLAYER_CHARGING_SPEED_FACTOR: f32 = 0.4;
const PLAYER_SWING_COOLDOWN_SECS: f32 = 0.5;

const BG_WIDTH: f32 = 272.;
const BG_HEIGHT: f32 = 256.;
const PX_SCALE: f32 = 2.;
const WORLD_SCALE: f32 = 10.;

fn default<T: Default>() -> T {
    Default::default()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum SystemOrder {
    First,
    Input,
    Main,
    Last,
}

// ====== State ======

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum AppState {
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
    position: WorldPosition,
    velocity: RigidBodyVelocity,
}

struct SpawnCourtEvent;

struct SpawnPlayerEvent {
    position: WorldPosition,
}

// ====== Components ======

#[derive(Component)]
struct DebugDot(Color);

#[derive(Component, Clone, Copy, Debug, Default)]
struct WorldPosition(Vec3);

#[derive(Component)]
struct WorldSprite {
    base: Vec2,
}

impl Default for WorldSprite {
    fn default() -> Self {
        Self { base: Vec2::ZERO }
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

// ====== Player components ======

#[derive(Component, Clone)]
enum PlayerState {
    Idle,
    Run,
    Charge,
    Swing,
}

#[derive(Component)]
struct PlayerSpeed(f32);

#[derive(Component)]
struct PlayerDirection(Vec3);

#[derive(Component)]
enum PlayerFacing {
    Right,
    Left,
}

#[derive(Component)]
struct SwingCooldown(Timer);

#[derive(Component)]
struct UserControlled;

#[derive(Component)]
struct CpuControlled;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: BG_WIDTH * PX_SCALE,
            height: BG_HEIGHT * PX_SCALE,
            resizable: false,
            vsync: true,
            ..Default::default()
        })
        .add_plugin(setup::SetupPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(AppState::Loading)
        .run();
}


fn spawn_debug_dot(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    pos: WorldPosition,
) {
    let texture_handle = asset_server.get_handle("textures/ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let dot_id = commands
        .spawn_bundle((
            pos,
            SyncWorldPosition,
            DebugDot(Color::RED),
            UserControlled,
        ))
        .insert(WorldSprite::default())
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 6,
                ..default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        })
        .id();
    commands
        .spawn_bundle((
            Shadow { parent: dot_id },
            WorldPosition::default(),
            SyncWorldPosition,
        ))
        .insert(WorldSprite::default())
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 5,
                ..default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        });
}