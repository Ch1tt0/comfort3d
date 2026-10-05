// WORK_AROUND: Work around for wasm-bindgen failing on:
// error: failed to find intrinsics to enable `clone_ref` function.
#![allow(unused_imports)]
use avian3d::parry::simba::scalar::SupersetOf;
use web_sys::*;

use bevy::{prelude::*, window::Window};

use comfort3d::AppPlugin;

fn main() {
    let app: &mut App = &mut App::new();

    app.add_plugins(AppPlugin)
        .add_plugins(DefaultPlugins.build().disable::<WindowPlugin>());

    app.run();
}
