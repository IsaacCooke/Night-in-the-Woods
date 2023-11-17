use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use enemy_cube::EnemyCubePlugin;
use environment::EnvironmentPlugins;
use physics::PhysicsPlugin;
use player_physics::PlayerPhysicsPlugin;
use ui::UiPlugins;

mod enemy_cube;
mod environment;
mod physics;
mod player_physics;
mod ui;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            EnemyCubePlugin,
            EnvironmentPlugins,
            PhysicsPlugin,
            PlayerPhysicsPlugin,
            // UiPlugins,
        ))
        .add_systems(Update, detect_collision)
        //.add_systems(Startup, setup)
        .run();
}

use enemy_cube::Enemy;
use player_physics::Player;

fn detect_collision(query: Query<(&Player, &Transform)>, enemy_query: Query<(&Enemy, &Transform)>) {
    let (_, player_transform) = query.single();

    for (_, enemy_transform) in enemy_query.iter() {
        let collision = player_transform
            .translation
            .distance(enemy_transform.translation)
            < 1.0;
        if collision {
            println!("Collision detected!");
        }
    }
}
