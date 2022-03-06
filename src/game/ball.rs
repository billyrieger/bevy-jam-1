use crate::*;

use super::{world::{WorldPosition, WorldSprite, SyncWorldPosition, Shadow}, player::Player};

pub(crate) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBallEvent>()
            .init_resource::<BallBouncesSinceHit>()
            .add_event::<HitEvent>()
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(ball_spawner)
                    .with_system(hit_ball_system),
            );
    }
}

#[derive(Default)]
pub struct BallBouncesSinceHit(pub u32);

// ====== Events ======

pub struct SpawnBallEvent {
    pub position: WorldPosition,
    pub velocity: RigidBodyVelocity,
}

pub struct HitEvent {
    pub ball_id: Entity,
    pub new_velocity: Vec3,
}

#[derive(Component)]
pub struct GameBall;

#[derive(Component)]
pub struct GameBallShadow;

#[derive(Component)]
pub struct LastHitBy(pub Player);

fn hit_ball_system(
    mut bounces: ResMut<BallBouncesSinceHit>,
    mut events: EventReader<HitEvent>,
    mut ball_query: Query<&mut RigidBodyVelocityComponent, With<GameBall>>,
) {
    for ev in events.iter() {
        bounces.0 = 0;
        info!("bounces reset");
        let mut ball_velocity = ball_query.get_mut(ev.ball_id).expect("ball not found");
        *ball_velocity = RigidBodyVelocity {
            linvel: ev.new_velocity.into(),
            ..default()
        }
        .into();
    }
}

fn ball_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut events: EventReader<SpawnBallEvent>,
) {
    let texture_handle = asset_server.get_handle("textures/ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for ev in events.iter() {
        let ball_id = commands
            .spawn()
            .insert(GameBall)
            .insert(LastHitBy(Player::User))
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..Default::default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, -0.7)),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: ev.position.0.into(),
                velocity: ev.velocity.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::ball(1.0).into(),
                material: ColliderMaterial {
                    friction: 0.8,
                    restitution: 0.6,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert_bundle((
                WorldPosition::default(),
                WorldSprite::default(),
                SyncWorldPosition,
            ))
            .id();
        commands
            .spawn_bundle((
                Shadow {
                    parent: ball_id,
                    scale: 1.0,
                },
                WorldPosition(Vec3::new(ev.position.0.x, ev.position.0.y, 0.)),
                SyncWorldPosition,
                GameBallShadow,
            ))
            .insert(WorldSprite {
                base: Vec2::new(-0., -4.),
                ..default()
            })
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 5,
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                ..default()
            });
    }
}
