use bevy::{prelude::*, DefaultPlugins};

mod camera;
mod player;
mod world;
//mod controller;

use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
//use controller::ControllerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            CameraPlugin,
            WorldPlugin, /*ControllerPlugin*/
        ))
        .run();
}

// Working on player controller system
