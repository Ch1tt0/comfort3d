use bevy::app::{App, Plugin};

use bevy::{prelude::*, window::Window};

use crate::data::GameInfo;

pub struct CCCWindowPlugin;

impl Plugin for CCCWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WindowPlugin {
            primary_window: Some(Window {
                title: GameInfo::default().get_game_name(),
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
