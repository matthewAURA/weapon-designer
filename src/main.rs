use crate::simulation::{create_initial_simulation_objects, Simulation};
use window::DebugWindow;

mod simulation;
mod window;

fn main() {
    let window = DebugWindow::new(480, 640);

    let mut simulation = Simulation::create_physics_simulation(create_initial_simulation_objects());

    while let Some(objects) = simulation.next() {
        window.draw_scene(&objects);
    }
}
