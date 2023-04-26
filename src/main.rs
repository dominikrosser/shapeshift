use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .add_system(player_movement_system)
        .add_system(player_movement_confine_bounce_window_walls)
        .run();
}

fn setup_physics(
    mut commands: Commands,
) {
    /* Create the ground */
    commands.spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -250.0, 0.0)));
    
    /* Create the bouncing ball */
    commands.spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 250.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn setup_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
            //UiCameraConfig::default(),
            ));
}

#[derive(Component)]
pub struct Player {}

// TODO make velocity acceleration and mass into a bundle

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Acceleration(Vec2);

#[derive(Component)]
struct Mass(f32);

fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    ) {
    let shape_material = assets.load("sprites\\ball_blue_large.png");
    commands.spawn((
            SpriteBundle {
                texture: shape_material,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            Player {},
            Velocity(Vec2::new(0.0, 0.0)),
            Acceleration(Vec2::new(0.0, 0.0)),
            Mass(1.0),
        ));
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &Mass), With<Player>>,
    ) {

    let mut input_acceleration = Vec2::new(0.0, 0.0);
    if keyboard_input.pressed(KeyCode::W) {
        input_acceleration.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input_acceleration.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        input_acceleration.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input_acceleration.x += 1.0;
    }

    if let Ok((mut transform, mut velocity, mut acceleration, mass)) = player_query.get_single_mut() { 

        if input_acceleration.length() > 0.0 {
            input_acceleration = input_acceleration.normalize()*10000.0 / mass.0;
        }
        acceleration.0 = input_acceleration;// TODO

        // Update velocity based on acceleration and time
        velocity.0 += acceleration.0 * time.delta_seconds();

        // Update position based on velocity and time
        transform.translation += velocity.0.extend(0.0) * time.delta_seconds();
    }
}

const PLAYER_SIZE : f32 = 64.0;

fn player_movement_confine_bounce_window_walls(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &Mass), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ) {

    let window = window_query.get_single().unwrap();

    if let Ok((mut transform, mut velocity, mut acceleration, mass)) = player_query.get_single_mut() {
        if transform.translation.x < -window.width()/2.0 + PLAYER_SIZE/2.0 {
            transform.translation.x = -window.width()/2.0 + PLAYER_SIZE/2.0;
            velocity.0.x = -velocity.0.x;
        } else if transform.translation.x > window.width()/2.0 - PLAYER_SIZE/2.0 {
            transform.translation.x = window.width()/2.0 - PLAYER_SIZE/2.0;
            velocity.0.x = -velocity.0.x;
        }
        if transform.translation.y < -window.height()/2.0 + PLAYER_SIZE/2.0 {
            transform.translation.y = -window.height()/2.0 + PLAYER_SIZE/2.0;
            velocity.0.y = -velocity.0.y;
        } else if transform.translation.y > window.height()/2.0 - PLAYER_SIZE/2.0 {
            transform.translation.y = window.height()/2.0 - PLAYER_SIZE/2.0;
            velocity.0.y = -velocity.0.y;
        }
    }
}
