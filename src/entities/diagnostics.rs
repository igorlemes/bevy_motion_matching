use crate::components::diagnostics::FpsText;
use crate::components::diagnostics::ZoomText;
use bevy::{
    ecs::system::Commands,
    math::Vec3,
    prelude::BackgroundColor,
    render::color::Color,
    text::{Text, TextSection, TextStyle},
    transform::components::Transform,
    ui::node_bundles::TextBundle,
};

pub fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    // commands.spawn(Camera2dBundle::default());
    // create our text
    commands.spawn((
        FpsText,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: "FPS: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::GREEN,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..Default::default()
                    },
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::GREEN,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..Default::default()
                    },
                },
            ]),
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
            transform: Transform {
                translation: Vec3::new(5.0, 5.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

pub fn setup_zoom_viewer(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    // commands.spawn(Camera2dBundle::default());
    // create our text
    commands.spawn((
        ZoomText,
        TextBundle {
            // use two sections, so it is easy to update just the number
            text: Text::from_sections([
                TextSection {
                    value: "Zoom: ".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::GREEN,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..Default::default()
                    },
                },
                TextSection {
                    value: " N/A".into(),
                    style: TextStyle {
                        font_size: 16.0,
                        color: Color::GREEN,
                        // if you want to use your game's font asset,
                        // uncomment this and provide the handle:
                        // font: my_font_handle
                        ..Default::default()
                    },
                },
            ]),
            background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
            transform: Transform {
                translation: Vec3::new(10.0, 10.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
