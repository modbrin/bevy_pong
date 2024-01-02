use bevy::prelude::*;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;

use crate::game_over::GameOverPlugin;
use crate::gameplay::GameplayPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::utils::*;

// TODO
// v bug: hit on dice can lead to ball leaking through wall
// v add main menu
// v add game restart
// v add delayed ball start
// v add player lost screen
// v add scores ui
// ? add window scaling
// ? add screen margin and fix window size
// * add ball loose effect (scale down)
// * add ai player
// * add sound effects
// * add mouse play mode
// * add difficulty selector

mod game_over;
mod gameplay;
mod main_menu;
mod utils;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Gameplay,
    GameOver,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Resource)]
pub struct LastWinner {
    player: Option<DiceKind>,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
        .add_state::<GameState>()
        .insert_resource(LastWinner::default())
        .add_plugins((
            DefaultPlugins
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1100.0, 1000.0).into(),
                        resize_constraints: WindowResizeConstraints {
                            min_width: 1100.0,
                            min_height: 1000.0,
                            ..default()
                        },
                        ..default()
                    }),
                    ..default()
                }),
            MainMenuPlugin,
            GameplayPlugin,
            GameOverPlugin,
        ))
        .add_systems(Startup, global_setup)
        .run();
}

fn global_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
