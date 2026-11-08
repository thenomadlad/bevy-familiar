use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct ForestMap;

impl ForestMap {
    fn scene() -> impl Scene {
        bsn! {
            TiledMap("forest-map.tmx")
            TilemapAnchor::Center
            TiledMapLayerZOffset(1.0)
            TiledPhysicsSettings<TiledPhysicsAvianBackend> {
                tiles_layer_filter: TiledFilter::Names(vec![
                                        String::from("Water"),
                                        String::from("RockSlopes_Auto")
                ]),
                objects_layer_filter: TiledFilter::Names(vec![
                                        String::from("Props"),
                ])
            }
        }
    }
}
