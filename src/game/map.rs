use avian2d::collision::{
    collider::{Collider, Sensor},
    collision_events::{CollisionEventsEnabled, CollisionStart},
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::game::{GameplaySet, Inventory, Witch, inventory::InventoryItem};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<SpawnHandler>()
        .add_systems(Update, tick_and_spawn.in_set(GameplaySet));
}

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct ForestMap;

#[derive(Resource, Clone, Debug)]
struct SpawnHandler {
    timer: Timer,
}

impl Default for SpawnHandler {
    fn default() -> Self {
        SpawnHandler {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct SpawnPoint;

fn pickup(
    ev: On<CollisionStart>,
    witches: Query<(), With<Witch>>,
    items: Query<&InventoryItem>,
    mut inventory: ResMut<Inventory>,
    mut commands: Commands,
) {
    if !witches.contains(ev.collider2) {
        return;
    }
    if let Ok(item) = items.get(ev.collider1) {
        inventory.add(item.clone());
    }
    commands.entity(ev.collider1).despawn();
}

fn tick_and_spawn(
    time: Res<Time>,
    mut spawn_handler: ResMut<SpawnHandler>,
    empty_points: Query<Entity, (With<SpawnPoint>, Without<Children>)>,
    mut commands: Commands,
) {
    if spawn_handler.timer.tick(time.delta()).just_finished() {
        for spawn_point in empty_points.iter() {
            let item = InventoryItem::random_item();
            let image_path = format!(
                "Humble Gift - Paper UI System/Humble Gift - Paper UI System v1.1/Sprites/Content/1 Items/{}.png",
                item.index()
            );

            commands
                .entity(spawn_point)
                .queue_spawn_related_scenes::<Children>(bsn_list! {
                    item
                    Sprite { image: image_path, custom_size: Vec2::splat(16.0) }
                    Collider::circle(6.0)
                    Sensor
                    CollisionEventsEnabled
                    on(pickup)
                });
        }
    }
}

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
