use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use tap::Tap;

use crate::input::{MousePosition, PrimaryButtonPress};
use crate::world::{SyncCoords, WorldCoords};
use crate::GameState;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugTimer(Timer::new(Duration::from_secs_f32(10.0), true)))
            .insert_resource(SpawnBallTimer(Timer::new(
                Duration::from_secs_f32(1.0),
                true,
            )))
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(spawn_players)
                    .with_system(spawn_court)
                    .with_system(move_player_to_mouse),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(hit_ball)
                    .with_system(spawn_balls)
                    .with_system(debug_ball_pos)
                    .with_system(flip_player_sprite)
                    .with_system(remove_oob_ball)
                    .with_system(move_player_to_mouse),
            );
    }
}

#[derive(Component)]
struct Player {
    _facing: Facing,
}

enum Facing {
    North,
}

#[derive(Component)]
struct UserControlled;

#[derive(Component)]
struct CpuControlled;

#[derive(Component)]
struct Court;

#[derive(Component)]
struct TennisBallShadow;

#[derive(Component)]
struct TennisBall;

#[derive(Component)]
struct PlayerTargetLocation(Vec3);

fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.get_handle("player_female_dark_blue.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(crate::PX_SCALE)),
        ..Default::default()
    };

    commands
        .spawn()
        .insert(Player {
            _facing: Facing::North,
        })
        .insert(WorldCoords(Vec3::new(0.0, 0.0, 0.0)))
        .insert(SyncCoords)
        .insert(UserControlled)
        .insert_bundle(sprite_sheet_bundle.clone().tap_mut(|bundle| {
            bundle.transform =
                Transform::from_translation(Vec3::new(-200.0, -200.0, 1.0)) * bundle.transform;
            bundle.sprite.index = 16;
        }));
}

struct SpawnBallTimer(Timer);

fn spawn_balls(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnBallTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return;
    };
    let texture_handle = asset_server.get_handle("ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 9, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert(TennisBallShadow)
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 51,
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(crate::PX_SCALE)),
            ..Default::default()
        })
        .insert(WorldCoords(Vec3::new(0.0, 0.0, 0.0)))
        .insert(SyncCoords);

    commands
        .spawn()
        .insert(TennisBall)
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 43,
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(crate::PX_SCALE)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: (Vec3::new(0.0, 1.0, 0.0), Quat::IDENTITY).into(),
            velocity: RigidBodyVelocity {
                linvel: Vec3::new(
                    rand::random::<f32>() * 100.0 - 200.0,
                    0.0,
                    300.0 + rand::random::<f32>() * 200.0,
                )
                .into(),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(0.25).into(),
            material: ColliderMaterial {
                friction: 0.8,
                restitution: 0.8,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(WorldCoords(Vec3::new(0.0, 10.0, 0.0)))
        .insert(SyncCoords);
}

fn spawn_court(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("court_grass.png"),
            transform: Transform::from_scale(Vec3::splat(crate::PX_SCALE)),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(200.0, 1.0, 200.0).into(),
            position: (Vec3::new(0.0, -1.0, 0.0), Quat::IDENTITY).into(),
            material: ColliderMaterial {
                friction: 0.6,
                restitution: 0.8,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        });
}

fn remove_oob_ball(
    mut commands: Commands,
    ball_query: Query<(Entity, &WorldCoords), With<TennisBall>>,
) {
    for (entity, coords) in ball_query.iter() {
        if coords.0.x.abs() > 1000.0 || coords.0.y < -10.0 || coords.0.z.abs() > 1000.0 {
            println!("despawning ball");
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum HitQuality {
    Perfect,
    Amazing,
    Great,
    Good,
    Poor,
    Miss,
}

impl HitQuality {
    fn from_dist(dist: f32) -> Self {
        assert!(dist >= 0.0);
        match dist {
            x if x < 4.0 => Self::Perfect,
            x if x < 7.0 => Self::Amazing,
            x if x < 12.0 => Self::Great,
            x if x < 30.0 => Self::Good,
            x if x < 50.0 => Self::Poor,
            _ => Self::Miss,
        }
    }

    fn return_speed(&self) -> f32 {
        match self {
            HitQuality::Perfect => 1000.0,
            HitQuality::Amazing => 500.0,
            HitQuality::Great => 300.0,
            HitQuality::Good => 240.0,
            HitQuality::Poor => 100.0,
            HitQuality::Miss => 0.0,
        }
    }
}

enum PlayerState {
    Idle,
    Stroke,
}

fn hit_ball(
    mouse_position: Res<MousePosition>,
    mut events: EventReader<PrimaryButtonPress>,
    mut query: Query<
        (
            &Transform,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<TennisBall>,
    >,
) {
    for _ in events.iter() {
        for (ball_transform, ball_pos, mut ball_vel) in query.iter_mut() {
            let distance =
                (mouse_position.0.unwrap().extend(0.0) - ball_transform.translation).length();
            let quality = HitQuality::from_dist(distance);
            if matches!(quality, HitQuality::Miss) {
                continue;
            }
            println!("{quality:?}");
            let target_x = rand::random::<f32>() * 400.0 - 200.0;
            let world_target = Vec3::new(target_x, 0.0, 0.0);
            let new_velocity = (world_target - Vec3::from(ball_pos.position.translation))
                .normalize()
                * quality.return_speed();
            println!("{new_velocity}");
            ball_vel.linvel = new_velocity.into();
        }
    }
}

fn move_player_to_mouse(
    mouse_position: Res<MousePosition>,
    mut query: Query<&mut WorldCoords, (With<Player>, With<UserControlled>)>,
) {
    for mut player_coords in query.iter_mut() {
        match mouse_position.0 {
            Some(position) => {
                player_coords.0.x = (position.x - 20.0) / 1.0;
                player_coords.0.z = (-position.y + 18.0) / 1.0;
                player_coords.0.y = 0.0;
            }
            None => {}
        }
    }
}

struct DebugTimer(Timer);

fn debug_ball_pos(
    time: Res<Time>,
    mut debug_timer: ResMut<DebugTimer>,
    ball_query: Query<&WorldCoords, With<TennisBall>>,
) {
    if debug_timer.0.tick(time.delta()).just_finished() {
        for WorldCoords(coords) in ball_query.iter() {
            println!("{coords:?}");
        }
    }
}

fn flip_player_sprite(mut players: Query<(&mut TextureAtlasSprite, &Transform), With<Player>>) {
    for (mut sprite, transform) in players.iter_mut() {
        sprite.flip_x = transform.translation.x > 0.0;
    }
}