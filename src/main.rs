use macroquad::prelude::*;

const TIME_STEP: f64 = 0.5;
const STEPS: usize = 100000;
const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

type Position = DVec2;

#[derive(Debug, Clone, Copy)]
struct Body {
    mass: f64,
    position: Position,
    velocity: Position,
}

impl Body {
    fn new(position: Position) -> Self {
        Body {
            mass: 1.0,
            velocity: Position::ZERO,
            position,
        }
    }
}

struct Step {
    time: f64,
    step: u32,
    bodies: [Body; 3],
}

fn main() {
    let mut first = Body::new(dvec2(0.3089693008, 0.4236727692));
    let mut second = Body::new(dvec2(-0.5, 0.0));
    let mut third = Body::new(dvec2(0.5, 0.0));

    for n in 0..STEPS {
        let mut new_step = Step {
            time: (n as f64) * TIME_STEP,
            step: n as u32,
            bodies: [first, second, third],
        };

        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    let a = &new_step.bodies[j];
                    let mut b: Body = new_step.bodies[i];

                    let dx = a.position.x - b.position.x;
                    let dy: f64 = a.position.y - b.position.y;

                    let r: f64 = (dx * dx + dy * dy).sqrt();
                    let force = GRAVITATIONAL_CONSTANT * a.mass * b.mass / r / r;
                    let angle = dy.atan2(dx);
                    let fx = force * angle.cos();
                    let fy = force * angle.sin();
                    b.velocity.x += fx / b.mass * TIME_STEP;
                    b.velocity.y += fy / b.mass * TIME_STEP;

                    new_step.bodies[i] = b;
                }
            }
        }

        for body in new_step.bodies.iter_mut() {
            body.position.x += body.velocity.x * TIME_STEP;
            body.position.y += body.velocity.y * TIME_STEP;
        }

        first = new_step.bodies[0];
        second = new_step.bodies[1];
        third = new_step.bodies[2];

        // report current state
        if n % 1000 == 0 {
            print!("({:.04}, {:.04})", first.position.x, first.position.y);
            print!(" ({:.04}, {:.04})", second.position.x, second.position.y);
            println!(" ({:.04}, {:.04})", third.position.x, third.position.y);
        }
    }
}
