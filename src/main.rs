#![feature(slice_range)]

mod nbody;
mod spawner;
mod force_application_plugin;
mod camera_plugin;
mod player_input_plugin;

use bevy::{prelude::*, render::render_resource::{PipelineDescriptor, ShaderStages, RenderPipelineDescriptor}};
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

use nbody::ParticularPlugin;
use spawner::{SpawnStuffTimer, tick_spawn_stuff_timer, spawn_stuff_over_time};
use force_application_plugin::GMForceApplicationPlugin;
use camera_plugin::{GMCameraPlugin, CameraSettings};
use player_input_plugin::{PlayerInputPlugin};

const G: f32 = 500000.0;// Arbitrarily chosen gravitational constant
const NUM_RANDOMLY_ADDED_BODIES: usize = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ParticularPlugin)

        .add_plugin(GMCameraPlugin)
        .add_plugin(GMForceApplicationPlugin)
        .add_plugin(PlayerInputPlugin)

        .add_startup_system(setup_player)
        .add_startup_system(setup_bodies)
        .add_startup_system(setup_platform_walls)
        //.add_startup_system(setup_bodies_large)

        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::default())// idk what this does
         
        .init_resource::<Fuel>()
        .init_resource::<SpawnStuffTimer>()
        .init_resource::<PlayerInputForce>()
        
        //.add_system(player_movement_input
                    //.before(force_application_plugin::apply_player_input_force)
                    //.before(nbody::accelerate_particles)
                    //.after(force_application_plugin::pre_update_reset_forces)
                    //.before(print_fuel))
        .add_system(print_fuel)
        .add_system(print_ball_altitude)
        
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


use bevy::sprite::MaterialMesh2dBundle;
use std::f32::consts::PI;
use bevy::sprite::Material2d;


#[derive(Bundle)]
pub struct CircleWithGravity<M: Material2d> {
    #[bundle]
    pub shape_bundle: MaterialMesh2dBundle<M>,
    pub collider: Collider,
    pub friction: Friction,
    pub mass: ColliderMassProperties,
    pub restitution: Restitution,
    pub rigidbody: RigidBody,
    pub velocity: Velocity,
    pub acceleration: ExternalForce,
    pub point_mass: ReadMassProperties,
}

fn setup_bodies_large(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    for i in 0..5 {
        /// Planet 1
        let mass = 10E5;
        let density = 20.0;
        let i_fl = i as f32;
        let radius = (mass / (density * PI)).sqrt();
        let entity = commands.spawn(CircleWithGravity {
            shape_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        ..default()
                    }))
                .into(),
                transform: Transform::from_xyz(
                    -300.0+200.0*i_fl, -50.0, 1.0,
                    ),
                    material: materials
                        .add(ColorMaterial::from(Color::BLUE)),
                        ..default()
            },
            collider: Collider::ball(radius),
            friction: Friction {
                coefficient: 10.0,
                ..default()
            },
            mass: ColliderMassProperties::Mass(mass),
            restitution: Restitution {
                coefficient: 0.0,
                ..default()
            },
            rigidbody: RigidBody::Fixed,
            velocity: Velocity::zero(),
            acceleration: ExternalForce::default(),
            point_mass: ReadMassProperties::default(),
        });
    }
}

fn setup_bodies(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let shape_material = assets.load("sprites\\ball_red_large.png");
   
    for i in 1..=NUM_RANDOMLY_ADDED_BODIES {
        let fact = i as f32;
        commands.spawn(RigidBody::Dynamic)
            .insert(Collider::ball(32.0))
            .insert(TransformBundle::from(Transform::from_xyz(100.0*fact, 200.0, 0.0)))
            .insert(Velocity {
                linvel: Vec2::new(rand::random::<f32>(),rand::random::<f32>()) * 200.0,
                angvel: 0.0,
            })
        .insert(ColliderMassProperties::Density(0.5))
            .insert(Restitution::coefficient(0.7))
            .insert(Damping {
                linear_damping: 0.5,
                angular_damping: 0.5
            })
        .insert(ReadMassProperties::default())
        .insert(GravityScale(0.0))
            .insert(Ccd::enabled())
            .insert(ExternalForce {
               force: Vec2::new(0.0,0.0),
               torque: 0.0,
            })
            // Custom Stuff:
            .insert(SpriteBundle {
                texture: shape_material.clone(),
                transform: Transform::default(),
                ..Default::default()
            });
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

fn setup_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let shape_material = assets.load("sprites\\ball_blue_large.png");

    /* Create the bouncing ball */
    commands.spawn(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(TransformBundle::from(Transform::from_xyz(-500.0, 500.0, 0.0)))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(ColliderMassProperties::Density(1.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5
        })
        .insert(ReadMassProperties::default())
        .insert(GravityScale(0.0))
        .insert(Ccd::enabled())
        .insert(ExternalForce::default())
        // Custom Stuff:
        .insert(SpriteBundle {
                texture: shape_material,
                transform: Transform::default(),
                ..Default::default()
            })
        .insert(Player{})
        .insert(Name::new("Player"));
}


pub struct GolfBallSettings {
    pub position: Option<Vec3>,
    pub mass: f32,
    pub trail: bool,
}

impl Default for GolfBallSettings {
    fn default() -> Self {
        Self {
            position: None,
            mass: 20.0,
            trail: false,
        }
    }
}

fn print_ball_altitude(positions: Query<&Transform, (With<RigidBody>, With<Player>)>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

#[derive(Component)]
pub struct Player {}



fn print_fuel(fuel: Res<Fuel>) {
    println!("Fuel: {}", fuel.amount);
}













