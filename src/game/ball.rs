use crate::*;

pub(crate) struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(ball_spawner));
    }
}

fn ball_spawner(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut events: EventReader<SpawnBallEvent>,
) {
    for ev in events.iter() {
        spawn_ball(&mut commands, &game_assets, &ev);
    }
}

fn spawn_ball(commands: &mut Commands, game_assets: &GameAssets, ev: &SpawnBallEvent) {
    let ball = commands
        .spawn()
        .insert(GameBall)
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: game_assets.ball_texture_atlas.clone(),
            transform: Transform::from_scale(Vec3::splat(PX_SCALE))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, -0.7)),
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
            Shadow { parent: ball },
            WorldPosition(Vec3::new(ev.position.0.x, ev.position.0.y, 0.)),
            SyncWorldPosition,
        ))
        .insert(WorldSprite {
            base: Vec2::new(-0., -8.),
            ..default()
        })
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 5,
                ..default()
            },
            texture_atlas: game_assets.ball_texture_atlas.clone(),
            ..default()
        });
}
