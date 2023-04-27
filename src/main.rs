use bevy::{prelude::*, ecs::storage::Resources};
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButtonInput;
use bevy_mouse_tracking_plugin::{
    MousePosPlugin, MousePosWorld,
};
use bevy::input::ButtonState;
use particular::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // Local Geomorpher Plugins:
        .add_plugin(GMCameraPlugin)
        // Particle set
        .add_startup_system(place_body)

        .add_startup_system(setup_player)
        .add_startup_system(setup_platform)
        .add_system(print_ball_altitude)
        .add_system(player_movement_system)
        .run();
}

/* PARTICULAR */
fn place_body(
    mut commands: Commands,
    mut body_info: ResMut<GolfBallSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ) {

    body_info.position = Some(Vec3::new(0.0, 0.0, 0.0));
    let place_pos = body_info.position.unwrap();

    let mass: f32 = 100.0_f32.max(1.0);
    let density = 1.0;
    let radius =
        (mass / (density * PI)).sqrt();
    let entity = commands.spawn(
        CircleWithGravity {
            shape_bundle: MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Circle {
                        radius,
                        ..default()
                    }))
                .into(),
                transform: Transform::from_xyz(place_pos.x, place_pos.y, place_pos.z),
                material: materials
                    .add(ColorMaterial::from(Color::WHITE)),
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
            rigidbody: RigidBody::Dynamic,
            velocity: Velocity::linear(Vec2::new(0.1,0.1)),
            acceleration: ExternalForce::default(),
            point_mass: PointMass::HasGravity {
                // mass: body_info.mass,
                mass: mass,
            },
        });
}
use std::f32::consts::PI;
use bevy::sprite::{Material2d, MaterialMesh2dBundle};
#[derive(Component)]
pub enum PointMass {
    HasGravity { mass: f32 },
    AffectedByGravity,
}

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
    pub point_mass: PointMass,
}

pub struct GolfBallSettings {
    pub position: Option<Vec3>,
    pub mass: f32,
    pub trail: bool,
}
impl bevy::prelude::Resource for GolfBallSettings {
    
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
#[derive(Particle, Component)]
struct Body {
    position: Vec3,
    mu: f32,
}
fn create_bodies(
    mut commands: Commands,
    ) {
    commands.spawn(
        Body {
        position: Vec3::new(0.0, 0.0, 0.0),
        mu: 1.0,
    }
    );
}
/* PARTICULAR */

pub struct GMCameraPlugin;
impl Plugin for GMCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_camera)
            .add_system(move_camera_system);
    }
}

fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let _window = window_query.get_single().unwrap();
    commands.spawn(
            Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            })
            ;
}

fn move_camera_system(
    mut camera_transform_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_transform_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    time: Res<Time>,
    ) {
    // Get the player's translation
    if let Ok(player_transform) = player_transform_query.get_single() {
        let player_translation = player_transform.translation;
        for mut camera_transform in camera_transform_query.iter_mut() {
            let camera_translation = camera_transform.translation;
            let speed = 100.0;
            let movement_delta = (player_translation - camera_translation) * time.delta_seconds() * speed;
            let movement_delta = movement_delta.clamp_length_max((player_translation-camera_translation).length());
            camera_transform.translation += movement_delta;
        }
    }
}

fn setup_platform(
    mut commands: Commands,
) {
    /* Create the ground */
    commands.spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -250.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.8));
    
}

fn setup_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let shape_material = assets.load("sprites\\ball_blue_large.png");

    /* Create the bouncing ball */
    commands.spawn(RigidBody::Dynamic)
        .insert(Collider::ball(32.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 250.0, 0.0)))
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
        .insert(GravityScale(1.0))
        .insert(Ccd::enabled())
        .insert(ExternalForce::default())
        // Custom Stuff:
        .insert(SpriteBundle {
                texture: shape_material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
        .insert(Player{});

}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

#[derive(Component)]
pub struct Player {}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
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
    for mut ext_force in ext_forces.iter_mut() {
        ext_force.force = direction * 150.0;
    }
}


