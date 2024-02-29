use bevy::log::info;
use bevy::window::Window;
use bevy::{
    ecs::{event::EventReader, query::With, system::Query},
    input::gamepad::{GamepadButtonChangedEvent, GamepadButtonType},
    transform::components::Transform,
    window::{PrimaryWindow, WindowResized},
};

use crate::components::spring::{ControllerTrigger, Spring};

pub fn damped_spring_system(
    mut button_event: EventReader<GamepadButtonChangedEvent>,
    mut query: Query<&mut Spring, With<ControllerTrigger>>,
) {
    let trigger = button_event.read().find(|e| {
        e.button_type == GamepadButtonType::LeftTrigger2
            || e.button_type == GamepadButtonType::RightTrigger2
    });
    if let Some(trigger) = trigger {
        for mut value in query.iter_mut() {
            if trigger.button_type == GamepadButtonType::RightTrigger2 {
                value.list.push_front(trigger.value);
            } else if trigger.button_type == GamepadButtonType::LeftTrigger2 {
                value.list.push_front(-trigger.value);
            }

            if value.list.len() > value.max_values {
                value.list.pop_back();
            }
            info!("Spring: {:?}", value.list);
        }
    }
}
