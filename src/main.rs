mod components;
mod playground;
mod systems;
mod utils;

use bevy::{
    prelude::*,
};

use bevy_inspector_egui::WorldInspectorPlugin;

use utils::{camera, fps};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 800.0,
            height: 800.0,
            title: "Sim Game".to_string(),
            ..Default::default()
        },
        ..default()
    }))
    .add_plugin(fps::ScreenDiagsPlugin)
    .add_plugin(camera::PanOrbitCameraPlugin)
    .add_startup_system(playground::crossing_uv_spheres::setup);
    app.run();
}
