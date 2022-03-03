use crate::*;

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_event::<MovePlayerEvent>()
            .add_event::<PrimaryButtonPress>()
            .add_system(bevy::input::system::exit_on_esc_system)
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(sync_mouse_position_2d)
                    .with_system(send_input_events),
            );
    }
}

fn send_input_events(
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut events: EventWriter<PrimaryButtonPress>,
    mut move_events: EventWriter<MovePlayerEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        events.send(PrimaryButtonPress);
    }
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction -= Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction += Vec3::X;
    }
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction -= Vec3::Y;
    }
    if direction.length() > 0.01 {
        move_events.send(MovePlayerEvent { direction });
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
        ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
    };
}
