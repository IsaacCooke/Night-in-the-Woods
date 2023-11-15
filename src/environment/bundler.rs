use crate::environment::floor::FloorPlugin;
use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct EnvironmentPlugins;

impl PluginGroup for EnvironmentPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(FloorPlugin)
    }
}
