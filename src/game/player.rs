use crate::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(user_movement_system)
                .with_system(user_begin_charge_system)
                .with_system(user_release_charge_system)
                .with_system(opponent_movement_system)
                .with_system(opponent_hit_system),
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
    mut query: Query<(&Player, &PlayerState, &mut SpriteAnimation), Changed<PlayerState>>,
) {
    for (player, state, mut animation) in query.iter_mut() {
        *animation = match (player, state) {
            (Player::User, PlayerState::Idle) => SpriteAnimation::player_idle(),
            (Player::User, PlayerState::Run) => SpriteAnimation::player_run(),
            (Player::User, PlayerState::Charge) => SpriteAnimation::player_charge(),
            (Player::User, PlayerState::Swing) => SpriteAnimation::player_swing(),
            (Player::Opponent, PlayerState::Idle) => SpriteAnimation::opponent_idle(),
            (Player::Opponent, PlayerState::Run) => SpriteAnimation::opponent_run(),
            (Player::Opponent, PlayerState::Charge) => SpriteAnimation::opponent_charge(),
            (Player::Opponent, PlayerState::Swing) => SpriteAnimation::opponent_swing(),
        };
    }
}

fn user_movement_system(
    time: Res<Time>,
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
            position.0 += direction.normalize() * speed.0 * time.delta().as_secs_f32();
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

fn opponent_movement_system(
    time: Res<Time>,
    mut player_query: Query<
        (&mut PlayerState, &mut WorldPosition, &PlayerSpeed),
        (With<CpuControlled>, Without<GameBall>),
    >,
    ball_query: Query<&WorldPosition, With<GameBall>>,
) {
    for (mut opponent_state, mut opponent_pos, speed) in player_query.iter_mut() {
        if let Ok(ball_pos) = ball_query.get_single() {
            let delta_x = ball_pos.0.x - opponent_pos.0.x;
            let direction = if delta_x > 0. { Vec3::X } else { -Vec3::X };
            let max_distance = speed.0 * time.delta().as_secs_f32();
            if delta_x.abs() <= max_distance {
                opponent_pos.0.x = ball_pos.0.x;
                if matches!(*opponent_state, PlayerState::Run) {
                    *opponent_state = PlayerState::Idle;
                }
            } else {
                opponent_pos.0 += max_distance * direction;
                if matches!(*opponent_state, PlayerState::Idle) {
                    *opponent_state = PlayerState::Run;
                }
            }
        }
    }
}

fn opponent_hit_system(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &mut PlayerState, &WorldPosition),
        (With<CpuControlled>, Without<GameBall>),
    >,
    mut hit_events: EventWriter<HitEvent>,
    mut ball_query: Query<(Entity, &WorldPosition, &mut LastHitBy), With<GameBall>>,
) {
    for (opponent_id, mut opponent_state, opponent_pos) in player_query.iter_mut() {
        if matches!(*opponent_state, PlayerState::Charge | PlayerState::Swing) {
            continue;
        }
        if let Ok((ball_id, ball_pos, mut last_hit)) = ball_query.get_single_mut() {
            if (ball_pos.0.y - opponent_pos.0.y).abs() < 0.25 {
                let delta_x = ball_pos.0.x - opponent_pos.0.x;
                if delta_x.abs() < 2.0 {
                    *last_hit = LastHitBy(Player::Opponent);
                    let return_velocity = Vec3::new(rand::random::<f32>() * 10., -15., 10.);
                    hit_events.send(HitEvent {
                        new_velocity: return_velocity,
                        ball_id,
                    });
                    *opponent_state = PlayerState::Swing;
                    commands
                        .entity(opponent_id)
                        .insert(SwingCooldown(Timer::from_seconds(1.0, false)));
                }
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

fn user_begin_charge_system(
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

fn user_release_charge_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<
        (Entity, &mut PlayerState, &PlayerFacing, &WorldPosition),
        With<UserControlled>,
    >,
    mut ball_query: Query<(Entity, &WorldPosition, &mut LastHitBy), With<GameBall>>,
    mut hit_events: EventWriter<HitEvent>,
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
                for (ball_id, ball_pos, mut last_hit) in ball_query.iter_mut() {
                    let dist_to_ball = (sweet_spot - ball_pos.0).length();
                    let hit_direction_xy = (Vec3::new(X_CENTER_LINE, Y_FAR_BASELINE, 0.)
                        - player_position.0)
                        .normalize();
                    info!("{dist_to_ball:?}");
                    if dist_to_ball < 2.0 {
                        *last_hit = LastHitBy(Player::User);
                        hit_events.send(HitEvent {
                            ball_id,
                            new_velocity: hit_direction_xy * 20. + Vec3::Z * 8.,
                        });
                    }
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
        if matches!(*player_state, PlayerState::Idle | PlayerState::Run | PlayerState::Charge) {
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
    let player_texture_handle = asset_server.get_handle("textures/player.png");
    let player_texture_atlas =
        TextureAtlas::from_grid(player_texture_handle, Vec2::new(24.0, 24.0), 4, 8);
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);

    let opponent_texture_handle = asset_server.get_handle("textures/opponent.png");
    let opponent_texture_atlas =
        TextureAtlas::from_grid(opponent_texture_handle, Vec2::new(24.0, 24.0), 4, 8);
    let opponent_texture_atlas_handle = texture_atlases.add(opponent_texture_atlas);

    for ev in events.iter() {
        let speed = if ev.opponent {
            PLAYER_SPEED
        } else {
            PLAYER_SPEED
        };
        let texture_atlas_handle = if ev.opponent {
            opponent_texture_atlas_handle.clone()
        } else {
            player_texture_atlas_handle.clone()
        };
        let id = commands
            .spawn_bundle((
                if ev.opponent {
                    Player::Opponent
                } else {
                    Player::User
                },
                PlayerState::Idle,
                PlayerSpeed(speed),
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
        if ev.opponent {
            commands.entity(id).insert(Opponent).insert(CpuControlled);
        } else {
            commands.entity(id).insert(UserControlled);
        }
        commands
            .spawn_bundle((
                Shadow {
                    parent: id,
                    scale: 1.0,
                },
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
                texture_atlas: player_texture_atlas_handle.clone(),
                ..default()
            });
    }
}
