use bevy::{app::PluginGroupBuilder, prelude::*};

use self::button_test::ButtonTestPlugin;

mod button_test;
mod frame_counter;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ButtonTestPlugin)
        // .add(FrameCountPlugin)
    }
}
