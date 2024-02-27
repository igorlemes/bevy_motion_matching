mod components;
mod systems;
mod plugins;
mod resources;
mod entities;
mod scenes;

use bevy::{
    app::{App, Startup, Update},
    diagnostic::FrameTimeDiagnosticsPlugin,
    render::{
        camera::ClearColor,
        color::Color,
    },
    DefaultPlugins,
};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use systems::fps::{fps_text_update_system, setup_fps_counter};
use scenes::main_scene::setup_3d_scene;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(ThirdPersonCameraPlugin)
        .add_systems(Startup, 
            (setup_fps_counter, setup_3d_scene)
        )
        .add_systems(Update, fps_text_update_system)
        .run();
}
