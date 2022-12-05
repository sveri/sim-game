use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use bevy_inspector_egui::WorldInspectorPlugin;

use smooth_bevy_cameras::{LookTransform, LookTransformPlugin, controllers::orbit::{OrbitCameraPlugin}};
#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("systems/src/lib.rs");
}

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(systems::setup)
        .add_plugin(OrbitCameraPlugin::default())
        // .add_system(move_camera_system)
            // .add_system(greet_galaxy)
            ;
    }
}

fn move_camera_system(mut cameras: Query<&mut LookTransform>) {
    // Later, another system will update the `Transform` and apply smoothing automatically.
    for mut c in cameras.iter_mut() {
        c.target += Vec3::new(1.0, 1.0, 1.0);
    }
}

fn main() {
    let mut app = App::new();
    app //.add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
                title: "Sim Game".to_string(),
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LookTransformPlugin)
        .add_plugin(TestPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default());

    // .add_startup_system(systems::setup)
    // .add_system_set(SystemSet::new().with_system(greet_galaxy))
    // .add_system(bevy::window::close_on_esc);
    // .with_system(player_movement_system)
    // .with_system(player_shooting_system)
    // .with_system(bullet_movement_system)
    // .with_system(bullet_hit_system)
    // .with_system(spawn_other_ships)
    // .with_system(move_other_ships),
    // )

    app.run();
}
