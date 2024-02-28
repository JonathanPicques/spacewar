use bevy::prelude::*;
use rapier2d::control::{CharacterCollision, KinematicCharacterController};

#[derive(Clone, Component)]
pub struct PhysicsCharacterController {
    pub velocity: Vec2,
    //
    pub(crate) on_floor: bool,
    pub(crate) collisions: Vec<CharacterCollision>,
    pub(crate) rapier_controller: KinematicCharacterController,
}

impl Default for PhysicsCharacterController {
    fn default() -> Self {
        Self {
            velocity: default(),
            on_floor: default(),
            collisions: default(),
            rapier_controller: KinematicCharacterController { slide: true, autostep: None, ..default() },
        }
    }
}

impl PhysicsCharacterController {
    pub fn is_on_floor(&self) -> bool {
        self.on_floor
    }
}
