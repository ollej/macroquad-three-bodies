use macroquad::prelude::*;
use std::fmt;

const TIME_STEP: f64 = 0.05;
const STEPS: usize = 10000000;
const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;
const ANIMATION_FPS: u32 = 30;
const ANIMATION_LENGTH: u32 = 40;

type Position = DVec2;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn update(&mut self, time_step: f64) {
        self.position.x += self.velocity.x * time_step;
        self.position.y += self.velocity.y * time_step;
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.04}, {:.04})", self.position.x, self.position.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Step {
    time: f64,
    step: u32,
    bodies: [Body; 3],
}

impl Step {
    fn new(first: Body, second: Body, third: Body) -> Self {
        Step {
            time: 0.0,
            step: 0,
            bodies: [first, second, third],
        }
    }

    fn update(&mut self, time_step: f64) {
        self.calculate_step(time_step);
        self.bodies
            .iter_mut()
            .for_each(|body| body.update(time_step));
    }

    fn next_step(self, time_step: f64) -> Self {
        Step {
            time: self.time + time_step,
            step: self.step + 1,
            bodies: self.bodies,
        }
    }

    fn calculate_step(&mut self, time_step: f64) {
        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    self.calculate_bodies(i, j, time_step);
                }
            }
        }
    }

    fn calculate_bodies(&mut self, i: usize, j: usize, time_step: f64) {
        let a = &self.bodies[j];
        let mut b: Body = self.bodies[i];

        let dx = a.position.x - b.position.x;
        let dy: f64 = a.position.y - b.position.y;

        let r: f64 = (dx * dx + dy * dy).sqrt();
        let force = GRAVITATIONAL_CONSTANT * a.mass * b.mass / r / r;
        let angle = dy.atan2(dx);
        let fx = force * angle.cos();
        let fy = force * angle.sin();
        b.velocity.x += fx / b.mass * time_step;
        b.velocity.y += fy / b.mass * time_step;

        self.bodies[i] = b;
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Step:")?;
        for body in self.bodies.iter() {
            write!(f, " {}", body)?;
        }
        write!(f, "")
    }
}

fn simulate(mut step: Step, count: usize, time_step: f64) -> Vec<Step> {
    let mut steps = Vec::<Step>::with_capacity(count);

    for _ in 0..count {
        step.update(time_step);
        steps.push(step);
        step = step.next_step(time_step);
    }

    steps
}

#[macroquad::main("Three bodies")]
async fn main() {
    let first = Body::new(dvec2(0.3089693008, 0.4236727692));
    let second = Body::new(dvec2(-0.5, 0.0));
    let third = Body::new(dvec2(0.5, 0.0));

    let initial_step = Step::new(first, second, third);
    let steps = simulate(initial_step, STEPS, TIME_STEP);

    set_camera(&Camera2D::from_display_rect(Rect::new(
        -100., -100., 200., 200.,
    )));
    let steps_per_frame =
        (STEPS as f64 / (ANIMATION_LENGTH * ANIMATION_FPS) as f64).round() as usize;

    for step in steps.iter().step_by(steps_per_frame) {
        clear_background(WHITE);

        draw_circle(
            step.bodies[0].position.x as f32 * 100.,
            step.bodies[0].position.y as f32 * 100.,
            2.,
            RED,
        );
        draw_circle(
            step.bodies[1].position.x as f32 * 100.,
            step.bodies[1].position.y as f32 * 100.,
            2.,
            GREEN,
        );
        draw_circle(
            step.bodies[2].position.x as f32 * 100.,
            step.bodies[2].position.y as f32 * 100.,
            2.,
            BLUE,
        );
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate() {
        let first = Body::new(dvec2(0.3089693008, 0.4236727692));
        let second = Body::new(dvec2(-0.5, 0.0));
        let third = Body::new(dvec2(0.5, 0.0));

        let initial_step = Step::new(first, second, third);
        let steps = simulate(initial_step, 5, 0.5);

        assert_eq!(
            steps,
            vec![
                Step {
                    time: 0.0,
                    step: 0,
                    bodies: [
                        Body {
                            mass: 1.0,
                            position: dvec2(0.30896930081402885, 0.423672769120293),
                            velocity: dvec2(2.8057636600640765e-11, -1.5941407464626255e-10)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(-0.49999999996558936, 9.282862773097892e-12),
                            velocity: dvec2(6.882126950562697e-11, 1.8565725546195783e-11)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(0.4999999999515605, 7.042417455003339e-11),
                            velocity: dvec2(-9.687890610626773e-11, 1.4084834910006679e-10)
                        }
                    ]
                },
                Step {
                    time: 0.5,
                    step: 1,
                    bodies: [
                        Body {
                            mass: 1.0,
                            position: dvec2(0.30896930084208646, 0.4236727689608789),
                            velocity: dvec2(5.611527324112887e-11, -3.188281493901134e-10)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(-0.4999999998967681, 2.784858832017373e-11),
                            velocity: dvec2(1.3764253902280134e-10, 3.713145109415168e-11)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(0.49999999985468163, 2.1127252369801421e-10),
                            velocity: dvec2(-1.937578122639302e-10, 2.8169669829596167e-10)
                        }
                    ]
                },
                Step {
                    time: 1.0,
                    step: 2,
                    bodies: [
                        Body {
                            mass: 1.0,
                            position: dvec2(0.3089693008841729, 0.4236727687217578),
                            velocity: dvec2(8.417290996131172e-11, -4.782422243291407e-10)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(-0.49999999979353615, 5.569717664298761e-11),
                            velocity: dvec2(2.0646380856307043e-10, 5.569717664562777e-11)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(0.49999999970936326, 4.2254504753977065e-10),
                            velocity: dvec2(-2.906367185243821e-10, 4.225450476835129e-10)
                        }
                    ]
                },
                Step {
                    time: 1.5,
                    step: 3,
                    bodies: [
                        Body {
                            mass: 1.0,
                            position: dvec2(0.3089693009402882, 0.42367276840292967),
                            velocity: dvec2(1.1223054680103672e-10, -6.376562995609326e-10)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(-0.4999999996558936, 9.28286277441797e-11),
                            velocity: dvec2(2.752850781379816e-10, 7.426290220238417e-11)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(0.4999999995156055, 7.042417462190449e-10),
                            velocity: dvec2(-3.8751562493901825e-10, 5.633933973585484e-10)
                        }
                    ]
                },
                Step {
                    time: 2.0,
                    step: 4,
                    bodies: [
                        Body {
                            mass: 1.0,
                            position: dvec2(0.3089693010104323, 0.42367276800439446),
                            velocity: dvec2(1.402881838001512e-10, -7.970703751830775e-10)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(-0.49999999948384044, 1.3924294162727017e-10),
                            velocity: dvec2(3.4410634775908215e-10, 9.282862776618096e-11)
                        },
                        Body {
                            mass: 1.0,
                            position: dvec2(0.4999999992734082, 1.0563626199274932e-9),
                            velocity: dvec2(-4.843945315592333e-10, 7.042417474168966e-10)
                        }
                    ]
                }
            ]
        );
    }
}
