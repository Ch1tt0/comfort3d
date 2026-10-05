use bevy::app::{App, Plugin};

mod camera;
mod physics;

pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(physics::PhysicsDevPlugin);
        app.add_plugins(camera::CameraDevPlugin);
    }
}
