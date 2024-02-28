use std::collections::VecDeque;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ControllerTrigger;

#[derive(Component)]
pub struct Spring {
    pub list: VecDeque<f32>,
    pub max_values: usize,
}

impl Spring {
    pub fn new(max_values: usize) -> Self {
        Spring {
            list: VecDeque::new(),
            max_values,
        }
    }
    pub fn push(&mut self, value: f32) {
        self.list.push_front(value);
        if self.list.len() > self.max_values {
            self.list.pop_back();
        }
    }

    pub fn average(&self) -> f32 {
        let sum: f32 = self.list.iter().sum();
        sum / self.list.len() as f32
    }
}