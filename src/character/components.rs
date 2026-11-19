use core::f32::consts::PI;

use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

/// A [`Message`] written for a movement input action.
#[derive(Message)]
pub enum MovementAction {
    Move(Vec2),
    Jump,
}

/// A marker component indicating that an entity is using a character controller.
///
/// This also requires the entity to have a `CustomPositionIntegration` component, which is used
/// to prevent Avian from automatically applying the character's velocity to its position,
/// since the character controller will handle movement manually using move-and-slide.
#[derive(Component)]
#[require(RigidBody::Kinematic, CustomPositionIntegration)]
pub struct CharacterController;

/// Component for configuring movement settings for a character controller.
#[derive(Component)]
pub struct CharacterMovementSettings {
    /// The acceleration used for character movement.
    pub acceleration: f32,
    /// The damping coefficient used for slowing down movement.
    pub damping: f32,
    /// The strength of a jump.
    pub jump_impulse: f32,
    /// The gravitational acceleration used for the character.
    pub gravity: Vec3,
    /// The maximum speed that gravity can accelerate the character to.
    /// This prevents the character from accelerating indefinitely while falling.
    pub terminal_velocity: f32,
}

impl Default for CharacterMovementSettings {
    fn default() -> Self {
        Self {
            acceleration: 50.0,
            damping: 10.0,
            jump_impulse: 7.0,
            gravity: Vec3::new(0.0, -9.81 * 2.0, 0.0),
            terminal_velocity: 50.0,
        }
    }
}

/// Component for configuring ground detection for a character controller.
#[derive(Component)]
pub struct GroundDetection {
    /// The maximum angle (in radians) where a surface is considered ground/ceiling
    /// relative to the up-direction. Outside of this angle, surfaces are considered walls.
    ///
    /// **Default**: 30 degrees (π / 6 radians)
    pub max_angle: f32,
    /// The maximum distance for ground detection.
    pub max_distance: f32,
    /// The shape cast collider used for ground detection.
    pub cast_shape: Option<Collider>,
}

impl Default for GroundDetection {
    fn default() -> Self {
        Self {
            max_angle: PI / 6.0,
            max_distance: 0.2,
            cast_shape: None,
        }
    }
}

/// A marker component indicating that an entity is on a surface that is considered
/// ground, meaning the steepness is less than [`GroundDetection::max_angle`].
///
/// Characters that are grounded can jump, and do not slide down slopes.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// A component containing information about the current collisions for a character controller.
///
/// This is used to apply forces to dynamic rigid bodies hit by the character.
#[derive(Component, Default, Deref)]
pub struct CharacterCollisions(pub(crate) Vec<CharacterCollision>);

/// Information about a collision between a character controller and another collider.
pub struct CharacterCollision {
    /// The collider that was hit by the character.
    pub collider: Entity,
    /// The point of contact in world space.
    pub point: RVec3,
    /// The normal of the contact surface, pointing away from the character.
    pub normal: Dir3,
    /// The velocity of the character at the point of contact.
    pub character_velocity: Vec3,
}
