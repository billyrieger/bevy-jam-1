#![feature(bool_to_option)]

use std::time::Duration;

use bevy::{asset::LoadState, prelude::*};
use bevy_easings::*;

fn main() {
    App::new()
        .add_state(GameState::Loading)
        .init_resource::<ResourceHandles>()
        .insert_resource(WindowDescriptor {
            width: 272.0 * 3.0,
            height: 256.0 * 3.0,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_resources))
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_resources))
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup_game))
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(sprite_animation_system)
                .with_system(sync_mouse_position)
                .with_system(move_player_to_mouse)
                .with_system(update_player_animation),
        )
        .run();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    InGame,
}

// ====== Resources ======

#[derive(Default)]
struct ResourceHandles {
    textures: Vec<HandleUntyped>,
}

// ====== Components ======

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct MousePosition(Vec2);

#[derive(Component)]
struct Court {
    _surface: Surface,
}

enum Surface {
    Grass,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
enum PlayerAnimationState {
    Idle,
}

#[derive(Component)]
struct SpriteAnimation(Vec<SpriteAnimationFrame>);

#[derive(Clone, Copy)]
struct SpriteAnimationFrame {
    sprite_index: usize,
    duration: Duration,
    flipped: bool,
}

#[derive(Bundle)]
struct SpriteAnimationBundle {
    frames: SpriteAnimation,
    timer: SpriteAnimationTimer,
}

impl SpriteAnimationBundle {
    fn from_frames(frames: SpriteAnimation, repeating: bool) -> Self {
        Self {
            timer: SpriteAnimationTimer(Timer::new(frames.duration(), repeating)),
            frames,
        }
    }
}

#[derive(Component)]
struct SpriteAnimationTimer(Timer);

impl SpriteAnimation {
    fn player_idle_south() -> Self {
        let frames = [(12, 0.3), (13, 0.1), (14, 0.2), (15, 0.1)]
            .map(|(i, dt)| SpriteAnimationFrame {
                sprite_index: i,
                duration: Duration::from_secs_f32(dt),
                flipped: false,
            })
            .to_vec();
        Self(frames)
    }

    fn duration(&self) -> Duration {
        self.0.iter().map(|frame| frame.duration).sum()
    }
}

// ====== Systems ======

fn sprite_animation_system(
    time: Res<Time>,
    mut query: Query<(
        &mut TextureAtlasSprite,
        &SpriteAnimation,
        &mut SpriteAnimationTimer,
    )>,
) {
    for (mut sprite, animation, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        let mut sum = Duration::ZERO;
        let (index, flipped) = animation
            .0
            .iter()
            .find_map(|frame| {
                sum += frame.duration;
                (timer.0.elapsed() <= sum).then_some((frame.sprite_index, frame.flipped))
            })
            .expect("overextended animation timer");
        sprite.index = index;
        sprite.flip_x = flipped;
    }
}

fn load_resources(mut texture_handles: ResMut<ResourceHandles>, asset_server: Res<AssetServer>) {
    texture_handles.textures = asset_server.load_folder("textures").unwrap();
}

fn check_resources(
    mut state: ResMut<State<GameState>>,
    texture_handles: ResMut<ResourceHandles>,
    asset_server: Res<AssetServer>,
) {
    if matches!(
        asset_server.get_group_load_state(texture_handles.textures.iter().map(|handle| handle.id)),
        LoadState::Loaded
    ) {
        state.set(GameState::InGame).unwrap();
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let scale = Transform::from_scale(Vec3::splat(3.0));

    let texture_handle = asset_server.get_handle("textures/player_male_light_white.png");
    let texture_atlas = TextureAtlas::from_grid_with_padding(
        texture_handle,
        Vec2::new(23.0, 23.0),
        8,
        5,
        Vec2::new(1.0, 1.0),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert(Player)
        .insert_bundle((
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        ..Default::default()
                    },
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_translation(Vec3::new(0.0, 30.0, 0.0))
                        .with_scale(Vec3::splat(3.0)),
                    ..Default::default()
                })
                .insert_bundle(SpriteAnimationBundle::from_frames(
                    SpriteAnimation::player_idle_south(),
                    true,
                ))
                .insert(PlayerAnimationState::Idle);
        });

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("textures/court_grass.png"),
            transform: scale,
            ..Default::default()
        })
        .insert(Court {
            _surface: Surface::Grass,
        });

    commands.spawn().insert(MousePosition(Vec2::ZERO));
}

fn sync_mouse_position(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut q_mouse: Query<&mut MousePosition>,
) {
    let (camera, camera_transform) = q_camera.single();
    let mut mouse_pos = q_mouse.single_mut();

    // get the window that the camera is displaying to
    let wnd = wnds.get(camera.window).unwrap();

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        mouse_pos.0 = world_pos;
    }
}

fn update_player_animation(
    mut query: Query<(&mut SpriteAnimation, &PlayerAnimationState), Changed<PlayerAnimationState>>,
) {
    for (mut anim, anim_state) in query.iter_mut() {
        *anim = match anim_state {
            PlayerAnimationState::Idle => SpriteAnimation::player_idle_south(),
        };
    }
}

fn move_player_to_mouse(
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mouse_query: Query<&MousePosition>,
) {
    if let Ok(mouse_pos) = mouse_query.get_single() {
        for mut transform in player_query.iter_mut() {
            let delta = mouse_pos.0 - transform.translation.truncate();
            let max_distance = time.delta_seconds() * (delta.length().min(100.0)) * 4.0;
            if delta.length() <= max_distance {
                transform.translation = mouse_pos.0.extend(transform.translation.z);
            } else {
                transform.translation += max_distance * delta.normalize().extend(0.0);
            }
        }
    }
}
