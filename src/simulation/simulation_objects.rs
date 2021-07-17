use rand::distributions::{Distribution, Standard};
use rand::{rngs::StdRng, SeedableRng};
use rapier2d::prelude::{ColliderBuilder, Point, RigidBodyBuilder, Vector};

use crate::simulation::simulation::InitialSimulationObjects;
use crate::simulation::simulation::PhysicsObject;

pub fn create_initial_simulation_objects() -> InitialSimulationObjects {
    return InitialSimulationObjects {
        spinner: create_spinner(),
        target: create_target(),
    };
}

fn create_target() -> PhysicsObject {
    let rigid_body = RigidBodyBuilder::new_dynamic()
        .translation(Vector::new(100.0, 380.0))
        .build();
    let collider = ColliderBuilder::ball(10.0).restitution(0.7).build();

    return PhysicsObject {
        body: rigid_body,
        collider: collider,
    };
}

fn create_spinner() -> PhysicsObject {
    let rigid_body = RigidBodyBuilder::new_kinematic_position_based()
        .translation(Vector::new(10.0, 10.0))
        .build();
    let collider = ColliderBuilder::cuboid(200.0, 380.0)
        .restitution(0.7)
        .build();

    let scale = 4.0;
    let mut rng = StdRng::seed_from_u64(0);
    let distribution = Standard;

    let mut points = Vec::new();

    for _ in 0..10 {
        let (x, y): (f32, f32) = distribution.sample(&mut rng);
        let pt: Point<f32> = Point::new(x, y);
        points.push(pt * scale);
    }

    return PhysicsObject {
        body: rigid_body,
        collider: collider,
    };
}
