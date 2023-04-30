#![feature(slice_range)]

mod nbody;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;
use nbody::ParticularPlugin;

const G: f32 = 500000.0;// Arbitrarily chosen gravitational constant
const NUM_RANDOMLY_ADDED_BODIES: usize = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GMCameraPlugin)
        .add_plugin(ParticularPlugin)
        .add_startup_system(setup_player)
        .add_startup_system(setup_bodies)
        .add_startup_system(setup_platform_walls)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Fuel>()
        .add_system(print_ball_altitude)
        .add_system(
            pre_update_reset_forces.before(nbody::accelerate_particles).before(player_movement_system))
        .add_system(player_movement_system.after(nbody::accelerate_particles))
        .add_system(print_fuel)
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

const STUFF_SPAWN_TIME_INTERVAL: f32 = 5.0;// seconds

#[derive(Resource)]
pub struct SpawnStuffTimer {
    pub timer: Timer,
}
impl Default for SpawnStuffTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STUFF_SPAWN_TIME_INTERVAL, TimerMode::Repeating),
        }
    }
}

fn tick_spawn_stuff_timer(
    mut spawn_stuff_timer: ResMut<SpawnStuffTimer>, time: Res<Time>) {
    spawn_stuff_timer.timer.tick(time.delta());
}

fn spawn_stuff_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    spawn_stuff_timer: Res<SpawnStuffTimer>,
    ) {
    let shape_material = asset_server.load("sprites\\ball_red_large.png");

    if spawn_stuff_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        commands.spawn(RigidBody::Dynamic)
            .insert(Collider::ball(32.0))
            .insert(TransformBundle::from(Transform::from_xyz(100.0*rand::random::<f32>(), 200.0*rand::random::<f32>(), 0.0)))
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

fn pre_update_reset_forces(
    mut bodies: Query<&mut ExternalForce>,
) {
    for mut body in bodies.iter_mut() {
        body.force = Vec2::new(0.0, 0.0);
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

pub struct GMCameraPlugin;
impl Plugin for GMCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_camera)
            .add_system(move_camera_system);
    }
}

pub struct CameraSettings {
    pub far: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub follow_player: bool,
    pub camera_follow_speed: f32,
}

const CAMERA_SETTINGS : CameraSettings = CameraSettings {
    far: 1000.0,
    scale_x: 2.0,
    scale_y: 2.0,
    follow_player: true,
    camera_follow_speed: 100.0,
};

fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let _window = window_query.get_single().unwrap();
    commands.spawn(
            Camera2dBundle {
                transform: 
                    Transform::from_xyz(0.0, 0.0, 1.0)
                        .with_scale(Vec3::new(CAMERA_SETTINGS.scale_x, CAMERA_SETTINGS.scale_y, 1.)),
                projection: OrthographicProjection {
                    far: CAMERA_SETTINGS.far,
                    ..default()
                },
                ..default()
            })
            ;
}

fn move_camera_system(
    mut camera_transform_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_transform_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    time: Res<Time>,
    ) {

    if !CAMERA_SETTINGS.follow_player {
        return;
    }
    // Get the player's translation
    if let Ok(player_transform) = player_transform_query.get_single() {
        let player_translation = player_transform.translation;
        for mut camera_transform in camera_transform_query.iter_mut() {
            let camera_translation = camera_transform.translation;
            let speed = CAMERA_SETTINGS.camera_follow_speed;
            let movement_delta = (player_translation - camera_translation) * time.delta_seconds() * speed;
            let movement_delta = movement_delta.clamp_length_max((player_translation-camera_translation).length());
            camera_transform.translation += movement_delta;// Replace this with = for a cool effect
        }
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

fn player_movement_system(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
    mut fuel: ResMut<Fuel>,
    ) {
    let mut direction = Vec2::new(0.0, 0.0);
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
        for mut ext_force in ext_forces.iter_mut() {
            if fuel.amount >= 0.0 {
                fuel.amount -= 1.0*_time.delta_seconds();
                ext_force.force += direction * 150.0;
            }
        }
    }
}

fn print_fuel(fuel: Res<Fuel>) {
    println!("Fuel: {}", fuel.amount);
}













