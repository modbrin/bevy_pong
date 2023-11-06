use bevy::app::AppExit;
use bevy::prelude::*;

use crate::utils::*;
use crate::GameState;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct MenuUIRoot;

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu)
            .add_systems(Update, (play_button_clicked, quit_button_clicked));
    }
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = spawn_button(&mut commands, &asset_server, "PvP", Color::LIME_GREEN);
    commands.entity(start_button).insert(StartButton);

    let quit_button = spawn_button(&mut commands, &asset_server, "Quit", Color::DARK_GRAY);
    commands.entity(quit_button).insert(QuitButton);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(MenuUIRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    "PONG",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 96.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .add_child(start_button)
        .add_child(quit_button);
}

fn despawn_menu(mut commands: Commands, menu_root: Query<Entity, With<MenuUIRoot>>) {
    let root_entity = menu_root.single();
    commands.entity(root_entity).despawn_recursive();
}

fn play_button_clicked(
    interactions: Query<&Interaction, (With<StartButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            next_state.set(GameState::Gameplay);
        }
    }
}

fn quit_button_clicked(
    interactions: Query<&Interaction, (With<QuitButton>, Changed<Interaction>)>,
    mut event_writer: EventWriter<AppExit>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            event_writer.send(AppExit);
        }
    }
}
