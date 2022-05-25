use self::{anim::AnimPlugin, core::CorePlugin, input::InputPlugin};
use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy::prelude::*;

mod anim;
pub mod core;
mod input;

pub struct PCatcherPlugins;

impl PluginGroup for PCatcherPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(CorePlugin).add(AnimPlugin).add(InputPlugin);
    }
}
