mod components;
mod entities;
mod plugins;
mod resources;
mod scenes;
mod systems;

use bevy::{
    app::{App, Startup, Update},
    diagnostic::FrameTimeDiagnosticsPlugin,
    render::{camera::ClearColor, color::Color},
    DefaultPlugins,
};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use scenes::main_scene::setup_3d_scene;
use systems::fps::{fps_text_update_system, setup_fps_counter};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, (setup_fps_counter, setup_3d_scene))
        .add_systems(Update, fps_text_update_system)
        .run();
}
