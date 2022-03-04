#![feature(try_blocks)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_easings::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

mod game;
mod setup;
mod ui;

const Y_FAR_BASELINE: f32 = 10.5;
const Y_FAR_MIDLINE: f32 = 0.5;
const Y_NETLINE: f32 = -7.;
const Y_NEAR_MIDLINE: f32 = -11.75;
const Y_NEAR_BASELINE: f32 = -19.75;

const X_DOUBLES_LINE_LEFT: f32 = -18.75;
const X_SINGLES_LINE_LEFT: f32 = -15.25;
const X_CENTER_LINE: f32 = 0.;
const X_SINGLES_LINE_RIGHT: f32 = 15.25;
const X_DOUBLES_LINE_RIGHT: f32 = 18.75;

const NET_HEIGHT: f32 = 2.5;
const NET_THICKNESS: f32 = 0.05;

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
const PX_SCALE: f32 = 2.;
const WORLD_SCALE: f32 = 10.;

fn default<T: Default>() -> T {
    Default::default()
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
struct UserScore(u32);

#[derive(Default)]
struct OpponentScore(u32);

// ====== Events ======

struct SpawnPlayerEvent {
    position: WorldPosition,
    opponent: bool,
}

struct SpawnBallEvent {
    position: WorldPosition,
    velocity: RigidBodyVelocity,
}

struct SpawnCourtEvent;

struct HitEvent {
    ball_id: Entity,
    new_velocity: Vec3,
}

struct PointOverEvent {
    winner: Player,
}

struct GameOverEvent;

#[derive(Default)]
struct BallBouncesSinceHit(u32);

// ====== Components ======

#[derive(Component, Clone, Copy, Debug, Default)]
struct WorldPosition(Vec3);

#[derive(Component, Default)]
struct WorldSprite {
    base: Vec2,
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
    scale: f32,
}

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
struct CustomScale(f32);

// ====== Player components ======

#[derive(Component, Debug)]
enum Player {
    User,
    Opponent,
}

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
struct Opponent;

#[derive(Component)]
struct CpuControlled;

// ====== Ball components ======

#[derive(Component)]
struct GameBall;

#[derive(Component)]
struct GameBallShadow;

#[derive(Component)]
struct LastHitBy(Player);

#[derive(Component)]
struct UserScoreText;

#[derive(Component)]
struct OpponentScoreText;

#[derive(Component)]
struct ResultsText;

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
        .add_plugin(ui::UiPlugin)
        .add_plugin(game::GamePlugin)
        .add_state(AppState::Loading)
        .run();
}
