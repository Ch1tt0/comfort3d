use avian3d::prelude::*;
use bevy::prelude::*;

use crate::character::CharacterControllerPlugin;

mod character;
use character::spawn::spawn_player;

mod window;
use window::WindowSettingsPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.build().disable::<WindowPlugin>()); // Bevy Default
        app.add_plugins(PhysicsPlugins::default()); // Avian3D
        app.add_plugins(PhysicsDebugPlugin); // Avian3D Debug

        app.add_plugins(WindowSettingsPlugin);

        app.add_plugins(CharacterControllerPlugin);

        app.add_systems(Startup, (setup, spawn_player));
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    fn _setup() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, crate::AppPlugin));
        app.update();
        app
    }

    // Tests here...
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // Environment
    commands.spawn((
        WorldAssetRoot(assets.load("DevMap.glb#Scene0")),
        Transform::from_rotation(Quat::from_rotation_y(-core::f32::consts::PI * 0.5)),
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexHullFromMesh),
        RigidBody::Static,
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0),
    ));
}
