use bevy::app::{App, Plugin};
use bevy::camera_controller::free_camera::FreeCameraPlugin;

pub struct CameraDevPlugin;

impl Plugin for CameraDevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FreeCameraPlugin);
    }
}
