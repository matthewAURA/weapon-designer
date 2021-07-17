use piston_window::{
    clear, polygon,
    types::{Polygon, Vec2d},
    PistonWindow, WindowSettings,
};

use crate::simulation::{SimulationObject, SimulationObjectShape};

pub struct DebugWindow {
    window: PistonWindow,
}

impl DebugWindow {
    pub fn new(window_height: u32, window_width: u32) -> DebugWindow {
        let mut window: PistonWindow =
            WindowSettings::new("Simulation", [window_width, window_height])
                .exit_on_esc(true)
                .build()
                .unwrap();

        DebugWindow { window: window }
    }

    pub fn draw_scene(self, objects: &Vec<SimulationObject>) {
        let ball_colour = [1.0, 0.0, 0.0, 1.0];

        match self.window.next() {
            Some(event) => {
                self.window.draw_2d(&event, |_, graphics, _device| {
                    clear([1.0; 4], graphics);
                });
                for object in objects.iter() {
                    self.window.draw_2d(&event, |context, graphics, _device| {
                        let position = [object.x as f64, object.y as f64, 5.0, 5.0];

                        let p = polygon_points(object);

                        polygon(ball_colour, &p, context.transform, graphics)

                        // circle_arc(
                        //     ball_colour,
                        //     10.0,
                        //     0.0,
                        //     3.1415 * 2.0,
                        //     position,
                        //     context.transform,
                        //     graphics,
                        // )
                    });
                }
            }
            None => {
                print!("Attempted to draw frame, but window was closed");
            }
        };
        debug_log_objects(&objects);
    }
}

fn polygon_points(object: SimulationObject) -> [Vec2d<f64>] {
    let points = match object.shape {
        SimulationObjectShape::Polygon(p) => p,
    };

    points.iter().map(f)
}

fn debug_log_objects(objects: &Vec<SimulationObject>) {
    println!("Total objects count: {}", objects.len());
    for object in objects.iter() {
        println!("Object: {} {}", object.x, object.y);
    }
}
