use bevy::prelude::*;

use crate::game::GameplaySet;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FamiliarsManager>()
        .add_systems(Update, (tick_and_move).in_set(GameplaySet));
}

fn tick_and_move() {}

#[derive(Resource, Clone, Default)]
struct FamiliarsManager(Vec<Familiar>);

#[derive(Clone, Default)]
struct Familiar {}
