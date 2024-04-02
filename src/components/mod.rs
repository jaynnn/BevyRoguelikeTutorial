use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Movement {
    pub dir: Option<Vec2>,
}

