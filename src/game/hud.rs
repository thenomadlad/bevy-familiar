use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<Hud>();
}

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct Hud;

impl Hud {
    fn scene() -> impl Scene {
        bsn! {
            Hud
            Node { width: percent(100), height: percent(100) }
            Pickable::IGNORE
            Text("hello")
            Actions<Hud> []
        }
    }
}
