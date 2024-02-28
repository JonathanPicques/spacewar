use bevy::prelude::*;

#[derive(Clone, Default, Component)]
pub struct PhysicsCharacterController {
    pub velocity: Vec2,
    //
    pub(crate) on_floor: bool,
}

impl PhysicsCharacterController {
    pub fn is_on_floor(&self) -> bool {
        self.on_floor
    }
}
