use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<TaskList>();
}

#[derive(Resource, Clone, Default)]
struct TaskList {}

impl TaskList {
    fn current_task(&self) -> impl Scene {
        bsn! {}
    }
}
