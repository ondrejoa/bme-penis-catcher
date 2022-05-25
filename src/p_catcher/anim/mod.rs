use bevy::app::Plugin;
use bevy::math::vec3;
use bevy::prelude::*;

mod bme;

pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(bme::add_sprite);
    }

    fn name(&self) -> &str {
        "Penis catcher anim"
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
