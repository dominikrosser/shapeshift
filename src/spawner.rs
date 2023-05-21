use bevy::{prelude::*, render::render_resource::{PipelineDescriptor, ShaderStages, RenderPipelineDescriptor}};
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

const STUFF_SPAWN_TIME_INTERVAL: f32 = 2.0;// seconds

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

pub fn tick_spawn_stuff_timer(
    mut spawn_stuff_timer: ResMut<SpawnStuffTimer>, time: Res<Time>) {
    spawn_stuff_timer.timer.tick(time.delta());
}

pub fn spawn_stuff_over_time(
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
                linvel: Vec2::new(rand::random::<f32>(),rand::random::<f32>()) * 2000.0,
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
