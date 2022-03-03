use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::*;

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
            HitQuality::Perfect => 1000.0,
            HitQuality::Amazing => 500.0,
            HitQuality::Great => 300.0,
            HitQuality::Good => 240.0,
            HitQuality::Poor => 100.0,
            HitQuality::Miss => 0.0,
        }
    }
}

fn hit_ball(
    mouse_position: Res<MousePosition>,
    mut events: EventReader<PrimaryButtonPress>,
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
        for (ball_transform, ball_pos, mut ball_vel) in query.iter_mut() {
            let distance =
                (mouse_position.0.unwrap().extend(0.0) - ball_transform.translation).length();
            let quality = HitQuality::from_dist(distance);
            if matches!(quality, HitQuality::Miss) {
                continue;
            }
            let target_x = rand::random::<f32>() * 10.0;
            let world_target = Vec3::new(target_x, 200.0, 0.0);
            let new_velocity = (world_target - Vec3::from(ball_pos.position.translation))
                .normalize()
                * quality.return_speed();
            ball_vel.linvel = new_velocity.into();
        }
    }
}

fn move_player_keyboard(
    mut events: EventReader<MovePlayerEvent>,
    mut query: Query<
        (&mut WorldPosition, Option<&InvertControls>),
        (With<Player>, With<UserControlled>),
    >,
) {
    for ev in events.iter() {
        for (mut position, invert) in query.iter_mut() {
            let scale = if invert.is_some() { -1.0 } else { 1.0 };
            position.0 += ev.direction.normalize() * 0.3 * scale;
        }
    }
}

fn flip_player_sprite(mut players: Query<(&mut TextureAtlasSprite, &Transform), With<Player>>) {
    for (mut sprite, transform) in players.iter_mut() {
        sprite.flip_x = transform.translation.x > 0.0;
    }
}
