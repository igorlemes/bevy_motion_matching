use crate::components::camera::CameraController;
use bevy::ecs::event::EventReader;
use bevy::ecs::query::With;
use bevy::ecs::system::{Local, Query, Res};
use bevy::input::gamepad::{
    Gamepad, GamepadButton, GamepadButtonChangedEvent, GamepadButtonType, GamepadEvent,
};
use bevy::input::{Axis, ButtonInput};
use bevy::transform::components::Transform;
// Logging
use bevy::log::info;

use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::render::camera::{Camera, OrthographicProjection, Projection, ScalingMode};
use bevy::utils::info;
use bevy_third_person_camera::camera::GamepadResource;

pub fn zoom_camera(
    button_event: GamepadButtonChangedEvent,
    mut query: Query<&mut Projection, With<Camera>>,
) {

    if button_event.button_type == GamepadButtonType::LeftTrigger2 {
        info!("LeftTrigger2 Pressed {}", button_event.value);
        // Update the scaling_mode: ScalingMode::WindowSize() according to the value of the event
        if let Ok(mut projection) = query.get_single_mut() {
            if let Projection::Orthographic(projection) = &mut *projection {
                while button_event.value > 0.0 {
                    let zoom_scalar = 1.0 - 0.05 * button_event.value;
                    projection.scale *= zoom_scalar;
                    projection.scale = projection.scale.max(1.0).min(5.0);
                    info!("Zoomed: {}", projection.scale);
                }
            }
        }
    } else if button_event.button_type == GamepadButtonType::RightTrigger2 {
        info!("RightTrigger2 Pressed {}", button_event.value);
        // Update the scaling_mode: ScalingMode::WindowSize() according to the value of the event
        if let Ok(mut projection) = query.get_single_mut() {
            if let Projection::Orthographic(projection) = &mut *projection {
                while button_event.value > 0.0 {
                    let zoom_scalar = 1.0 + 0.05 * button_event.value;
                    projection.scale *= zoom_scalar;
                    projection.scale = projection.scale.max(1.0).min(5.0);
                    info!("Zoomed: {}", projection.scale);
                }
            }
        }
    }
}
