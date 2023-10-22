use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_controller);
    }
}

// Player movement controller
fn player_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            transform.translation += transform.rotation * Vec3::Z * player.speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            transform.translation -= transform.rotation * Vec3::Z * player.speed;
        }
        if keyboard_input.pressed(KeyCode::A) {
            transform.translation -= transform.rotation * Vec3::X * player.speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            transform.translation += transform.rotation * Vec3::X * player.speed;
        }
    }
}