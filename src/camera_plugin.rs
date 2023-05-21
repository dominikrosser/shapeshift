use bevy::{prelude::*, render::render_resource::{PipelineDescriptor, ShaderStages, RenderPipelineDescriptor}};
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

use crate::Player;

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
