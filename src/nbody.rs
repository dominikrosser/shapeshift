use crate::G;

use bevy::math::Vec3;
use bevy::prelude::*;
use particular::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Particle)]
pub struct Body {
    position: Vec3,
    mu: f32,
    external_force_idx: usize, // Change this to store an index instead of a reference
}

impl Body {
    pub fn new(position: Vec3, mu: f32, external_force_idx: usize) -> Self {
        Self {
            position,
            mu,
            external_force_idx,
        }
    }
}

#[derive(Component)]
pub enum PointMass {
    HasGravity { mass: f32 },
    AffectedByGravity,
}

pub struct ParticularPlugin;

impl Plugin for ParticularPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(accelerate_particles);
    }
}

fn accelerate_particles(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut ExternalForce), With<PointMass>>,
) {
    let mut external_forces = Vec::new();
    let mut bodies = Vec::new();

    for (i, (transform, external_force)) in query.iter_mut().enumerate() {
        let position = transform.translation;
        let mass = 1.0;
        bodies.push(Body::new(position, mass, i));
        external_forces.push(external_force);
    }

    for (body, acceleration) in bodies.iter_mut().accelerations(&mut sequential::BruteForce) {
        let external_force = &mut external_forces[body.external_force_idx];
        external_force.force += Vec2::new(acceleration.x, acceleration.y);
    }
}

