use bevy::feathers::controls::FeathersButton;
use bevy::feathers::display::caption;
use bevy::feathers::theme::ThemeBackgroundColor;
use bevy::feathers::tokens;
use bevy::input_focus::AutoFocus;
use bevy::prelude::*;
use bevy::ui::FlexDirection::Column;
use bevy::ui_widgets::Activate;

use crate::AppState;

pub struct MainMenuPlugin;

fn handle_play_button(_: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

#[cfg(not(target_arch = "wasm32"))]
fn handle_exit_button(_: On<Activate>, mut exit_writer: MessageWriter<AppExit>) {
    exit_writer.write(AppExit::Success);
}

#[cfg(not(target_arch = "wasm32"))]
fn optional_exit_button() -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! { @caption("Quit") }
        }
          AccessibleLabel("Quit")
        on(handle_exit_button)
    }
}

#[cfg(target_arch = "wasm32")]
fn optional_exit_button() -> impl Scene {
    bsn! {}
}

fn main_menu() -> impl Scene {
    bsn! {
      Node {
        width: percent(100),
        height: percent(100),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        flex_direction: Column,
        row_gap: px(20)
      }
      ThemeBackgroundColor(tokens::WINDOW_BG)
      Children [
        @FeathersButton {
            @caption: bsn! { @caption("Play") }
        }
          AccessibleLabel("Play")
          AutoFocus
        on(handle_play_button)
        --
        @optional_exit_button()
      ]
      DespawnOnExit<AppState>(AppState::MainMenu)
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), main_menu.spawn());
    }
}
