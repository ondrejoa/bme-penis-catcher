use crate::p_catcher::core::Bme;
use bevy::prelude::*;
use bevy::render::render_resource::Texture;

pub const BME_SCALE: f32 = 0.2;

pub fn add_sprite(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut bme: Query<Entity, (With<Bme>, Without<Sprite>)>,
) {
    for bme in bme.iter_mut() {
        commands.entity(bme).insert_bundle(SpriteBundle {
            texture: assets_server.load("bme.png"),
            transform: Transform{
                translation: Vec3::new(0.0, -200.0, 0.0),
                rotation: Default::default(),
                scale: Vec3::new(BME_SCALE, BME_SCALE, 1.0),
            },
            ..Default::default()
        });
    }
}
