mod components;
mod entities;
mod plugins;
mod resources;
mod scenes;
mod systems;

use bevy::{
    app::{App, FixedUpdate, Startup, Update}, diagnostic::FrameTimeDiagnosticsPlugin, render::{camera::ClearColor, color::Color}, time::{Fixed, Time}, window::Window, DefaultPlugins
};
use bevy::utils::Duration;

use entities::diagnostics::{setup_fps_counter, setup_zoom_viewer};
use scenes::main_scene::setup_3d_scene;
use scenes::trigger_viz::setup_2d_scene;
use systems::{
    damped_spring::damped_spring_system,
    diagnostics::fps_text_update_system,
    graphic::{
        move_circle_x_system, 
        move_circle_y_system, 
        update_position_history,
        draw_spring_lines
    },
};

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(100)))
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, (setup_fps_counter, setup_2d_scene))
        .add_systems(FixedUpdate, update_position_history)
        .add_systems(
            Update,
            (
                fps_text_update_system,
                move_circle_y_system,
                move_circle_x_system,
                damped_spring_system,
                draw_spring_lines
            ),
        )        // .add_systems(Startup, (setup_fps_counter, setup_zoom_viewer, setup_3d_scene))
        // .add_systems(Update, (fps_text_update_system))
        .run();
}
