use bevy::{
    prelude::*,
    ui_widgets::{Activate, Button},
};
use bevy_enhanced_input::prelude::*;

use crate::game::{GameState, Inventory, inventory::InventoryItem};

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<Hud>()
        .add_systems(OnEnter(GameState::Items), show_items_menu)
        .add_systems(OnEnter(GameState::Task), show_task_menu);
}

#[derive(SceneComponent, Default, Clone, Debug)]
pub(super) struct Hud;

#[derive(Component, Default, Clone, Debug)]
pub(super) struct MenuContainer;

#[derive(InputAction)]
#[action_output(bool)]
struct ItemsMenuToggle;

#[derive(InputAction)]
#[action_output(bool)]
struct TaskMenuToggle;

fn toggle_items_menu(
    _: On<Start<ItemsMenuToggle>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set_if_different(match current_state.get() {
        GameState::Items => GameState::Running,
        _ => GameState::Items,
    });
}

fn handle_items_button_click(
    _: On<Activate>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set_if_different(match current_state.get() {
        GameState::Items => GameState::Running,
        _ => GameState::Items,
    });
}

fn toggle_task_menu(
    _: On<Start<TaskMenuToggle>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set_if_different(match current_state.get() {
        GameState::Task => GameState::Running,
        _ => GameState::Task,
    });
}

fn handle_task_button_click(
    _: On<Activate>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set_if_different(match current_state.get() {
        GameState::Task => GameState::Running,
        _ => GameState::Task,
    });
}

fn item_icon(item: InventoryItem, count: usize) -> impl Scene {
    let image_path = format!(
        "Humble Gift - Paper UI System/Humble Gift - Paper UI System v1.1/Sprites/Content/1 Items/{}.png",
        item.index()
    );
    let count_str = count.to_string();
    bsn! {
        Node {
            width: px(32),
            height: px(32),
        }
        ImageNode {
            image: image_path,
            image_mode: NodeImageMode::Stretch
        }
        Children [
            Text(count_str)
            TextFont { font_size: px(12.0) }
            TextColor(Color::BLACK)
        ]
    }
}

fn show_items_menu(
    menu_container: Single<Entity, With<MenuContainer>>,
    inventory: Res<Inventory>,
    mut commands: Commands,
) {
    let aspect_ratio = 768.0 / 560.0;

    let icons = inventory
        .get_all()
        .into_iter()
        .map(|(item, count)| item_icon(item, count))
        .collect::<Vec<_>>();

    commands
        .entity(menu_container.entity())
        .queue_spawn_related_scenes::<Children>(bsn_list! {
            Node {
                height: percent(90),
                aspect_ratio: aspect_ratio,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center
            }
            ImageNode {
                image: "Humble Gift - Paper UI System/Humble Gift - Paper UI System v1.1/Sprites/Book Desk/5.png",
                image_mode: NodeImageMode::Stretch
            }
            Children [
                Node {
                    width: percent(40),
                    flex_wrap: FlexWrap::Wrap,
                    column_gap: px(8),
                    row_gap: px(8)
                }
                Children [
                    {icons}
                ]
            ]
            DespawnOnExit<GameState>(GameState::Items)
        });
}

fn show_task_menu(menu_container: Single<Entity, With<MenuContainer>>, mut commands: Commands) {
    let aspect_ratio = 768.0 / 560.0;
    commands
        .entity(menu_container.entity())
        .queue_spawn_related_scenes::<Children>(bsn_list! {
            Node {
                height: percent(90),
                aspect_ratio: aspect_ratio,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center
            }
            ImageNode {
                image: "Humble Gift - Paper UI System/Humble Gift - Paper UI System v1.1/Sprites/Book Desk/2.png",
                image_mode: NodeImageMode::Stretch
            }
            DespawnOnExit<GameState>(GameState::Task)
        });
}

fn button(label: &str) -> impl Scene {
    bsn! {
        Button
        Node {
            width: px(160),
            height: px(64),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center
        }
        ImageNode {
            image: "Humble Gift - Paper UI System/Humble Gift - Paper UI System v1.1/Sprites/Content/4 Buttons/0.png",
            image_mode: NodeImageMode::Stretch
        }
        Children [
            Text(label)
            TextFont {
                font_size: px(20.0),
            }
            TextColor(Color::BLACK)
        ]
    }
}

impl Hud {
    fn scene() -> impl Scene {
        bsn! {
            Node {
                width: percent(100),
                height: percent(100),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                align_items: AlignItems::Center
            }
            Pickable::IGNORE
            Actions<Hud> [
                Action<ItemsMenuToggle>
                Bindings [
                    Binding(KeyCode::KeyQ)
                ]
                --
                Action<TaskMenuToggle>
                Bindings [
                    Binding(KeyCode::KeyT)
                ]
            ]
            Children [
                @top_menu()
                --
                MenuContainer
                Node { flex_grow: 1.0, width: percent(100), justify_content: JustifyContent::Center, align_items: AlignItems::Center }
                Pickable::IGNORE
            ]
            on(toggle_items_menu)
            on(toggle_task_menu)
        }
    }
}

fn top_menu() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: px(100),
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Start,
            padding: UiRect::all(px(10)),
            column_gap: px(10),
        }
        Pickable::IGNORE
        Children [
            @button("Items (Q)")
            on(handle_items_button_click)
            --
            @button("Task (T)")
            on(handle_task_button_click)
        ]
    }
}
