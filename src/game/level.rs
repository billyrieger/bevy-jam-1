use crate::*;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_scene));
    }
}

fn setup_scene(
    mut ball_events: EventWriter<SpawnBallEvent>,
    mut court_events: EventWriter<SpawnCourtEvent>,
    mut player_events: EventWriter<SpawnPlayerEvent>,
) {
    ball_events.send(SpawnBallEvent {
        position: WorldPosition(Vec3::new(-10.0, 14.0, 3.0)),
        velocity: RigidBodyVelocity {
            linvel: Vec3::new(10., -20., 10.).into(),
            angvel: Vec3::new(1.0, 0.0, 0.0).into(),
            ..Default::default()
        },
    });
    court_events.send(SpawnCourtEvent);
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, Y_NEAR_BASELINE, 0.0)),
        opponent: false,
    });
    player_events.send(SpawnPlayerEvent {
        position: WorldPosition(Vec3::new(0.0, Y_FAR_BASELINE, 0.0)),
        opponent: true,
    });
}
