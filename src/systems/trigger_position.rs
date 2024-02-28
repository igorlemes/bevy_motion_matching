use bevy::window::Window;
use bevy::{
    ecs::{event::EventReader, query::With, system::Query},
    input::gamepad::{GamepadButtonChangedEvent, GamepadButtonType},
    transform::components::Transform,
    window::{PrimaryWindow, WindowResized},
};

use crate::components::spring::ControllerTrigger;

pub fn move_circle_y_system(
    mut button_event: EventReader<GamepadButtonChangedEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<ControllerTrigger>>,
) {
    let trigger = button_event.read().find(|e| {
        e.button_type == GamepadButtonType::LeftTrigger2
            || e.button_type == GamepadButtonType::RightTrigger2
    });
    if let Some(trigger) = trigger {
        for mut transform in &mut query {
            if let Some(window) = window.iter().next() {
                let height = window.height();
                if trigger.button_type == GamepadButtonType::RightTrigger2 {
                    transform.translation.y = trigger.value * height * 0.45;
                } else if trigger.button_type == GamepadButtonType::LeftTrigger2 {
                    transform.translation.y = -trigger.value * height * 0.45;
                }
            };
        }
    }
}

pub fn move_circle_x_system(
    mut window_resize_event: EventReader<WindowResized>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<ControllerTrigger>>,
) {
    if let Some(_event) = window_resize_event.read().last() {
        for mut transform in &mut query {
            if let Some(window) = window.iter().next() {
                let width = window.width();
                transform.translation.x = width * 0.45;
            };
        }
    }
}
