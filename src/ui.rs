use crate::*;

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_ui_text))
            .add_system_set(
                SystemSet::on_update(AppState::InGame).with_system(sync_score_text_system),
            );
    }
}

fn sync_score_text_system(
    user_score: ResMut<UserScore>,
    opponent_score: ResMut<OpponentScore>,
    mut user_text: Query<&mut Text, (With<UserScoreText>, Without<OpponentScoreText>)>,
    mut opponent_text: Query<&mut Text, (With<OpponentScoreText>, Without<UserScoreText>)>,
) {
    if let Ok(mut text) = user_text.get_single_mut() {
        text.sections[0].value = format!("User:     {}", user_score.0);
    }
    if let Ok(mut text) = opponent_text.get_single_mut() {
        text.sections[0].value = format!("Opponent: {}", opponent_score.0);
    }
}

fn setup_ui_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "User:     0",
                TextStyle {
                    font: asset_server.load("fonts/Press_Start_2P/PressStart2P-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(UserScoreText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(32.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "Opponent: 0",
                TextStyle {
                    font: asset_server.load("fonts/Press_Start_2P/PressStart2P-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(OpponentScoreText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/Press_Start_2P/PressStart2P-Regular.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..default()
                },
            ),
            ..default()
        })
        .insert(ResultsText);
}
