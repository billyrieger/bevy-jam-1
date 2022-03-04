#![feature(try_blocks)]

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_easings::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;

mod game;
mod setup;

const Y_FAR_BASELINE: f32 = 10.5;
const Y_FAR_MIDLINE: f32 = 0.5;
const Y_NETLINE: f32 = -7.;
const Y_NEAR_MIDLINE: f32 = -11.75;
const Y_NEAR_BASELINE: f32 = -19.75;

const X_CENTER: f32 = 0.;
const X_SINGLES: f32 = 15.25;
const X_DOUBLES: f32 = 18.75;

const NET_HEIGHT: f32 = 2.5;
const NET_THICKNESS: f32 = 0.05;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemOrder {
    Input,
}

// ====== State ======

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum AppState {
    Loading,
    StartScreen,
    InGame,
}

// ====== Resources ======

#[derive(Default)]
struct ResourceHandles(Vec<HandleUntyped>);

#[derive(Default)]
struct GameAssets {
    ball_texture_atlas: Handle<TextureAtlas>,
    court_texture_atlas: Handle<TextureAtlas>,
    player_texture: Handle<TextureAtlas>,
}

// ====== Events ======

#[derive(Default)]
struct SpawnBallEvent {
    position: WorldPosition,
    velocity: RigidBodyVelocity,
}

struct SpawnCourtEvent;

struct SpawnPlayerEvent {
    position: WorldPosition,
    opponent: bool,
}

struct BallBounceEvent;

struct BallOutOfBoundsEvent;

// ====== Components ======

#[derive(Component)]
struct DebugDot(Color);

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
}

// Court components

#[derive(Component)]
struct Court;

// Tennis ball components

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
        // .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_debug_dot))
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(move_debug_dot).with_system(display_events
        ))
        .run();
}

fn spawn_debug_dot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.get_handle("textures/ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 7);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let dot_id = commands
        .spawn_bundle((
            WorldPosition::default(),
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

fn move_debug_dot(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut WorldPosition, With<DebugDot>>,
) {
    for mut position in query.iter_mut() {
        if keyboard.just_pressed(KeyCode::A) {
            position.0 -= 0.25 * Vec3::X;
            info!("{position:?}");
        }
        if keyboard.just_pressed(KeyCode::D) {
            position.0 += 0.25 * Vec3::X;
            info!("{position:?}");
        }
        if keyboard.just_pressed(KeyCode::S) {
            position.0 -= 0.25 * Vec3::Y;
            info!("{position:?}");
        }
        if keyboard.just_pressed(KeyCode::W) {
            position.0 += 0.25 * Vec3::Y;
            info!("{position:?}");
        }
        if keyboard.just_pressed(KeyCode::Q) {
            position.0 -= 0.25 * Vec3::Z;
            info!("{position:?}");
        }
        if keyboard.just_pressed(KeyCode::E) {
            position.0 += 0.25 * Vec3::Z;
            info!("{position:?}");
        }
    }
}


fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        // println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        // println!("Received contact event: {:?}", contact_event);
    }
}

