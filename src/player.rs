use bevy::prelude::*;
use bevy::render::mesh::shape::Cube;

pub struct PlayerPlugin;

#[derive(Component)]
struct CharacterController;

const MOVE_SPEED: f32 = 2.5;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, character_movement);
    }
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Load mesh
    let cube_mesh = meshes.add(Mesh::from(Cube { size: 1.0 }));

    // Cube character
    commands
        .spawn(PbrBundle {
            mesh: cube_mesh,
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(CharacterController);
}

fn character_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<CharacterController>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction -= Vec3::new(0.0, 0.0, -1.0);
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    let delta = time.delta_seconds();

    let translation = direction * MOVE_SPEED * delta;
    transform.translation += translation;
}
