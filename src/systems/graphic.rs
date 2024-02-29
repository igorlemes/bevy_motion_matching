use bevy::ecs::query::Without;
use bevy::gizmos::gizmos::Gizmos;
use bevy::log::info;
use bevy::math::{Vec2, Vec3Swizzles};
use bevy::render::color::Color;
use bevy::window::Window;
use bevy::{
    ecs::{event::EventReader, query::With, system::Query},
    input::gamepad::{GamepadButtonChangedEvent, GamepadButtonType},
    transform::components::Transform,
    window::{PrimaryWindow, WindowResized},
};


use crate::components::spring::{
    ControllerTrigger, 
    Spring, 
    PositionHistory
};

pub fn move_circle_y_system(
    mut button_event: EventReader<GamepadButtonChangedEvent>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Spring), With<ControllerTrigger>>,
) {
    let trigger = button_event.read().find(|e| {
        e.button_type == GamepadButtonType::LeftTrigger2 || 
        e.button_type == GamepadButtonType::RightTrigger2
    });
    if let Some(trigger) = trigger {
        for (mut transform, mut _spring) in &mut query {
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

pub fn update_position_history(
    mut query: Query<(&mut PositionHistory, &Transform), With<ControllerTrigger>>,
    mut spring_query: Query<(&mut PositionHistory, &mut Transform), Without<ControllerTrigger>>,
) {
    for (mut position_history, transform) in query.iter_mut() {
        position_history.list.push_front(transform.translation);
        if position_history.list.len() > position_history.max_values {
            position_history.list.pop_back();
        }
    }
    for (mut position_history, spring_transform) in spring_query.iter_mut() {   
        position_history.list.push_front(spring_transform.translation);
        if position_history.list.len() > position_history.max_values {
            position_history.list.pop_back();
        }
    }
    if let Some((position_history, _c_pos)) = query.iter().next() {
        for (index, (_, mut transform)) in spring_query.iter_mut().enumerate(){
            if index > 0 {
                transform.translation.y = position_history.list[index - 1].y;
                transform.translation.x = position_history.list[index - 1].x - (index) as f32 * 20.0;
            }            
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

pub fn draw_spring_lines(
    mut gizmos: Gizmos,
    mut query: Query<&PositionHistory, With<ControllerTrigger>>,
) {

    for position_history  in query.iter_mut() {
        for i in 0..position_history.list.len() - 1 {
            let start: Vec2 = position_history.list[i].xy() - Vec2::new( (i+1) as f32 * 20.0, 0.0);
            let end: Vec2 = position_history.list[i + 1].xy() - Vec2::new( (i+2) as f32 * 20.0, 0.0);
            info!("Start: {:?}, End: {:?}", start, end);
            let color = Color::rgb(0.0, 0.0, 1.0);
            gizmos.line_2d(start, end, color);
        }
    }
}