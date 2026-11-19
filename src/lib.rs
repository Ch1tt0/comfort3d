use avian3d::prelude::*;
use bevy::prelude::*;

use crate::character::plugin::CharacterControllerPlugin;

mod character;
mod startup;

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

        app.add_systems(Startup, startup::setup);
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
