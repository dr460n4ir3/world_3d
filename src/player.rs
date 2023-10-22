use bevy::prelude::*;
//use bevy::input::gamepad::{Gamepad, GamepadAxis, GamepadButton};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_controller);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::ORANGE_RED.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        Speed(5.0),
    );

    commands.spawn(player);
}

fn player_controller(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    // gamepad_buttons: Res<Input<GamepadButton>>,
    // gamepad_axis: Res<Axis<GamepadAxis>>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("Error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        // // Gamepad input (left stick)
        // // Correct the gamepad access
        // let l_stick_x = gamepad_axes.get(Gamepad(0)).map_or(0.0, |axis| axis[GamepadAxis::LeftStickX]);
        // let l_stick_y = gamepad_axes.get(Gamepad(0)).map_or(0.0, |axis| axis[GamepadAxis::LeftStickY]);

        // direction += cam.forward() * -l_stick_y;
        // direction += cam.right() * l_stick_x;

        // // For there are other buttons on the gamepad to handle for movement,
        // // use the `gamepad_buttons.pressed()` method in a similar manner to keyboard inputs.

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;
    }
}
