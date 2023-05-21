use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Material2d}};
use bevy_rapier2d::{prelude::{*, Velocity}, rapier::prelude::{ColliderShape, ColliderBuilder}, na::Point2};
use bevy::render::render_resource::Texture;

#[derive(Component)]
pub struct Player {}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
) {
    let shape = CustomShape::RegularPolygon {
        radius: 32.0,
        sides: 6,
        color: Color::rgb(0.0, 1.0, 0.0),
    };
    let transform = Transform::from_xyz(0.0, 0.0, 0.0);
    commands.spawn((
        Name::new("Player"),
        Player{},
        PhysicsRigidBodyBundle::new(),
        CustomShapeBundle::new(shape, transform, meshes, materials, assets)
        ));
}

pub enum CustomShape {
    RegularPolygon {
        radius: f32,
        sides: usize,
        color: Color,
    },
}

impl CustomShape {
    fn random_color() -> Color {
        Color::rgb(rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>())
    }

    fn random_radius() -> f32 {
        rand::random::<f32>() * 32.0 + 16.0
    }

    fn random_sides() -> usize {
        rand::random::<usize>() % 10 + 3
    }

    pub fn random_regular_polygon() -> Self {
        Self::RegularPolygon {
            radius: Self::random_radius(),
            sides: Self::random_sides(),
            color: Self::random_color(),
        }
    }
}

#[derive(Bundle)]
pub struct PhysicsRigidBodyBundle {
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub collider_mass_properties: ColliderMassProperties,
    pub restitution: Restitution,
    pub read_mass_properties: ReadMassProperties,
    pub damping: Damping,
    pub gravity_scale: GravityScale,
    pub ccd: Ccd,
    pub external_force: ExternalForce,
}

impl PhysicsRigidBodyBundle {
    pub fn new() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            collider_mass_properties: ColliderMassProperties::Density(1.0),
            restitution: Restitution::coefficient(0.7),
            read_mass_properties: ReadMassProperties::default(),
            damping: Damping {
                linear_damping: 0.5,
                angular_damping: 0.5
            },
            gravity_scale: GravityScale(0.0),
            ccd: Ccd::enabled(),
            external_force: ExternalForce::default(),
        }
    }
}

impl Default for PhysicsRigidBodyBundle {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Bundle)]
pub struct CustomShapeBundle {
    #[bundle]
    pub material_mesh_2d: MaterialMesh2dBundle<ColorMaterial>,

    pub collider: Collider,
}

impl CustomShapeBundle {
    pub fn new(shape: CustomShape, transform: Transform, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, assets: Res<AssetServer>) -> Self {
        match shape {
            CustomShape::RegularPolygon { radius, sides, color } => Self {
                material_mesh_2d: MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::RegularPolygon::new(radius, sides))).into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform,
                    ..Default::default()
                },
                collider: create_vertices_from_shape(&shape),
            },
        }
    }
}

fn create_vertices_from_shape(shape: &CustomShape) -> Collider {
    match shape {
        CustomShape::RegularPolygon { radius, sides, .. } => {
            let mut angles = Vec::new();
            let mut points = Vec::new();
            for i in 0..*sides {
                let angle = i as f32 * 2.0 * std::f32::consts::PI / *sides as f32;
                angles.push(angle);
            }

            let angle_zero = angles[0] - std::f32::consts::PI / 2.0;
            let angles: Vec<_> = angles.into_iter().map(|angle| angle - angle_zero).collect();

            for angle in angles {
                let x = angle.cos() * *radius;
                let y = angle.sin() * *radius;
                points.push(Vec2::new(x, y));
            }
            Collider::convex_hull(&points).unwrap()
        },
    }
}
