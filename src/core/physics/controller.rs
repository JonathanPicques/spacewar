use bevy::prelude::*;
use rapier2d::control::KinematicCharacterController;

#[derive(Clone, Default, Component)]
pub struct PhysicsCharacterController {
    pub velocity: Vec2,
    //
    pub(crate) on_floor: bool,
    pub(crate) rapier_controller: KinematicCharacterController,
}

impl PhysicsCharacterController {
    pub fn is_on_floor(&self) -> bool {
        self.on_floor
    }
}
