use std::cmp::Ordering;
use std::time::Duration;

use bevy::prelude::*;

use crate::core::anim::SpriteSheetAnimator;
use crate::core::input::CoreInput;
use crate::core::physics::controller::PhysicsCharacterController;
use crate::core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use crate::core::utilities::maths::{compute_acceleration, compute_deceleration};
use crate::spacewar::game::input::{INPUT_LEFT, INPUT_RIGHT, INPUT_SHOOT, INPUT_UP};
use crate::spacewar::game::player::{Direction, Player, PlayerState};
use crate::spacewar::game::projectile::ProjectileBundle;
use crate::spacewar::GameAssets;

const JUMP_STRENGTH: f32 = 7.5;

const FLOOR_MAX_SPEED: f32 = 2.5;
const FLOOR_ACCELERATION: f32 = 0.4;
const FLOOR_DECELERATION: f32 = 0.35;

const AIRBORNE_MAX_SPEED: f32 = 2.25;
const AIRBORNE_ACCELERATION: f32 = 0.35;
const AIRBORNE_DECELERATION: f32 = 0.3;

const GRAVITY_MAX_SPEED: f32 = -2.5;
const GRAVITY_ACCELERATION: f32 = 0.5;

pub struct PlayerArgs<'a, 'w, 's> {
    pub delta: f32,
    pub input: &'a CoreInput,
    pub assets: &'a GameAssets,
    pub sprite: &'a mut Sprite,
    pub animator: &'a mut SpriteSheetAnimator,
    pub commands: &'a mut Commands<'w, 's>,
    pub controller: &'a mut PhysicsCharacterController,
    pub translation: &'a Vec3,
}

#[allow(clippy::needless_return)]
impl Player {
    pub fn tick(&mut self, mut args: PlayerArgs) {
        self.shoot_clock
            .tick(Duration::from_secs_f32(args.delta));

        match self.state {
            super::PlayerState::None => self.tick_none(&mut args),
            super::PlayerState::Idle => self.tick_idle(&mut args),
            super::PlayerState::Fall => self.tick_fall(&mut args),
            super::PlayerState::Jump => self.tick_jump(&mut args),
            super::PlayerState::Walk => self.tick_walk(&mut args),
        }
    }

    pub fn set_state(&mut self, new_state: PlayerState, args: &mut PlayerArgs) {
        let old_state = self.state;

        self.state = new_state;
        match old_state {
            PlayerState::None => (),
            PlayerState::Idle => self.leave_idle(args),
            PlayerState::Fall => self.leave_fall(args),
            PlayerState::Jump => self.leave_jump(args),
            PlayerState::Walk => self.leave_walk(args),
        };
        match new_state {
            PlayerState::None => (),
            PlayerState::Idle => self.enter_idle(args),
            PlayerState::Fall => self.enter_fall(args),
            PlayerState::Jump => self.enter_jump(args),
            PlayerState::Walk => self.enter_walk(args),
        };
    }

    // States

    fn tick_none(&mut self, args: &mut PlayerArgs) {
        if args.controller.is_on_floor() {
            self.set_state(PlayerState::Idle, args);
            return;
        }
        self.set_state(PlayerState::Fall, args);
        return;
    }

    fn tick_idle(&mut self, args: &mut PlayerArgs) {
        self.projectile(args);
        self.apply_gravity(args);
        self.apply_deceleration(args, FLOOR_DECELERATION);
        self.apply_velocity_direction(args);

        if !args.controller.is_on_floor() {
            self.set_state(PlayerState::Fall, args);
            return;
        }
        if args.input.is_set(INPUT_UP) {
            self.set_state(PlayerState::Jump, args);
            self.apply_jump(args);
            return;
        }
        if self.only_left(args) || self.only_right(args) {
            self.set_state(PlayerState::Walk, args);
            return;
        }
    }

    fn tick_jump(&mut self, args: &mut PlayerArgs) {
        self.projectile(args);
        self.apply_gravity(args);
        self.apply_movement(
            args,
            AIRBORNE_MAX_SPEED,
            AIRBORNE_ACCELERATION,
            AIRBORNE_DECELERATION,
        );
        self.apply_velocity_direction(args);

        if args.controller.is_on_wall() {
            self.apply_wall_bump(args);
        }
        if args.controller.is_on_floor() {
            self.set_state(PlayerState::Idle, args);
            return;
        }
        if args.controller.is_on_ceiling() {
            self.set_state(PlayerState::Fall, args);
            self.apply_ceiling_bump(args);
            return;
        }
        if args.controller.velocity.y < 0.0 {
            self.set_state(PlayerState::Fall, args);
            return;
        }
    }

    fn tick_fall(&mut self, args: &mut PlayerArgs) {
        self.projectile(args);
        self.apply_gravity(args);
        self.apply_movement(
            args,
            AIRBORNE_MAX_SPEED,
            AIRBORNE_ACCELERATION,
            AIRBORNE_DECELERATION,
        );
        self.apply_velocity_direction(args);

        if args.controller.is_on_wall() {
            self.apply_wall_bump(args);
        }
        if args.controller.is_on_floor() {
            self.set_state(PlayerState::Idle, args);
            return;
        }
        if args.controller.is_on_ceiling() {
            self.set_state(PlayerState::Fall, args);
            self.apply_ceiling_bump(args);
            return;
        }
    }

    fn tick_walk(&mut self, args: &mut PlayerArgs) {
        self.projectile(args);
        self.apply_gravity(args);
        self.apply_movement(
            args,
            FLOOR_MAX_SPEED,
            FLOOR_ACCELERATION,
            FLOOR_DECELERATION,
        );
        self.apply_velocity_direction(args);

        if args.controller.is_on_wall() {
            self.apply_wall_bump(args);
        }
        if !args.controller.is_on_floor() {
            self.set_state(PlayerState::Fall, args);
            return;
        }
        if args.input.is_set(INPUT_UP) {
            self.set_state(PlayerState::Jump, args);
            self.apply_jump(args);
            return;
        }
        if args.controller.velocity.x == 0.0 {
            self.set_state(PlayerState::Idle, args);
            return;
        }
    }

    // Transitions

    fn enter_idle(&mut self, args: &mut PlayerArgs) {
        args.animator.animation = args.assets.player_idle_anim.clone();
    }
    fn leave_idle(&mut self, _: &mut PlayerArgs) {}

    fn enter_fall(&mut self, args: &mut PlayerArgs) {
        args.animator.animation = args.assets.player_fall_anim.clone();
    }
    fn leave_fall(&mut self, _: &mut PlayerArgs) {}

    fn enter_jump(&mut self, args: &mut PlayerArgs) {
        args.animator.animation = args.assets.player_jump_anim.clone();
    }
    fn leave_jump(&mut self, _: &mut PlayerArgs) {}

    fn enter_walk(&mut self, args: &mut PlayerArgs) {
        args.animator.animation = args.assets.player_walk_anim.clone();
    }
    fn leave_walk(&mut self, _: &mut PlayerArgs) {}

    // Input helpers

    fn only_left(self, args: &mut PlayerArgs) -> bool {
        args.input.is_set(INPUT_LEFT) && !args.input.is_set(INPUT_RIGHT)
    }

    fn only_right(self, args: &mut PlayerArgs) -> bool {
        args.input.is_set(INPUT_RIGHT) && !args.input.is_set(INPUT_LEFT)
    }

    // Instant helpers

    fn apply_jump(self, args: &mut PlayerArgs) {
        args.controller.velocity.y = JUMP_STRENGTH;
    }

    fn apply_wall_bump(self, args: &mut PlayerArgs) {
        args.controller.velocity.x = 0.0;
    }

    fn apply_ceiling_bump(self, args: &mut PlayerArgs) {
        args.controller.velocity.y = 0.0;
    }

    // Movement helpers

    fn apply_gravity(self, args: &mut PlayerArgs) {
        args.controller.velocity.y = compute_acceleration(
            args.controller.velocity.y,
            GRAVITY_MAX_SPEED,
            GRAVITY_ACCELERATION,
        );
    }

    fn apply_movement(self, args: &mut PlayerArgs, max_speed: f32, acceleration: f32, deceleration: f32) {
        let left = self.only_left(args);
        let right = self.only_right(args);
        let velocity = &mut args.controller.velocity;

        if left {
            velocity.x = compute_acceleration(velocity.x, -max_speed, acceleration);
        } else if right {
            velocity.x = compute_acceleration(velocity.x, max_speed, acceleration);
        } else {
            velocity.x = compute_deceleration(velocity.x, deceleration);
        }
    }

    fn apply_deceleration(self, args: &mut PlayerArgs, deceleration: f32) {
        let velocity = &mut args.controller.velocity;

        velocity.x = compute_deceleration(velocity.x, deceleration);
    }

    fn apply_velocity_direction(&mut self, args: &mut PlayerArgs) {
        self.direction = match 0.0_f32.total_cmp(&args.controller.velocity.x) {
            Ordering::Less => Direction::Right,
            Ordering::Equal => self.direction,
            Ordering::Greater => Direction::Left,
        };
        args.sprite.flip_x = match self.direction {
            Direction::Left => true,
            Direction::Right => false,
        };
    }

    // Miscellaneous helpers

    fn projectile(&mut self, args: &mut PlayerArgs) {
        if args.input.is_set(INPUT_SHOOT) && self.shoot_clock.finished() {
            args.commands
                .spawn_with_rollback(ProjectileBundle::new(
                    self,
                    args.assets,
                    args.translation,
                ));
            self.shoot_clock.reset();
        }
    }
}
