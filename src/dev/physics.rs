use avian3d::diagnostics::PhysicsDiagnosticsPlugin;
use avian3d::prelude::*;
use bevy::app::{App, Plugin};

pub struct PhysicsDevPlugin;

impl Plugin for PhysicsDevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PhysicsDiagnosticsPlugin,
            PhysicsDiagnosticsUiPlugin,
            PhysicsDebugPlugin,
        ));
    }
}
