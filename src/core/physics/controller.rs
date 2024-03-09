use bevy::prelude::*;
use derivative::Derivative;
use rapier2d::control::{CharacterCollision, EffectiveCharacterMovement, KinematicCharacterController};

use crate::core::utilities::maths::*;

#[derive(Copy, Clone, Default)]
pub struct Wall {
    pub left: bool,
    pub right: bool,
}

#[derive(Copy, Clone, Default)]
pub struct Floor {
    pub on: bool,
    pub angle: f32,
}

#[derive(Copy, Clone, Component, Derivative)]
#[derivative(Hash)]
pub struct PhysicsCharacterController {
    #[derivative(Hash = "ignore")]
    pub up: Vec2,
    #[derivative(Hash = "ignore")]
    pub right: Vec2,
    #[derivative(Hash = "ignore")]
    pub velocity: Vec2,
    //
    #[derivative(Hash = "ignore")]
    pub wall: Wall,
    #[derivative(Hash = "ignore")]
    pub floor: Floor,
    //
    #[derivative(Hash = "ignore")]
    pub(crate) rapier_controller: KinematicCharacterController,
}

impl Default for PhysicsCharacterController {
    fn default() -> Self {
        Self {
            up: Vec2::Y,
            right: Vec2::X,
            velocity: default(),
            //
            wall: default(),
            floor: default(),
            //
            rapier_controller: KinematicCharacterController { slide: true, autostep: None, ..default() },
        }
    }
}

impl PhysicsCharacterController {
    pub fn is_on_wall(&self) -> bool {
        self.wall.left || self.wall.right
    }

    pub fn is_on_floor(&self) -> bool {
        self.floor.on
    }

    pub(crate) fn update_with_movement(&mut self, movement: EffectiveCharacterMovement, collisions: Vec<CharacterCollision>) {
        self.wall.left = false;
        self.wall.right = false;
        self.floor.on = movement.grounded;
        self.floor.angle = 0.0;

        for collision in collisions.iter() {
            match collision.toi.status {
                rapier2d::parry::query::TOIStatus::Failed
                | rapier2d::parry::query::TOIStatus::Converged
                | rapier2d::parry::query::TOIStatus::OutOfIterations => {
                    let normal = collision.toi.normal1.to_bevy();
                    let up_angle = normal.angle_between(self.up);

                    if abs(up_angle) > self.rapier_controller.max_slope_climb_angle {
                        if self.right.dot(normal) > 0.0 {
                            self.wall.left = true;
                        } else {
                            self.wall.right = true;
                        }
                    } else {
                        self.floor.on = true;
                        self.floor.angle = up_angle;
                    }
                }
                _ => (),
            }
        }
    }
}
