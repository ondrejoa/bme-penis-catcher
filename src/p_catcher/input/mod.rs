use bevy::prelude::*;
use bevy::app::Plugin;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementEvent>()
            .add_system(movement);
    }

    fn name(&self) -> &str {
        "Penis catcher input"
    }
}

pub enum MovementEvent {
    Left,
    Right,
}

fn movement(keys: Res<Input<KeyCode>>, mut m: EventWriter<MovementEvent>) {
    if keys.just_pressed(KeyCode::Left) {
        m.send(MovementEvent::Left);
    }
    if keys.just_pressed(KeyCode::Right) {
        m.send(MovementEvent::Right);
    }
}