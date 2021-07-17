use rapier2d::prelude::Point;

use rapier2d::prelude::BroadPhase;
use rapier2d::prelude::CCDSolver;
use rapier2d::prelude::ColliderBuilder;
use rapier2d::prelude::ColliderSet;
use rapier2d::prelude::IntegrationParameters;
use rapier2d::prelude::IslandManager;
use rapier2d::prelude::JointSet;
use rapier2d::prelude::NarrowPhase;
use rapier2d::prelude::PhysicsPipeline;
use rapier2d::prelude::RigidBodySet;
use rapier2d::prelude::TypedShape;
use rapier2d::prelude::Vector;
use rapier2d::prelude::{Collider, RigidBody};

pub enum SimulationObjectShape {
    Polygon(Vec<Point<f32>>),
    Rectangle(f32, f32),
}

pub struct SimulationObject {
    pub x: f64,
    pub y: f64,
    pub shape: SimulationObjectShape,
}

pub struct PhysicsObject {
    pub body: RigidBody,
    pub collider: Collider,
}

pub struct InitialSimulationObjects {
    pub spinner: PhysicsObject,
    pub target: PhysicsObject,
}

fn get_points_for_collider(collider: &Collider) -> SimulationObjectShape {
    match collider.shape().as_typed_shape() {
        TypedShape::ConvexPolygon(cp) => SimulationObjectShape::Polygon(cp.points().to_vec()),
        _ => return SimulationObjectShape::Rectangle(10.0, 10.0),
    }
}

struct SimulationData {
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    collider_set: ColliderSet,
    joint_set: JointSet,
    ccd_solver: CCDSolver,
}

pub struct Simulation {
    max_steps: i32,
    current_step: i32,
    objects: RigidBodySet,
    physics_pipeline: PhysicsPipeline,
    simulation_data: SimulationData,
}

impl Simulation {
    pub fn create_physics_simulation(inital_objects: InitialSimulationObjects) -> Simulation {
        let spinner = inital_objects.spinner;
        let target = inital_objects.target;

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        // Create the ground
        let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
        collider_set.insert(collider);

        // Create the "weapon"
        let spinner_handle = rigid_body_set.insert(spinner.body);

        // Create the object to be yeeted
        let ball_body_handle = rigid_body_set.insert(target.body);

        collider_set.insert_with_parent(target.collider, ball_body_handle, &mut rigid_body_set);
        collider_set.insert_with_parent(spinner.collider, spinner_handle, &mut rigid_body_set);
        /* Create other structures necessary for the simulation. */

        let physics_pipeline = PhysicsPipeline::new();

        let simulation_data = SimulationData {
            gravity: Vector::new(0.0, -9.81),
            collider_set: collider_set,
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joint_set: JointSet::new(),
            ccd_solver: CCDSolver::new(),
        };

        return Simulation {
            max_steps: 500,
            current_step: 0,
            objects: rigid_body_set,
            physics_pipeline: physics_pipeline,
            simulation_data: simulation_data,
        };
    }

    fn step(&mut self) {
        let physics_hooks = ();
        let event_handler = ();
        self.physics_pipeline.step(
            &self.simulation_data.gravity,
            &self.simulation_data.integration_parameters,
            &mut self.simulation_data.island_manager,
            &mut self.simulation_data.broad_phase,
            &mut self.simulation_data.narrow_phase,
            &mut self.objects,
            &mut self.simulation_data.collider_set,
            &mut self.simulation_data.joint_set,
            &mut self.simulation_data.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
    }

    fn simulation_objects(&mut self) -> Vec<SimulationObject> {
        let mut objects: Vec<SimulationObject> = Vec::new();

        for (_, collider) in self.simulation_data.collider_set.iter() {
            let object = SimulationObject {
                x: collider.translation().x as f64,
                y: collider.translation().y as f64,
                shape: get_points_for_collider(collider),
            };

            objects.push(object);
        }

        objects
    }
}

impl Iterator for Simulation {
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        if self.max_steps == self.current_step {
            return None;
        }

        println!("Step: {}", self.current_step);
        self.current_step += 1;

        self.step();

        return Some(self.simulation_objects());
    }
    type Item = Vec<SimulationObject>;
}
