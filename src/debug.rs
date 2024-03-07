use crate::core::anim::SpriteSheetAnimator;
use crate::core::clock::{Clock, TimeToLive};

use crate::core::physics::body::{PhysicsBody, PhysicsBodyHandle, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::physics::collider::{PhysicsCollider, PhysicsColliderHandle, PhysicsColliderOptions};
use crate::core::physics::controller::PhysicsCharacterController;
use crate::core::physics::Physics;
use crate::spacewar::game::player::Player;
use crate::spacewar::game::Game;

macro_rules! impl_debug_hash {
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut hasher = std::hash::DefaultHasher::default();
                std::hash::Hash::hash(self, &mut hasher);
                f.write_fmt(format_args!(
                    "{}",
                    std::hash::Hasher::finish(&hasher)
                ))
            }
        }
    };
}

// Core

impl_debug_hash!(Clock);
impl_debug_hash!(TimeToLive);
impl_debug_hash!(SpriteSheetAnimator);
impl_debug_hash!(Physics);
impl_debug_hash!(PhysicsBody);
impl_debug_hash!(PhysicsBodyHandle);
impl_debug_hash!(PhysicsBodyOptions);
impl_debug_hash!(PhysicsBodyVelocity);
impl_debug_hash!(PhysicsCollider);
impl_debug_hash!(PhysicsColliderHandle);
impl_debug_hash!(PhysicsColliderOptions);
impl_debug_hash!(PhysicsCharacterController);

// Game

impl_debug_hash!(Game);
impl_debug_hash!(Player);
