// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod animation;
mod asset_tracking;
mod audio;
mod camera;
mod collision_layers;
#[cfg(feature = "dev")]
mod dev_tools;
mod enemy;
mod health;
mod level;
mod menus;
mod physics;
mod player;
mod screens;
mod script;
mod theme;

use avian2d::{PhysicsPlugins, math::*, prelude::*};
use bevy::{asset::AssetMetaCheck, prelude::*};
#[cfg(feature = "dev")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::physics::configs::GRAVITY_ACCELERATION;

pub const GAME_NAME: &'static str = "Vision of Asad";

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: GAME_NAME.to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );
        app.add_plugins((
            asset_tracking::plugin,
            PhysicsPlugins::default(),
            physics::plugin,
            #[cfg(feature = "dev")]
            PhysicsDebugPlugin::default(),
        ));
        // Add other plugins.
        app.add_plugins((
            audio::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            health::plugin,
            player::plugin,
            enemy::plugin,
            camera::plugin,
            animation::plugin,
            level::arena::plugin,
        ));
        app.add_plugins((script::plugin,));

        // pysicks
        app.insert_resource(ClearColor(Color::srgb(0., 4. / 256., 73. / 256.)))
            .insert_resource(Gravity(Vector::NEG_Y * GRAVITY_ACCELERATION));

        #[cfg(feature = "dev")]
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;
