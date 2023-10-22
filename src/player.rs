use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

// FOURTH
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
        material: materials.add(Color::ORANGE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(player);
}
// pub fn create_player(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Lets create our basic Player object
//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Capsule {
//             radius: 1.,
//             rings: 10,
//             depth: 1.,
//             latitudes: 4,
//             longitudes: 4,
//             ..Default::default()
//         })),
//         material: materials.add(Color::rgb(0., 0.8, 0.8).into()),
//         transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
//         ..Default::default()
//     });

// }
