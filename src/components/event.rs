use bevy::prelude::*;

/// 碰撞事件，当球发生碰撞时触发
#[derive(Event)]
pub struct BallCollided;