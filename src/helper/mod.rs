use bevy::prelude::*;

pub mod camera;
mod ldtk;


pub struct Vec2Step {
    start: Vec2,
    end: Vec2,
    step: f32,
}

impl Vec2Step {
    pub fn new(start: Vec2, end: Vec2, step: f32) -> Self {
        Vec2Step { start, end, step }
    }
}

impl Iterator for Vec2Step {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.x < self.end.x && self.start.y < self.end.y {
            let result = self.start;
            self.start.x += self.step;
            self.start.y += self.step;
            Some(result)
        } else {
            None
        }
    }
}

pub struct FloatStep {
    start: f32,
    end: f32,
    step: f32,
}

impl FloatStep {
    pub fn new(start: f32, end: f32, step: f32) -> Self {
        FloatStep { start, end, step }
    }
}

impl Iterator for FloatStep {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let result = self.start;
            self.start += self.step;
            Some(result)
        } else {
            None
        }
    }
}