
use bevy::{prelude::*, render::render_resource::{PipelineDescriptor, ShaderStages, RenderPipelineDescriptor}};
use bevy_rapier2d::prelude::{*, Velocity};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::PrimaryWindow;

use crate::nbody;
use crate::PlayerInputForce;
use crate::Player;

pub struct GMForceApplicationPlugin;
impl Plugin for GMForceApplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(pre_update_reset_forces
                        .before(nbody::accelerate_particles)
                        .before(apply_player_input_force))
            .add_system(apply_player_input_force.after(nbody::accelerate_particles));
    }
}

pub fn apply_player_input_force(
    player_input_force: Res<PlayerInputForce>,
    mut player: Query<&mut ExternalForce, With<Player>>,
) {
    for mut player in player.iter_mut() {
        player.force += player_input_force.0;
    }
}

pub fn pre_update_reset_forces(
    mut bodies: Query<&mut ExternalForce>,
) {
    for mut body in bodies.iter_mut() {
        body.force = Vec2::new(0.0, 0.0);
    }
}
