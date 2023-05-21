#![feature(slice_range)]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

mod nbody;
mod spawner;
mod force_application_plugin;
mod camera_plugin;
mod player;
mod player_input_plugin;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;


use nbody::ParticularPlugin;
use spawner::{SpawnStuffTimer, tick_spawn_stuff_timer, spawn_stuff_over_time};
use force_application_plugin::GMForceApplicationPlugin;
use camera_plugin::GMCameraPlugin;
use player::{setup_player, Player, PhysicsRigidBodyBundle};
use player_input_plugin::PlayerInputPlugin;

const G: f32 = 100000.0;// Arbitrarily chosen gravitational constant
const NUM_RANDOMLY_ADDED_BODIES: usize = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ParticularPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(GMCameraPlugin)
        .add_plugin(GMForceApplicationPlugin)

        .add_plugin(PlayerInputPlugin)
        .add_startup_system(setup_player)
        .add_startup_system(setup_bodies)
        .add_startup_system(setup_platform_walls)

        .init_resource::<Fuel>()
        .init_resource::<PlayerInputForce>()
        
        .init_resource::<SpawnStuffTimer>()
        .add_system(tick_spawn_stuff_timer)
        .add_system(spawn_stuff_over_time)
        .run();
}

#[derive(Resource)]
pub struct Fuel {
    pub amount: f32,
}
impl Default for Fuel {
    fn default() -> Self {
        Self {
            amount: 100.0,
        }
    }
}

#[derive(Resource)]
pub struct PlayerInputForce(pub Vec2);
impl Default for PlayerInputForce {
    fn default() -> Self {
        Self(Vec2::default())
    }
}

fn setup_bodies(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let shape_material = assets.load("sprites\\ball_red_large.png");
   
    for i in 1..=NUM_RANDOMLY_ADDED_BODIES {
        let fact = i as f32;
        commands.spawn((
                Name::new(format!("Randomly-added-body-{}", i)),
                SpriteBundle {
                    texture: shape_material.clone(),
                    transform: Transform::from_xyz(100.0*fact, 200.0, 0.0),
                    ..default()
                },
                Collider::ball(32.0),
                PhysicsRigidBodyBundle::default(),
                ));
    }

}

fn setup_platform_walls(
    mut commands: Commands,
) {
    /* Create the ground */
    commands.spawn(Collider::cuboid(2000.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -1000.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.8))
        .insert(Name::new("Ground"));

    /* Create the ceiling */
    commands.spawn(Collider::cuboid(5000.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 1000.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.8))
        .insert(Name::new("Ceiling"));
    
    /* Create the left wall */
    commands.spawn(Collider::cuboid(50.0, 2000.0))
        .insert(TransformBundle::from(Transform::from_xyz(-1000.0, 0.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.8))
        .insert(Name::new("Left Wall"));

    /* Create the right wall */
    commands.spawn(Collider::cuboid(50.0, 2000.0))
        .insert(TransformBundle::from(Transform::from_xyz(1000.0, 0.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.8))
        .insert(Name::new("Right Wall"));
}
