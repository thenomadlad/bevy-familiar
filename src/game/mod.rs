use avian2d::schedule::{Physics, PhysicsTime};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{
    AppState,
    game::{
        hud::Hud,
        inventory::Inventory,
        map::{ForestMap, SpawnPoint},
        witch::Witch,
    },
};

mod familiars;
mod hud;
mod inventory;
mod map;
mod tasks;
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
    Items,
    Task,
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Inventory>()
        .configure_sets(Update, GameplaySet.run_if(in_state(GameState::Running)))
        .add_sub_state::<GameState>()
        .add_input_context::<GameScene>()
        .add_plugins((
            hud::plugin,
            witch::plugin,
            map::plugin,
            tasks::plugin,
            familiars::plugin,
        ))
        .add_systems(OnEnter(AppState::InGame), scene.spawn())
        .add_systems(OnExit(GameState::Running), pause_physics)
        .add_systems(OnEnter(GameState::Running), unpause_physics);
}

fn back_to_main(_: On<Start<EscapeToMain>>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame.default_next());
}

fn pause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.as_mut().pause();
}

fn unpause_physics(mut physics_time: ResMut<Time<Physics>>) {
    physics_time.as_mut().unpause();
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
            --
            SpawnPoint
            Transform::from_xyz(-200.0, 0.0, 11.0)
            Visibility
            --
            SpawnPoint
            Transform::from_xyz(170.0, -60.0, 11.0)
            Visibility
            --
            SpawnPoint
            Transform::from_xyz(40.0, 800.0, 11.0)
            Visibility
        ]
        on(back_to_main)
        DespawnOnExit<AppState>(AppState::InGame)
    }
}
