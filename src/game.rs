use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use tap::Tap;

use crate::input::PrimaryButtonPress;
use crate::world::{SyncCoords, WorldCoords};
use crate::GameState;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(spawn_players)
                .with_system(spawn_court)
                .with_system(spawn_ball),
        )
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(hit_ball));
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
struct TennisBall;

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
        .insert(SyncCoords)
        .insert(UserControlled)
        .insert_bundle(sprite_sheet_bundle.clone().tap_mut(|bundle| {
            bundle.transform =
                Transform::from_translation(Vec3::new(-200.0, -200.0, 1.0)) * bundle.transform;
        }));
}

fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.get_handle("ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 9, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                linvel: Vec3::new(-5.0, 15.0, 5.0).into(),
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
            shape: ColliderShape::cuboid(200.0, 0.1, 200.0).into(),
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

fn hit_ball(
    mut inputs: EventReader<PrimaryButtonPress>,
    mut query: Query<&mut RigidBodyVelocityComponent, With<TennisBall>>,
) {
    for _ in inputs.iter() {
        for mut velocity in query.iter_mut() {
            velocity.linvel = Vec3::new(-1.0, 3.0, -20.0).into();
        }
    }
}
