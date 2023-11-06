use crate::game_over::GameOverPlugin;
use bevy::prelude::*;

use crate::gameplay::GameplayPlugin;
use crate::main_menu::MainMenuPlugin;

// TODO
// v bug: hit on dice can lead to ball leaking through wall
// v add main menu
// v add game restart
// v add delayed ball start
// v add player lost screen
// * add scores ui
// * add screen margin and fix window size
// * add window scaling
// * add ball loose effect
// * add difficulty selector
// * add ai player
// * add sound effects
// * add mouse play mode

mod game_over;
mod gameplay;
mod main_menu;
mod utils;

use crate::utils::*;

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
            DefaultPlugins,
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
