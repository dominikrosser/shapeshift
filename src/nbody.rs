use bevy::math::Vec3;
use bevy::prelude::*;
use particular::prelude::*;
use bevy_rapier2d::prelude::*;

//use crate::Player;

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

pub struct ParticularPlugin;

impl Plugin for ParticularPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(accelerate_particles);
    }
}

pub fn accelerate_particles(
    _time: Res<Time>,
    mut query: Query<(&mut Transform, &mut ExternalForce, &ReadMassProperties)/*, With<PointMass>*/>,
) {
    let mut external_forces = Vec::new();
    let mut bodies = Vec::new();

    for (i, (transform, external_force, read_mass_properties)) in query.iter_mut().enumerate() {
        let position = transform.translation;
        let mass = read_mass_properties.0.mass;
        bodies.push(Body::new(position, mass, i));
        external_forces.push(external_force);
    }

    for (body, acceleration) in bodies.iter_mut().accelerations(&mut sequential::BruteForce) {
        let external_force : &mut ExternalForce = &mut external_forces[body.external_force_idx];
        let acceleration = acceleration * crate::G;
        //println!("External Force: {:?}", external_force);
        //println!("Acceleration: {:?}", acceleration);
        //external_force.force = Vec2::new(acceleration.x, acceleration.y);
        if acceleration.x.is_nan() || acceleration.y.is_nan() {
            println!("Acceleration is NaN");
            continue;
        }
        external_force.force += Vec2::new(acceleration.x, acceleration.y);
    }
}

