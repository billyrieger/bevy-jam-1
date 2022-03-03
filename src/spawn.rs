use crate::*;

pub(crate) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTennisBall>()
            .add_event::<SpawnCourtEvent>()
            .add_event::<SpawnPlayerEvent>()
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_scene))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(spawn_players)
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
        position: Vec3::new(0.0, -2.0, 2.0).into(),
        velocity: RigidBodyVelocity {
            linvel: Vec3::new(0.0, 10.0, 0.0).into(),
            ..Default::default()
        },
    });
    court_events.send(SpawnCourtEvent);
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, 0.0, 0.0)),
        invert_controls: false,
    });
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, 0.0, 0.0)),
        invert_controls: true,
    });
}

fn ball_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnTennisBall>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for ev in events.iter() {
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
                transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: ev.position.into(),
                velocity: ev.velocity.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::ball(1.0).into(),
                material: ColliderMaterial {
                    friction: 0.8,
                    restitution: 0.8,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert(WorldPosition::default())
            .insert(SyncWorldPosition);
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
                texture: asset_server.load("court_grass.png"),
                transform: Transform::from_scale(Vec3::splat(crate::PX_SCALE)),
                ..Default::default()
            })
            .with_children(|parent| {
                // floor
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 200.0, 10.0).into(),
                    position: (Vec3::new(0.0, 0.0, -10.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.6,
                        restitution: 0.8,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
                // wall
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 1.0, 200.0).into(),
                    position: (Vec3::new(0.0, 21.0, 0.0), Quat::IDENTITY).into(),
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

fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut events: EventReader<SpawnPlayerEvent>,
) {
    let texture_handle = asset_server.get_handle("player_female_dark_blue.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 8, 5);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(crate::PX_SCALE)),
        ..Default::default()
    };

    for ev in events.iter() {
        let entity = commands
            .spawn_bundle((Player, UserControlled))
            .insert(ev.position)
            .insert(SyncWorldPosition)
            .insert_bundle(sprite_sheet_bundle.clone())
            .insert(SpriteAnimation::player_idle()).id();
        if ev.invert_controls {
            commands.entity(entity).insert(InvertControls);
        }

    }
}
