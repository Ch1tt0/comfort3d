use avian3d::prelude::*;
use bevy::prelude::*;

use super::components::*;

/// The decomposition of a velocity vector into parts relative to a collision normal and an up-direction.
///
/// This is used for determining how much of the velocity is contributing to climbing, slipping, and unconstrained movement.
#[derive(Debug)]
pub struct VelocityDecomposition {
    /// The part of the velocity that is directly against the collision normal.
    pub normal_part: Vec3,
    /// The part of the velocity that is tangent to the collision surface and perpendicular to the up-direction.
    pub horizontal_tangent: Vec3,
    /// The part of the velocity that is tangent to the collision surface and parallel to the up-direction.
    pub vertical_tangent: Vec3,
}

/// Decomposes a velocity vector into parts relative to a collision `normal` and an `up` direction.
pub fn decompose_hit_velocity(velocity: Vec3, normal: Dir3, up: Vec3) -> VelocityDecomposition {
    let normal_part = normal * normal.dot(velocity);
    let tangent_part = velocity - normal_part;

    let horizontal_tangent_dir = normal.cross(up).normalize_or_zero();
    let horizontal_tangent = tangent_part.dot(horizontal_tangent_dir) * horizontal_tangent_dir;
    let vertical_tangent = tangent_part - horizontal_tangent;

    VelocityDecomposition {
        normal_part,
        horizontal_tangent,
        vertical_tangent,
    }
}

/// Splits a vector into horizontal and vertical components relative to a given `up` direction.
pub fn split_into_components(v: Vec3, up: Vec3) -> [Vec3; 2] {
    let vertical_component = up * v.dot(up);
    let horizontal_component = v - vertical_component;
    [horizontal_component, vertical_component]
}

/// Applies forces to dynamic rigid bodies hit by character controllers based on their collisions.
pub fn apply_forces_to_dynamic_bodies(
    characters: Query<(&ComputedMass, &CharacterCollisions)>,
    colliders: Query<&ColliderOf>,
    mut rigid_bodies: Query<(&RigidBody, Forces)>,
) {
    for (mass, collisions) in &characters {
        let mass = mass.value();
        for collision in &collisions.0 {
            let Ok(collider_of) = colliders.get(collision.collider) else {
                continue;
            };
            let Ok((rigid_body, mut forces)) = rigid_bodies.get_mut(collider_of.body) else {
                continue;
            };
            if !rigid_body.is_dynamic() {
                continue;
            }

            let touch_dir = -collision.normal;
            let relative_velocity = collision.character_velocity - forces.linear_velocity();
            let touch_velocity = touch_dir.dot(relative_velocity) * touch_dir;
            let impulse = touch_velocity * mass;

            forces.apply_linear_impulse_at_point(impulse, collision.point);
        }
    }
}
