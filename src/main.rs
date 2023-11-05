use bevy::prelude::*;

use crate::gameplay::GameplayPlugin;
use crate::main_menu::MainMenuPlugin;

// TODO
// v bug: hit on dice can lead to ball leaking through wall
// v add main menu
// v add game restart
// * add delayed ball start
// * add player lost screen
// * add scores ui
// * add screen margin and fix window size
// * add window scaling
// * add ball loose effect
// * add difficulty selector
// * add ai player
// * add sound effects
// * add mouse play mode

mod gameplay;
mod main_menu;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Gameplay,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
        .add_state::<GameState>()
        .add_plugins((DefaultPlugins, MainMenuPlugin, GameplayPlugin))
        .add_systems(Startup, global_setup)
        .run();
}

fn global_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
