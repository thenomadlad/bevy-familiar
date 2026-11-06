use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    AppState,
    game::{hud::Hud, map::ForestMap, witch::Witch},
};

mod hud;
mod map;
mod witch;

#[derive(Component, Default, Debug, Clone)]
struct GameScene;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
struct GameplaySet;

#[derive(InputAction)]
#[action_output(bool)]
struct EscapeToMain;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
enum GameState {
    #[default]
    Running,
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(Update, GameplaySet.run_if(in_state(GameState::Running)))
        .add_sub_state::<GameState>()
        .add_input_context::<GameScene>()
        .add_plugins((hud::plugin, witch::plugin, map::plugin))
        .add_systems(OnEnter(AppState::InGame), scene.spawn());
}

fn back_to_main(_: On<Start<EscapeToMain>>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame.default_next());
}

fn scene() -> impl SceneList {
    bsn_list! {
        // UI root
        @Hud
        DespawnOnExit<AppState>(AppState::InGame)
        --
        // world/scene root
        Transform
        Visibility
        GameScene
        Actions<GameScene> [
            Action<EscapeToMain>
            Bindings [
                Binding(KeyCode::Escape)
            ]
        ]
        Children [
            @Witch
            --
            @ForestMap
        ]
        on(back_to_main)
        DespawnOnExit<AppState>(AppState::InGame)
    }
}
