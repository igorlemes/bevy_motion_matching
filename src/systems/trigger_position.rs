use bevy::{ecs::{event::EventReader, query::With, system::{Query, Res}}, input::{gamepad::{Gamepad, GamepadButton, GamepadButtonChangedEvent, GamepadButtonType}, ButtonInput}, log::info, transform::components::Transform, utils::tracing::Event, window::{PrimaryWindow, WindowResized}};
use bevy::window::Window;

use crate::components::spring::ControllerTrigger;

pub fn move_circle_y(
    mut button_event: EventReader<GamepadButtonChangedEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<ControllerTrigger>>,
) {
    let trigger = button_event.read().find(|e| 
        e.button_type == GamepadButtonType::LeftTrigger2 || 
        e.button_type == GamepadButtonType::RightTrigger2
    );
    if let Some(trigger) = trigger {
        for mut transform in &mut query {
            // Get the screen width and height
            if let Some(window) = window.iter().next() {
                let height = window.height();
                // Put the circle in the value percentage of the screen height
                if trigger.button_type == GamepadButtonType::RightTrigger2 {
                    transform.translation.y = trigger.value * height * 0.45;
                    info!("RightTrigger2 Pressed {}", trigger.value);
                    info!("Moved: {}", transform.translation.x);
                } else if trigger.button_type == GamepadButtonType::LeftTrigger2 {
                    transform.translation.y = - trigger.value * height * 0.45;
                    info!("LeftTrigger2 Pressed {}", trigger.value);
                    info!("Moved: {}", transform.translation.x);
                }
            };
            
        }
    }   
}

pub fn move_circle_x(
    mut window_resize_event: EventReader<WindowResized>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<ControllerTrigger>>,
) {
    // Is there a window resize event?
    if let Some(_event) = window_resize_event.read().last() {
        for mut transform in &mut query {
            // Get the screen width and height
            if let Some(window) = window.iter().next() {
                let width = window.width();
                // Put the circle in the value percentage of the screen height
                transform.translation.x = width * 0.45;
                info!("Moved: {}", transform.translation.x);
            };

        }
    }
}