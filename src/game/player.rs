use crate::game::world::*;
use crate::setup::TextureAtlasHandles;
use crate::{
    default, AppState, KEY_CODE_DOWN, KEY_CODE_LEFT, KEY_CODE_RIGHT, KEY_CODE_SECONDARY,
    KEY_CODE_UP,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(spawn_player_system))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(user_movement_system)
                    .with_system(slow_user_movement_system)
                    .with_system(flip_player_system)
                    .with_system(animation_update_system),
            );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    speed: Speed,
    animation_state: PlayerState,
    facing: Facing,
    racket_hand: RacketHand,
}

#[derive(Component)]
pub enum Player {
    User,
    Opponent,
}

#[derive(Component)]
struct Speed(f32);

impl Speed {
    const USER_DEFAULT: Self = Self(10.);
    const USER_SLOW: Self = Self(4.);
}

#[derive(Component, PartialEq, Eq, Hash)]
pub enum PlayerState {
    ServeReady,
    ServeToss,
    ServeHit,
    Idle,
    Run,
    Charge,
    Swing,
}

use super::animation::{Animation, PlayerFrameData};

#[derive(Component)]
pub enum Facing {
    Away,
    Toward,
}

#[derive(Component)]
pub enum RacketHand {
    Right,
    Left,
}

#[derive(Component)]
struct PlayerDirection(Vec3);

#[derive(Component)]
struct SwingCooldown(Timer);

#[derive(Component)]
struct UserControlled;

#[derive(Component)]
struct User;

#[derive(Component)]
struct Opponent;

#[derive(Component)]
struct CpuControlled;

fn spawn_player_system(
    animations: Res<PlayerFrameData>,
    mut commands: Commands,
    texture_atlas_handles: Res<TextureAtlasHandles>,
) {
    commands
        .spawn_bundle(PlayerBundle {
            player: Player::User,
            speed: Speed::USER_DEFAULT,
            animation_state: PlayerState::Idle,
            facing: Facing::Away,
            racket_hand: RacketHand::Right,
        })
        .insert(User)
        .insert(animations.0[&PlayerState::Idle].clone())
        .insert_bundle((
            Position::default(),
            BaseOffset(Vec2::new(0., -10.5)),
            PositionSync,
        ))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handles.player.clone(),
            sprite: TextureAtlasSprite {
                index: 4,
                ..default()
            },
            ..default()
        });
}

fn animation_update_system(
    animations: Res<PlayerFrameData>,
    mut query: Query<(&Player, &PlayerState, &mut Animation), Changed<PlayerState>>,
) {
    for (player, state, mut animation) in query.iter_mut() {
        *animation = match (player, state) {
            (Player::User, PlayerState::Idle) => animations.0[&PlayerState::Idle].clone(),
            (Player::User, PlayerState::Run) => animations.0[&PlayerState::Run].clone(),
            _ => todo!(),
        };
    }
}

fn slow_user_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerState, &mut Speed), With<User>>,
) {
    for (_state, mut speed) in query.iter_mut() {
        *speed = if keyboard.pressed(KEY_CODE_SECONDARY) {
            Speed::USER_SLOW
        } else {
            Speed::USER_DEFAULT
        };
    }
}

fn user_movement_system(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerState, &Speed, &mut Position), With<User>>,
) {
    for (mut state, speed, mut position) in query.iter_mut() {
        if matches!(*state, PlayerState::Charge | PlayerState::Swing) {
            continue;
        }
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KEY_CODE_RIGHT) {
            direction += Vec3::X;
        }
        if keyboard.pressed(KEY_CODE_LEFT) {
            direction -= Vec3::X;
        }
        if keyboard.pressed(KEY_CODE_UP) {
            direction -= Vec3::Z;
        }
        if keyboard.pressed(KEY_CODE_DOWN) {
            direction += Vec3::Z;
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

struct FlipTimer(Timer);

impl Default for FlipTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2., true))
    }
}

fn flip_player_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut RacketHand, &mut TextureAtlasSprite), With<User>>,
) {
    if keyboard.any_just_pressed([KEY_CODE_LEFT, KEY_CODE_RIGHT])
        && !keyboard.pressed(KEY_CODE_SECONDARY)
    {
        for (mut hand, mut sprite) in query.iter_mut() {
            if keyboard.just_pressed(KEY_CODE_LEFT) && matches!(*hand, RacketHand::Right) {
                *hand = RacketHand::Left;
                sprite.flip_x = !sprite.flip_x;
            } else if keyboard.just_pressed(KEY_CODE_RIGHT) && matches!(*hand, RacketHand::Left) {
                *hand = RacketHand::Right;
                sprite.flip_x = !sprite.flip_x;
            }
        }
    }
}

enum SwingState {
    Charge,
    Release,
}

enum ServeState {
    Ready,
    Toss,
    Hit,
}

enum State {
    Idle,
    Swing(SwingState),
    Serve(ServeState),
}

fn input_system(keyboard: Res<Input<KeyCode>>, query: Query<&mut PlayerState>) {
    for state in query.iter() {
        match state {
            PlayerState::ServeReady => todo!(),
            PlayerState::ServeToss => todo!(),
            PlayerState::ServeHit => todo!(),
            PlayerState::Idle => todo!(),
            PlayerState::Run => todo!(),
            PlayerState::Charge => todo!(),
            PlayerState::Swing => todo!(),
        }
    }
}

// fn opponent_movement_system(
//     time: Res<Time>,
//     mut player_query: Query<
//         (&mut PlayerState, &mut WorldPosition, &PlayerSpeed),
//         (With<CpuControlled>, Without<GameBall>),
//     >,
//     ball_query: Query<&WorldPosition, With<GameBall>>,
// ) {
//     for (mut opponent_state, mut opponent_pos, speed) in player_query.iter_mut() {
//         if let Ok(ball_pos) = ball_query.get_single() {
//             let delta_x = ball_pos.0.x - opponent_pos.0.x;
//             let direction = if delta_x > 0. { Vec3::X } else { -Vec3::X };
//             let max_distance = speed.0 * time.delta().as_secs_f32();
//             if delta_x.abs() <= max_distance {
//                 opponent_pos.0.x = ball_pos.0.x;
//                 if matches!(*opponent_state, PlayerState::Run) {
//                     *opponent_state = PlayerState::Idle;
//                 }
//             } else {
//                 opponent_pos.0 += max_distance * direction;
//                 if matches!(*opponent_state, PlayerState::Idle) {
//                     *opponent_state = PlayerState::Run;
//                 }
//             }
//         }
//     }
// }

// fn opponent_hit_system(
//     mut commands: Commands,
//     mut player_query: Query<
//         (Entity, &mut PlayerState, &WorldPosition),
//         (With<CpuControlled>, Without<GameBall>),
//     >,
//     mut hit_events: EventWriter<HitEvent>,
//     mut ball_query: Query<(Entity, &WorldPosition, &mut LastHitBy), With<GameBall>>,
// ) {
//     for (opponent_id, mut opponent_state, opponent_pos) in player_query.iter_mut() {
//         if matches!(*opponent_state, PlayerState::Charge | PlayerState::Swing) {
//             continue;
//         }
//         if let Ok((ball_id, ball_pos, mut last_hit)) = ball_query.get_single_mut() {
//             if (ball_pos.0.y - opponent_pos.0.y).abs() < 0.25 {
//                 let delta_x = ball_pos.0.x - opponent_pos.0.x;
//                 if delta_x.abs() < 2.0 {
//                     *last_hit = LastHitBy(Player::Opponent);
//                     let return_velocity = Vec3::new(
//                         rand::random::<f32>() * 6.,
//                         rand::random::<f32>() * 2.0 - 17.,
//                         10.,
//                     );
//                     hit_events.send(HitEvent {
//                         new_velocity: return_velocity,
//                         ball_id,
//                     });
//                     *opponent_state = PlayerState::Swing;
//                     commands
//                         .entity(opponent_id)
//                         .insert(SwingCooldown(Timer::from_seconds(1.0, false)));
//                 }
//             }
//         }
//     }
// }

// fn set_player_speed_system(
//     mut query: Query<(&mut PlayerSpeed, &PlayerState), With<UserControlled>>,
// ) {
//     for (mut player_speed, player_state) in query.iter_mut() {
//         player_speed.0 = match player_state {
//             PlayerState::Idle | PlayerState::Run => PLAYER_SPEED,
//             PlayerState::Charge => PLAYER_SPEED * PLAYER_CHARGING_SPEED_FACTOR,
//             PlayerState::Swing => 0.,
//         };
//     }
// }

// fn user_begin_charge_system(
//     keyboard: Res<Input<KeyCode>>,
//     mut query: Query<&mut PlayerState, With<UserControlled>>,
// ) {
//     if keyboard.just_pressed(KEY_CODE_ACTION) {
//         for mut state in query.iter_mut() {
//             if matches!(*state, PlayerState::Idle | PlayerState::Run) {
//                 *state = PlayerState::Charge;
//             }
//         }
//     }
// }

// fn user_release_charge_system(
//     mut commands: Commands,
//     keyboard: Res<Input<KeyCode>>,
//     mut player_query: Query<
//         (Entity, &mut PlayerState, &PlayerFacing, &WorldPosition),
//         With<UserControlled>,
//     >,
//     mut ball_query: Query<(Entity, &WorldPosition, &mut LastHitBy), With<GameBall>>,
//     mut hit_events: EventWriter<HitEvent>,
// ) {
//     if keyboard.just_released(KEY_CODE_ACTION) {
//         for (entity, mut player_state, player_facing, player_position) in player_query.iter_mut() {
//             if matches!(*player_state, PlayerState::Charge) {
//                 *player_state = PlayerState::Swing;
//                 commands
//                     .entity(entity)
//                     .insert(SwingCooldown(Timer::from_seconds(
//                         PLAYER_SWING_COOLDOWN_SECS,
//                         false,
//                     )));
//                 let flip = if matches!(player_facing, PlayerFacing::Left) {
//                     -1.0
//                 } else {
//                     1.0
//                 };
//                 let sweet_spot = player_position.0 + Vec3::new(9.0 * flip, 0.0, 11.0) / WORLD_SCALE;
//                 for (ball_id, ball_pos, mut last_hit) in ball_query.iter_mut() {
//                     let dist_to_ball = (sweet_spot - ball_pos.0).length();
//                     // let direction_left = (Vec3::new(X_SINGLES_LINE_LEFT, Y_FAR_BASELINE, 3.)
//                     //     - player_position.0)
//                     //     .normalize();
//                     // let direction_center = (Vec3::new(X_CENTER_LINE, Y_FAR_BASELINE, 0.)
//                     //     - player_position.0)
//                     //     .normalize();
//                     // let direction_right = (Vec3::new(X_SINGLES_LINE_RIGHT, Y_FAR_BASELINE, 3.)
//                     //     - player_position.0)
//                     //     .normalize();
//                     // let hit_direction_xy = if keyboard.pressed(KEY_CODE_LEFT) {
//                     //     direction_left
//                     // } else if keyboard.pressed(KEY_CODE_RIGHT) {
//                     //     direction_right
//                     // } else {
//                     //     direction_center
//                     // };
//                     // let direction = 20.0 * hit_direction_xy + Vec3::Z * 7.;
//                     let direction = -Vec3::Z;
//                     info!("{dist_to_ball:?}");
//                     if dist_to_ball < 2.0 {
//                         *last_hit = LastHitBy(Player::User);
//                         hit_events.send(HitEvent {
//                             ball_id,
//                             new_velocity: direction,
//                         });
//                     }
//                 }
//             }
//         }
//     }
// }

// fn tick_swing_cooldown_system(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut query: Query<(Entity, &mut PlayerState, &mut SwingCooldown)>,
// ) {
//     for (player_id, mut player_state, mut timer) in query.iter_mut() {
//         if timer.0.tick(time.delta()).just_finished() {
//             *player_state = PlayerState::Idle;
//             commands.entity(player_id).remove::<SwingCooldown>();
//         }
//     }
// }

// fn flip_sprite_facing_system(mut query: Query<(&PlayerFacing, &mut TextureAtlasSprite)>) {
//     for (facing, mut sprite) in query.iter_mut() {
//         sprite.flip_x = match facing {
//             PlayerFacing::Right => false,
//             PlayerFacing::Left => true,
//         }
//     }
// }

// fn turn_player_toward_ball(
//     mut player_query: Query<(&mut PlayerFacing, &PlayerState, &WorldPosition)>,
//     ball_query: Query<&WorldPosition, With<GameBall>>,
// ) {
//     for (mut player_facing, player_state, player_pos) in player_query.iter_mut() {
//         if matches!(*player_state, PlayerState::Idle | PlayerState::Run) {
//             if let Ok(ball_pos) = ball_query.get_single() {
//                 *player_facing = if ball_pos.0.x - player_pos.0.x > 0. {
//                     PlayerFacing::Right
//                 } else {
//                     PlayerFacing::Left
//                 };
//             }
//         }
//     }
// }

// fn player_spawn_system(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     mut events: EventReader<SpawnPlayerEvent>,
// ) {
//     let player_texture_handle = asset_server.get_handle("textures/player.png");
//     let player_texture_atlas =
//         TextureAtlas::from_grid(player_texture_handle, Vec2::new(24.0, 24.0), 4, 8);
//     let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);

//     let opponent_texture_handle = asset_server.get_handle("textures/opponent.png");
//     let opponent_texture_atlas =
//         TextureAtlas::from_grid(opponent_texture_handle, Vec2::new(24.0, 24.0), 4, 8);
//     let opponent_texture_atlas_handle = texture_atlases.add(opponent_texture_atlas);

//     for ev in events.iter() {
//         let speed = if ev.opponent {
//             PLAYER_SPEED * 0.5
//         } else {
//             PLAYER_SPEED
//         };
//         let texture_atlas_handle = if ev.opponent {
//             opponent_texture_atlas_handle.clone()
//         } else {
//             player_texture_atlas_handle.clone()
//         };
//         let id = commands
//             .spawn_bundle((
//                 if ev.opponent {
//                     Player::Opponent
//                 } else {
//                     Player::User
//                 },
//                 PlayerState::Idle,
//                 PlayerSpeed(speed),
//                 PlayerFacing::Right,
//             ))
//             .insert_bundle((
//                 ev.position,
//                 SyncWorldPosition,
//                 WorldSprite {
//                     base: Vec2::new(0.0, -10.5),
//                     ..Default::default()
//                 },
//             ))
//             .insert_bundle(SpriteSheetBundle {
//                 texture_atlas: texture_atlas_handle.clone(),
//                 ..Default::default()
//             })
//             .insert(SpriteAnimation::player_idle())
//             .id();
//         if ev.opponent {
//             commands.entity(id).insert(Opponent).insert(CpuControlled);
//         } else {
//             commands.entity(id).insert(UserControlled);
//         }
//         commands
//             .spawn_bundle((
//                 Shadow {
//                     parent: id,
//                     scale: 1.,
//                 },
//                 WorldPosition::default(),
//                 WorldSprite {
//                     base: Vec2::new(0., -10.5),
//                 },
//                 SyncWorldPosition,
//             ))
//             .insert_bundle(SpriteSheetBundle {
//                 sprite: TextureAtlasSprite {
//                     index: 15,
//                     ..default()
//                 },
//                 texture_atlas: player_texture_atlas_handle.clone(),
//                 ..default()
//             });
//     }
// }
