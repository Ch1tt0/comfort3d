use bevy::prelude::*;

pub mod spawn;

pub mod helpers;
use helpers::*;

pub mod components;
use components::*;

pub mod functions;
use functions::*;

/// A plugin that implements a basic platformer kinematic character controller using move-and-slide,
/// with support for ground detection and configurable movement settings.
pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MovementAction>();

        // Collect input in `PreUpdate` so that it's processed before the physics update.
        app.add_systems(PreUpdate, (keyboard_input, gamepad_input).chain());

        // Run movement logic in `FixedUpdate` to ensure consistent behavior regardless of frame rate.
        app.add_systems(
            FixedUpdate,
            (
                update_grounded,
                apply_gravity,
                movement,
                apply_movement_damping,
                move_and_slide,
                apply_forces_to_dynamic_bodies,
            )
                .chain(),
        );
    }
}
