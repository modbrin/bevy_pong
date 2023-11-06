use bevy::prelude::*;

use crate::utils::*;
use crate::{GameState, LastWinner};

pub struct GameOverPlugin;

#[derive(Component)]
pub struct GameOverUIRoot;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct MainMenuButton;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_menu)
            .add_systems(OnExit(GameState::GameOver), despawn_menu)
            .add_systems(Update, (restart_button_clicked, main_menu_button_clicked));
    }
}

fn spawn_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    last_winner: Res<LastWinner>,
) {
    let restart_button = spawn_button(&mut commands, &asset_server, "Restart", Color::GRAY);
    commands.entity(restart_button).insert(RestartButton);

    let main_menu_button = spawn_button(&mut commands, &asset_server, "Main Menu", Color::GRAY);
    commands.entity(main_menu_button).insert(MainMenuButton);

    let player_name = match &last_winner.player {
        Some(player) => {
            format!("{:?}", player)
        }
        _ => unreachable!(),
    };

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
        .insert(GameOverUIRoot)
        .with_children(|commands| {
            commands.spawn(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Percent(3.0)),
                    ..default()
                },
                text: Text::from_section(
                    format!("{} Player Won", player_name),
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 60.0,
                        color: Color::BLACK,
                    },
                ),
                ..default()
            });
        })
        .add_child(restart_button)
        .add_child(main_menu_button);
}

fn despawn_menu(mut commands: Commands, menu_root: Query<Entity, With<GameOverUIRoot>>) {
    let root_entity = menu_root.single();
    commands.entity(root_entity).despawn_recursive();
}

fn restart_button_clicked(
    interactions: Query<&Interaction, (With<RestartButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            next_state.set(GameState::Gameplay);
        }
    }
}

fn main_menu_button_clicked(
    interactions: Query<&Interaction, (With<MainMenuButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interactions {
        if matches!(interaction, Interaction::Pressed) {
            next_state.set(GameState::MainMenu);
        }
    }
}
