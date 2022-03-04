use crate::*;

pub(crate) struct CourtPlugin;

impl Plugin for CourtPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(court_spawner));
    }
}

fn court_spawner(
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnCourtEvent>,
) {
    for _ in events.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("textures/net.png"),
                transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
                ..Default::default()
            })
            .insert_bundle((
                WorldPosition(Vec3::new(0.0, Y_NETLINE, 0.0)),
                SyncWorldPosition,
                WorldSprite {
                    base: Vec2::new(0.0, -22.0),
                },
            ));
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
                    flags: ActiveEvents::CONTACT_EVENTS.into(),
                    position: (Vec3::new(0.0, 0.0, -10.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.9,
                        restitution: 0.5,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
                // wall
                parent.spawn_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(200.0, 1.0, 200.0).into(),
                    flags: ActiveEvents::CONTACT_EVENTS.into(),
                    position: (Vec3::new(0.0, 15.0, 0.0), Quat::IDENTITY).into(),
                    material: ColliderMaterial {
                        friction: 0.6,
                        restitution: 0.8,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                });
                // net
                parent.spawn_bundle(ColliderBundle {
                    position: (
                        Vec3::new(X_CENTER, Y_NETLINE, NET_HEIGHT / 2.0),
                        Quat::IDENTITY,
                    )
                        .into(),
                    shape: ColliderShape::cuboid(X_DOUBLES, NET_THICKNESS / 2.0, NET_HEIGHT / 2.0)
                        .into(),
                    flags: ActiveEvents::CONTACT_EVENTS.into(),
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
