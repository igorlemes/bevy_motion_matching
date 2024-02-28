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

use scenes::main_scene::setup_3d_scene;
use scenes::trigger_viz::setup_2d_scene;
use entities::diagnostics::{
    setup_fps_counter,
    setup_zoom_viewer,
};
use systems::{
    diagnostics::fps_text_update_system,
    trigger_position::{
        move_circle_y,
        move_circle_x,
    },
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (setup_fps_counter, setup_2d_scene))
        .add_systems(Update, (
            fps_text_update_system, 
            move_circle_y, 
            move_circle_x
        ))
        
        // .add_systems(Startup, (setup_fps_counter, setup_zoom_viewer, setup_3d_scene))
        // .add_systems(Update, (fps_text_update_system))
        .run();
}
