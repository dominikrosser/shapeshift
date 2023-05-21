
use bevy::{prelude::*, render::render_resource::{PipelineDescriptor, ShaderStages, RenderPipelineDescriptor}};
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

use crate::Fuel;
use crate::PlayerInputForce;
use crate::nbody;
use crate::force_application_plugin;

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_movement_input
                        .before(force_application_plugin::apply_player_input_force)
                        .before(nbody::accelerate_particles)
                        .after(force_application_plugin::pre_update_reset_forces))
            ;
    }
}

pub fn player_movement_input(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    //mut ext_forces: Query<&mut ExternalForce, With<Player>>,
    mut player_input_force: ResMut<PlayerInputForce>,
    mut fuel: ResMut<Fuel>,
    ) {

    player_input_force.0 = Vec2::new(0.0, 0.0);

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
        //for mut ext_force in ext_forces.iter_mut() {
            if fuel.amount >= 0.0 {
                fuel.amount -= 1.0*_time.delta_seconds();
                player_input_force.0 += direction * 150.0;
            }
        //}
    }
}
