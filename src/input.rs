use bevy::prelude::*;

use crate::GameState;

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_event::<PrimaryButtonPress>()
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(sync_mouse_position_2d)
                    .with_system(send_input_events),
            );
    }
}

#[derive(Default)]
pub(crate) struct MousePosition(Option<Vec2>);

#[derive(Component)]
pub(crate) struct MainCamera;

pub(crate) struct PrimaryButtonPress;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn send_input_events(
    mouse_input: Res<Input<MouseButton>>,
    mut events: EventWriter<PrimaryButtonPress>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        events.send(PrimaryButtonPress)
    }
}

fn sync_mouse_position_2d(
    windows: Res<Windows>,
    mut mouse_position: ResMut<MousePosition>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    mouse_position.0 = try {
        let (camera, camera_transform) = camera_query.get_single().ok()?;
        let active_window = windows.get(camera.window)?;
        let window_size = Vec2::new(active_window.width(), active_window.height());
        let screen_pos = windows.get(camera.window)?.cursor_position()?;
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let world_pos: Vec2 = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
        world_pos
    };
}
