use crate::simulation::{create_initial_simulation_objects, Simulation, SimulationObject, SimulationObjectShape};
use piston_window::*;

mod simulation;

fn draw_scene(window: &mut PistonWindow, objects: &Vec<SimulationObject>) {
    let ball_colour = [1.0, 0.0, 0.0, 1.0];

    match window.next() {
        Some(event) => {
            window.draw_2d(&event, |_, graphics, _device| {
                clear([1.0; 4], graphics);
            });
            for object in objects.iter() {
                window.draw_2d(&event, |context, graphics, _device| {
                    let position = [object.x as f64, object.y as f64, 5.0, 5.0];

                    match object.shape {
                        SimulationObjectShape::Polygon(points) => {polygon(ball_colour, points, context.transform, graphics)};
                        _ => {}
                    };

                    

                    circle_arc(
                        ball_colour,
                        10.0,
                        0.0,
                        3.1415 * 2.0,
                        position,
                        context.transform,
                        graphics,
                    )
                });
            }
        }
        None => {
            print!("Attempted to draw frame, but window was closed");
        }
    }
}

fn debug_log_objects(objects: &Vec<SimulationObject>) {
    println!("Total objects count: {}", objects.len());
    for object in objects.iter() {
        println!("Object: {} {}", object.x, object.y);
    }
}

fn main() {
    let window_height = 480;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, window_height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut simulation = Simulation::create_physics_simulation(create_initial_simulation_objects());

    while let Some(objects) = simulation.next() {
        debug_log_objects(&objects);
        draw_scene(&mut window, &objects);
    }

    // while let Some(event) = window.next() if true {
    //     ("Ball altitude: {}", ball_body.translation().y);
    // }

    // /* Run the game loop, stepping the simulation once per frame. */
    // for _ in 0..200 {
    //     physics_pipeline.step(
    //         &gravity,
    //         &integration_parameters,
    //         &mut island_manager,
    //         &mut broad_phase,
    //         &mut narrow_phase,
    //         &mut rigid_body_set,
    //         &mut collider_set,
    //         &mut joint_set,
    //         &mut ccd_solver,
    //         &physics_hooks,
    //         &event_handler,
    //     );

    //     let ball_body = &rigid_body_set[ball_body_handle];
    // println!("Ball altitude: {}", ball_body.translation().y);
    // }
}
