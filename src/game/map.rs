use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<ForestMap>();
}

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct ForestMap;

impl ForestMap {
    fn scene() -> impl Scene {
        bsn! {}
    }
}
