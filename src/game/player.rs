use crate::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .label(SystemOrder::Input)
                .with_system(player_movement_system)
                .with_system(begin_charge_system)
                .with_system(release_charge_system),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_spawn_system)
                .with_system(tick_swing_cooldown_system)
                .with_system(flip_sprite_facing_system)
                .with_system(turn_player_toward_ball)
                .with_system(set_player_speed_system)
                .with_system(update_animation_system),
        );
    }
}

fn update_animation_system(
    mut query: Query<(&PlayerState, &mut SpriteAnimation, Option<&Opponent>), Changed<PlayerState>>,
) {
    for (state, mut animation, opponent) in query.iter_mut() {
        *animation = if opponent.is_none() {
            match state {
                PlayerState::Idle => SpriteAnimation::player_idle(),
                PlayerState::Run => SpriteAnimation::player_run(),
                PlayerState::Charge => SpriteAnimation::player_charge(),
                PlayerState::Swing => SpriteAnimation::player_swing(),
            }
        } else {
            match state {
                PlayerState::Idle => SpriteAnimation::opponent_idle(),
                PlayerState::Run => SpriteAnimation::opponent_run(),
                PlayerState::Charge => SpriteAnimation::opponent_charge(),
                PlayerState::Swing => SpriteAnimation::opponent_swing(),
            }
        }
    }
}

fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerState, &PlayerSpeed, &mut WorldPosition), With<UserControlled>>,
) {
    for (mut state, speed, mut position) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KEY_CODE_RIGHT) {
            direction += Vec3::X;
        }
        if keyboard.pressed(KEY_CODE_LEFT) {
            direction -= Vec3::X;
        }
        if keyboard.pressed(KEY_CODE_UP) {
            direction += Vec3::Y;
        }
        if keyboard.pressed(KEY_CODE_DOWN) {
            direction -= Vec3::Y;
        }
        if direction.length() > 0. {
            position.0 += direction.normalize() * speed.0;
            // don't change state while charging or if it's already set
            if matches!(*state, PlayerState::Idle) {
                *state = PlayerState::Run;
            }
        } else {
            // don't change state while charging or if it's already set
            if matches!(*state, PlayerState::Run) {
                *state = PlayerState::Idle;
            }
        }
    }
}

fn set_player_speed_system(
    mut query: Query<(&mut PlayerSpeed, &PlayerState), With<UserControlled>>,
) {
    for (mut player_speed, player_state) in query.iter_mut() {
        player_speed.0 = match player_state {
            PlayerState::Idle | PlayerState::Run => PLAYER_SPEED,
            PlayerState::Charge => PLAYER_SPEED * PLAYER_CHARGING_SPEED_FACTOR,
            PlayerState::Swing => 0.,
        };
    }
}

fn begin_charge_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut PlayerState, With<UserControlled>>,
) {
    if keyboard.just_pressed(KEY_CODE_ACTION) {
        for mut state in query.iter_mut() {
            if matches!(*state, PlayerState::Idle | PlayerState::Run) {
                *state = PlayerState::Charge;
            }
        }
    }
}

fn release_charge_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<
        (Entity, &mut PlayerState, &PlayerFacing, &WorldPosition),
        With<UserControlled>,
    >,
    mut ball_query: Query<
        (
            &WorldPosition,
            &mut RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<GameBall>,
    >,
) {
    if keyboard.just_released(KEY_CODE_ACTION) {
        for (entity, mut player_state, player_facing, player_position) in player_query.iter_mut() {
            if matches!(*player_state, PlayerState::Charge) {
                *player_state = PlayerState::Swing;
                commands
                    .entity(entity)
                    .insert(SwingCooldown(Timer::from_seconds(
                        PLAYER_SWING_COOLDOWN_SECS,
                        false,
                    )));
                let flip = if matches!(player_facing, PlayerFacing::Left) {
                    -1.0
                } else {
                    1.0
                };
                let sweet_spot =
                    player_position.0 + Vec3::new(9.0 * flip, 0.0, 11.0) * PX_SCALE / WORLD_SCALE;
                for (ball_pos, phys_pos, mut phys_vel) in ball_query.iter_mut() {
                    let dist_to_ball = (sweet_spot - ball_pos.0).length();
                    phys_vel.linvel = Vec3::new(0.0, 20.0, 10.0).into();
                    phys_vel.angvel = Vec3::new(10.0, 0.0, 0.0).into();
                    info!("{dist_to_ball:?}");
                }
            }
        }
    }
}

fn tick_swing_cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PlayerState, &mut SwingCooldown)>,
) {
    for (player_id, mut player_state, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            *player_state = PlayerState::Idle;
            commands.entity(player_id).remove::<SwingCooldown>();
        }
    }
}

fn flip_sprite_facing_system(mut query: Query<(&PlayerFacing, &mut TextureAtlasSprite)>) {
    for (facing, mut sprite) in query.iter_mut() {
        sprite.flip_x = match facing {
            PlayerFacing::Right => false,
            PlayerFacing::Left => true,
        }
    }
}

fn turn_player_toward_ball(
    mut player_query: Query<(&mut PlayerFacing, &PlayerState, &WorldPosition)>,
    ball_query: Query<&WorldPosition, With<GameBall>>,
) {
    for (mut player_facing, player_state, player_pos) in player_query.iter_mut() {
        if matches!(*player_state, PlayerState::Idle | PlayerState::Run) {
            if let Ok(ball_pos) = ball_query.get_single() {
                *player_facing = if ball_pos.0.x - player_pos.0.x > 0. {
                    PlayerFacing::Right
                } else {
                    PlayerFacing::Left
                };
            }
        }
    }
}

fn player_spawn_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut events: EventReader<SpawnPlayerEvent>,
) {
    let texture_handle = asset_server.get_handle("textures/player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 4, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for ev in events.iter() {
        let id = commands
            .spawn_bundle((
                PlayerState::Idle,
                PlayerSpeed(PLAYER_SPEED),
                PlayerFacing::Right,
            ))
            .insert_bundle((
                ev.position,
                SyncWorldPosition,
                WorldSprite {
                    base: Vec2::new(0.0, -10.5) * PX_SCALE,
                    ..Default::default()
                },
            ))
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_scale(Vec3::splat(PX_SCALE)),
                ..Default::default()
            })
            .insert(SpriteAnimation::player_idle())
            .id();
        commands
            .spawn_bundle((
                Shadow { parent: id },
                WorldPosition::default(),
                WorldSprite {
                    base: Vec2::new(0.0, -10.5) * PX_SCALE,
                },
                SyncWorldPosition,
            ))
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 15,
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                ..default()
            });
    }
}
