use crate::*;

pub(crate) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTennisBall>()
            .add_event::<SpawnCourtEvent>()
            .add_event::<SpawnPlayerEvent>()
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scene))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(ball_spawner)
                    .with_system(court_spawner),
            );
    }
}

fn setup_scene(
    mut ball_events: EventWriter<SpawnTennisBall>,
    mut court_events: EventWriter<SpawnCourtEvent>,
    mut player_events: EventWriter<SpawnPlayerEvent>,
) {
    ball_events.send(SpawnTennisBall {
        position: WorldPosition(Vec3::new(1.0, 14.0, 3.0)),
        velocity: RigidBodyVelocity {
            linvel: Vec3::new(5.0, -20.0, 1.0).into(),
            ..Default::default()
        },
    });
    court_events.send(SpawnCourtEvent);
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, -22.0, 0.0)),
    });
}

fn ball_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnTennisBall>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
        spawn_ball(&mut commands, &asset_server, &mut texture_atlases, &ev);
    }
}

fn court_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnCourtEvent>,
) {
    for _ in events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("textures/court_grass.png"),
                transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
                ..Default::default()
            })
            .with_children(|parent| {
                // floor
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 200.0, 10.0).into(),
                    position: (Vec3::new(0.0, 0.0, -10.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.6,
                        restitution: 0.6,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
                // wall
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 1.0, 200.0).into(),
                    position: (Vec3::new(0.0, 15.0, 0.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.6,
                        restitution: 0.8,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
            });
    }
}

fn spawn_ball(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
    ev: &SpawnTennisBall,
) {
    let texture_handle = asset_server.get_handle("textures/ball.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 1, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let ball = commands
        .spawn()
        .insert(TennisBall)
        .insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 4,
                ..Default::default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
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
            base: Vec2::new(-1., -5.),
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
