use crate::p_catcher::PCatcherPlugins;
use bevy::prelude::*;

mod p_catcher;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "BME penis catcher".to_owned(),
            width: 1000.0,
            height: 600.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugins(PCatcherPlugins)
        .run();
}
