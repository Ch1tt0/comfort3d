use bevy::app::{App, Plugin};

use bevy::{prelude::*, window::Window};
pub struct WindowSettingsPlugin;

impl Plugin for WindowSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                // 1. Fits canvas to parent (<body>).
                // 2. Prevents browser hotkeys from escaping.
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        });
    }
}
