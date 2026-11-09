use avian2d::PhysicsPlugins;
use avian2d::debug_render::PhysicsDebugPlugin;
use avian2d::dynamics::integrator::Gravity;
use avian2d::dynamics::rigid_body::RigidBody;
use bevy::feathers::FeathersPlugins;
use bevy::feathers::dark_theme::create_dark_theme;
use bevy::feathers::theme::UiTheme;
use bevy::prelude::*;
use bevy_ecs_tiled::physics::TiledPhysicsPlugin;
use bevy_ecs_tiled::physics::backend::avian::TiledPhysicsAvianBackend;
use bevy_ecs_tiled::physics::collider::ColliderCreated;
use bevy_ecs_tiled::tiled::TiledPlugin;
use bevy_ecs_tiled::tiled::event::TiledEvent;
use bevy_enhanced_input::EnhancedInputPlugin;

use crate::mainmenu::MainMenuPlugin;

mod game;
mod mainmenu;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

impl AppState {
    pub fn default_next(&self) -> Self {
        match *self {
            AppState::InGame => AppState::MainMenu,
            _ => AppState::MainMenu,
        }
    }
}

pub struct FamiliarApp;

impl Plugin for FamiliarApp {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins((
                FeathersPlugins,
                EnhancedInputPlugin,
                TiledPlugin::default(),
                PhysicsPlugins::default(),
                TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            ))
            .add_plugins((MainMenuPlugin, game::plugin))
            .init_state::<AppState>()
            .insert_resource(UiTheme(create_dark_theme()))
            .insert_resource(Gravity::ZERO)
            .add_systems(Startup, setup_app)
            .add_observer(setup_collider_physics);
    }
}

fn setup_app(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 368.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn setup_collider_physics(created: On<TiledEvent<ColliderCreated>>, mut commands: Commands) {
    commands
        .entity(created.event().origin)
        .insert(RigidBody::Static);
}
