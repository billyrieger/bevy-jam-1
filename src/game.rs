use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::*;

mod player;

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(hit_ball)
                .with_system(flip_player_sprite)
                .with_system(move_player_keyboard),
        );
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
            HitQuality::Perfect => 100.0,
            HitQuality::Amazing => 50.0,
            HitQuality::Great => 30.0,
            HitQuality::Good => 24.0,
            HitQuality::Poor => 10.0,
            HitQuality::Miss => 0.0,
        }
    }
}

fn hit_ball(
    mut commands: Commands,
    mut events: EventReader<PrimaryKeyPress>,
    mut player_query: Query<(Entity, &mut PlayerState), With<UserControlled>>,
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
        let (player_id, mut player_state) = player_query.single_mut();
        *player_state = PlayerState::Swing;
        commands
            .entity(player_id)
            .insert(NextPlayerState(PlayerState::Idle));
        for (_, ball_pos, mut ball_vel) in query.iter_mut() {
            let distance = 40.0;
            let quality = HitQuality::from_dist(distance);
            if matches!(quality, HitQuality::Miss) {
                continue;
            }
            let target_x = rand::random::<f32>() * 10.0;
            let world_target = Vec3::new(target_x, 20.0, 20.0);
            let new_velocity = (world_target - Vec3::from(ball_pos.position.translation))
                .normalize()
                * quality.return_speed();
            ball_vel.linvel = new_velocity.into();
        }
    }
}

fn move_player_keyboard(
    mut events: EventReader<MovePlayerEvent>,
    mut query: Query<(&mut WorldPosition, &mut PlayerState), With<UserControlled>>,
) {
    for ev in events.iter() {
        for (mut position, mut player) in query.iter_mut() {
            if !matches!(*player, PlayerState::Run) {
                // *player = PlayerState::Run;
            }
            position.0 += ev.direction.normalize() * 0.3;
        }
    }
}

fn flip_player_sprite(
    mut players: Query<(&mut TextureAtlasSprite, &Transform), With<PlayerState>>,
) {
    for (mut sprite, transform) in players.iter_mut() {
        sprite.flip_x = transform.translation.x > 0.0;
    }
}
