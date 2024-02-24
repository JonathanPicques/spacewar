use std::hash::{Hash, Hasher};

use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Distance {
    Relative(f32),
    Absolute(f32),
}

#[derive(Copy, Clone, Debug, Component)]
pub struct PlayerController {
    pub up: Vec2,
    pub autostep: Option<Distance>,
    pub snap_to_ground: Option<Distance>,
    pub min_slope_slide_angle: f32,
    pub max_slope_slide_angle: f32,
    //
    pub velocity: Vec2,
    //
    on_wall: bool,
    on_floor: bool,
    on_ceiling: bool,
}

impl PlayerController {
    pub fn is_on_wall(&self) -> bool {
        self.on_wall
    }

    pub fn is_on_floor(&self) -> bool {
        self.on_floor
    }

    pub fn is_on_ceiling(&self) -> bool {
        self.on_ceiling
    }
}

impl Hash for PlayerController {
    fn hash<H: Hasher>(&self, state: &mut H) {
        assert!(
            self.velocity.is_finite(),
            "Hashing is not stable for NaN f32 values."
        );

        self.velocity.x.to_bits().hash(state);
        self.velocity.y.to_bits().hash(state);
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            up: Vec2::Y,
            autostep: None,
            snap_to_ground: None,
            min_slope_slide_angle: 5.0_f32.to_radians(),
            max_slope_slide_angle: 45.0_f32.to_radians(),
            //
            velocity: Vec2::ZERO,
            //
            on_wall: false,
            on_floor: false,
            on_ceiling: false,
        }
    }
}

pub fn player_controller_system(mut query: Query<(&mut Transform, &mut PlayerController)>) {
    for (mut transform, mut player_controller) in query.iter_mut() {
        player_controller.on_wall = false;
        player_controller.on_floor = false;
        player_controller.on_ceiling = false;
        transform.translation += player_controller.velocity.extend(0.0);
        if transform.translation.y <= -200.0 {
            transform.translation.y = -200.0;
            player_controller.on_floor = true;
        }
    }
}
