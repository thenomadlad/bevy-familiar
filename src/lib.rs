use bevy::feathers::FeathersPlugins;
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::UiTheme;
use bevy::prelude::*;
use bevy_enhanced_input::EnhancedInputPlugin;

use crate::mainmenu::MainMenuPlugin;

mod game;
mod mainmenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

impl AppState {
    pub fn default_next(&self) -> Self {
        match *self {
            AppState::InGame => AppState::MainMenu,
            _ => AppState::MainMenu,
        }
    }
}

pub struct FamiliarApp;

impl Plugin for FamiliarApp {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins((FeathersPlugins, EnhancedInputPlugin))
            .add_plugins((MainMenuPlugin, game::plugin))
            .insert_resource(UiTheme(create_dark_theme()))
            .add_systems(Startup, setup_app);
    }
}

fn setup_app(mut commands: Commands) {
    commands.spawn(Camera2d);
}
