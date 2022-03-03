use crate::*;

pub(crate) struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResourceHandles>()
            .add_system_set(
                SystemSet::on_enter(GameState::Loading)
                    .with_system(begin_resource_loading)
                    .with_system(setup),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Loading).with_system(check_resource_loading),
            );
    }
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(UiCamera);
    rapier_config.gravity = Vec3::new(0.0, 0.0, -10.0).into();
}

fn begin_resource_loading(
    mut texture_handles: ResMut<ResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    let fonts = ["fonts/Press_Start_2P/PressStart2P-Regular.ttf"];
    let textures = ["textures/ball.png", "player.png", "court_grass.png"];
    texture_handles.0.extend(
        std::iter::empty()
            .chain(fonts)
            .chain(textures)
            .map(|filename| asset_server.load_untyped(filename)),
    );
}

fn check_resource_loading(
    mut state: ResMut<State<GameState>>,
    resource_handles: ResMut<ResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    let handle_ids = resource_handles.0.iter().map(|handle| handle.id);
    if matches!(
        asset_server.get_group_load_state(handle_ids),
        bevy::asset::LoadState::Loaded
    ) {
        state.set(GameState::InGame).unwrap();
    }
}
