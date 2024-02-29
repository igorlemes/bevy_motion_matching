use bevy::ecs::component::Component;
use std::collections::VecDeque;
use bevy::math::Vec3;

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


#[derive(Component)]
pub struct PositionHistory {
    pub list: VecDeque<Vec3>,
    pub max_values: usize
}

impl PositionHistory {
    pub fn new(max_values: usize) -> Self {
        PositionHistory {
            list: VecDeque::from(vec![Vec3::default(); max_values]),
            max_values,
        }
    }
    pub fn push(&mut self, value: Vec3) {
        self.list.push_front(value);
        if self.list.len() > self.max_values {
            self.list.pop_back();
        }
    }
}

#[derive(Component)]
pub struct SpringLines;
