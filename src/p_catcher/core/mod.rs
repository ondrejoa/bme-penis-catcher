use bevy::app::Plugin;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::random;
use std::fmt::format;
use std::time::Duration;

pub mod bme;
use crate::p_catcher::core::bme::Landed;
use crate::p_catcher::input::MovementEvent;
pub use bme::Bme;
pub mod dick;
pub use dick::{BagOfDicks, Dick};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(BagOfDicks::new())
            .insert_resource(SpawnTimer(Timer::new(Duration::from_secs(3), true)))
            .insert_resource(BgSpawnTimer(Timer::new(Duration::from_secs(1), true)))
            .add_startup_system(setup)
            .add_system(movement)
            .add_system(collisions)
            .add_system(spawn_dicks)
            .add_system(remove_fallen_dicks);
    }

    fn name(&self) -> &str {
        "Penis catcher core"
    }
}

#[derive(Component)]
struct Ground;

fn setup(mut commands: Commands, mut dicks: ResMut<BagOfDicks>, assets_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.53, 0.09, 0.16),
                custom_size: Some(Vec2::new(1000.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..Default::default()
        })
        .insert(Ground)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(1000.0, 25.0));

    commands
        .spawn()
        .insert(Bme)
        .insert(RigidBody::Dynamic)
        .insert(Damping {
            linear_damping: 0.8,
            angular_damping: 0.0,
        })
        .insert(ExternalImpulse::default())
        .insert(LockedAxes::ROTATION_LOCKED_Z)
        .insert(Collider::cuboid(480.0, 150.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Landed(false))
        .insert(CollisionGroups {
            memberships: 0b01,
            filters: 0b01,
        });
}

fn movement(
    mut bme: Query<(&mut ExternalImpulse, &Landed), With<Bme>>,
    mut m: EventReader<MovementEvent>,
) {
    for m in m.iter() {
        for (mut impulse, landed) in bme.iter_mut() {
            if landed.0 {
                impulse.impulse = Vec2::new(
                    match m {
                        MovementEvent::Left => -150.0,
                        MovementEvent::Right => 150.0,
                    },
                    100.0,
                );
            }
        }
    }
}

fn collisions(
    mut bme: Query<(Entity, &mut Landed), With<Bme>>,
    ground: Query<Entity, With<Ground>>,
    mut events: EventReader<CollisionEvent>,
) {
    let get_ground = |e1: Entity, e2: Entity| ground.get(e1).or(ground.get(e2)).ok();
    for event in events.iter() {
        match event {
            CollisionEvent::Started(e1, e2, _) => {
                if let (Some((bme, mut landed)), Some(_)) = (
                    match bme.get_mut(*e1) {
                        Ok((bme, landed)) => Some((bme, landed)),
                        _ => match bme.get_mut(*e2) {
                            Ok((bme, landed)) => Some((bme, landed)),
                            _ => None,
                        },
                    },
                    get_ground(*e1, *e2),
                ) {
                    landed.0 = true;
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if let (Some((bme, mut landed)), Some(_)) = (
                    match bme.get_mut(*e1) {
                        Ok((bme, landed)) => Some((bme, landed)),
                        _ => match bme.get_mut(*e2) {
                            Ok((bme, landed)) => Some((bme, landed)),
                            _ => None,
                        },
                    },
                    get_ground(*e1, *e2),
                ) {
                    landed.0 = false;
                }
            }
        }
    }
}

fn remove_fallen_dicks(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    dicks: Query<Entity, With<Dick>>,
    ground: Query<Entity, With<Ground>>,
) {
    for event in events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            if let (Some(dick), Some(_)) = (
                dicks.get(*e1).or(dicks.get(*e2)).ok(),
                ground.get(*e1).or(ground.get(*e2)).ok(),
            ) {
                commands.entity(dick).despawn();
            }
        }
    }
}

#[derive(Component)]
struct SpawnTimer(Timer);

struct BgSpawnTimer(Timer);

fn spawn_dicks(
    mut commands: Commands,
    mut dicks: ResMut<BagOfDicks>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut timer2: ResMut<BgSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(dick) = dicks.next() {
            let asset_name = format!("dick{}.png", dick.variant);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::default(),
                        flip_x: dick.flipped,
                        flip_y: false,
                        custom_size: Some(Vec2::new(105.24, 74.4)),
                        anchor: Default::default(),
                    },
                    transform: Transform {
                        translation: Vec3::new((random::<f32>() - 0.5) * 800.0, 300.0, 0.0),
                        rotation: Default::default(),
                        scale: Vec3::new(dick.scale, dick.scale, 1.0),
                    },
                    global_transform: Default::default(),
                    texture: asset_server.load(&asset_name),
                    visibility: Default::default(),
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(35.0))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Velocity::angular(dick.rotation))
                .insert(Damping {
                    linear_damping: 0.9,
                    angular_damping: 0.0,
                })
                .insert(GravityScale(2.0))
                .insert(CollisionGroups {
                    memberships: 0b01,
                    filters: 0b01,
                })
                .insert(dick);
        }
    }
    if timer2.0.tick(time.delta()).just_finished() {
        if let Some(dick) = dicks.next() {
            let asset_name = format!("dick{}.png", dick.variant);
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.4),
                        flip_x: dick.flipped,
                        flip_y: false,
                        custom_size: Some(Vec2::new(105.24, 74.4)),
                        anchor: Default::default(),
                    },
                    transform: Transform {
                        translation: Vec3::new((random::<f32>() - 0.5) * 800.0, 300.0, 0.0),
                        rotation: Default::default(),
                        scale: Vec3::new(dick.scale, dick.scale, 1.0),
                    },
                    global_transform: Default::default(),
                    texture: asset_server.load(&asset_name),
                    visibility: Default::default(),
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(35.0))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Velocity::angular(dick.rotation))
                .insert(Damping {
                    linear_damping: 0.9,
                    angular_damping: 0.0,
                })
                .insert(GravityScale(2.0))
                .insert(CollisionGroups {
                    memberships: 0b10,
                    filters: 0b10,
                })
                .insert(dick);
        }
    }
}
