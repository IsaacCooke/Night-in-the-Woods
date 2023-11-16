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
        //.add_systems(Startup, setup)
        .run();
}
