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

fn detect_collision(
    mut query: Query<(&mut Player, &Transform)>,
    enemy_query: Query<(&Enemy, &Transform)>,
) {
    let (mut player, player_transform) = query.single_mut();

    for (_, enemy_transform) in enemy_query.iter() {
        let collision = player_transform
            .translation
            .distance(enemy_transform.translation)
            < 1.0;

        if collision {
            if player.health <= 0 {
                panic!("You are now dead! Congratulations!");
            }
            player.health -= 10;
            println!("{0}", player.health);
        }
    }
}
