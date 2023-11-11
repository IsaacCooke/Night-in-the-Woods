use bevy::{
    math::{cubic_splines::CubicCurve, vec3},
    prelude::*,
};
use bevy_xpbd_3d::prelude::*;

pub struct EnemyCubePlugin;

#[derive(Component)]
pub struct Curve(CubicCurve<Vec3>);

impl Plugin for EnemyCubePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, animate_cube);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let points = [[
        vec3(-6., 1., 1.),
        vec3(12., 1., 1.),
        vec3(-12., 1., 1.),
        vec3(6., 1., 1.),
    ]];

    // Make a CubicCurve
    let bezier = CubicBezier::new(points).to_curve();

    // Spawning a cube to experiment on
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_translation(points[0][0]),
            ..default()
        },
        Curve(bezier),
    ));
}

pub fn animate_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Curve)>,
    mut gizmos: Gizmos,
) {
    let t = (time.elapsed_seconds().sin() + 1.0) / 2.0;

    for (mut transform, cubic_cuve) in &mut query {
        gizmos.linestrip(cubic_cuve.0.iter_positions(50), Color::WHITE);
        transform.translation = cubic_cuve.0.position(t);
    }
}
