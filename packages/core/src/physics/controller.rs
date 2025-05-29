use bevy::prelude::*;
use derivative::Derivative;
use rapier2d::control::{CharacterCollision, EffectiveCharacterMovement, KinematicCharacterController};

use crate::utilities::maths::*;

#[derive(Hash, Copy, Clone, Default)]
pub struct Wall {
    pub left: bool,
    pub right: bool,
}

#[derive(Hash, Copy, Clone, Default)]
pub struct Floor {
    pub on: bool,
}

#[derive(Hash, Copy, Clone, Default)]
pub struct Ceiling {
    pub on: bool,
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
    pub wall: Wall,
    pub floor: Floor,
    pub ceiling: Ceiling,
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
            ceiling: default(),
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

    pub fn is_on_ceiling(&self) -> bool {
        self.ceiling.on
    }

    pub(crate) fn update_with_movement(&mut self, movement: EffectiveCharacterMovement, collisions: Vec<CharacterCollision>) {
        self.wall.left = false;
        self.wall.right = false;
        self.floor.on = movement.grounded;
        self.ceiling.on = false;

        for collision in collisions.iter() {
            match collision.hit.status {
                rapier2d::parry::query::ShapeCastStatus::Failed
                | rapier2d::parry::query::ShapeCastStatus::Converged
                | rapier2d::parry::query::ShapeCastStatus::OutOfIterations => {
                    let normal = collision.hit.normal1.to_bevy();
                    let up_angle = normal.angle_to(self.up);

                    if self.up.dot(normal) < 0.0 {
                        self.ceiling.on = true;
                    } else if abs(up_angle) > self.rapier_controller.max_slope_climb_angle {
                        self.wall.left = self.right.dot(normal) > 0.0;
                        self.wall.right = !self.wall.left;
                    }
                }
                _ => (),
            }
        }
        // println!(
        //     "left: {} right: {}, floor: {}, ceiling: {}",
        //     self.wall.left, self.wall.right, self.floor.on, self.ceiling.on
        // );
    }
}
