
mod vector;

use std::{u32, f64};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

pub use crate::vector::Vector;

const INDIVIDUAL_RADIUS: f64 = 7.5;
const DEATH_RATE: f64 = 0.0;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Healthy,
    Infected,
    Cured,
    Dead,
}

#[wasm_bindgen]
//#[derive(Clone, Debug, PartialEq)]
pub struct Individual {
    pos: Vector,
    vel: Vector,
    state: State,
    infected_for: f64,
}

impl Individual {
    fn update_position(&mut self, dt: f64, width: f64, height: f64) {
        if self.state == State::Dead {
            return;
        }

        self.pos += self.vel * dt;

        let x_too_small = self.pos.x < INDIVIDUAL_RADIUS;
        let x_too_big = self.pos.x + INDIVIDUAL_RADIUS > width;
        let y_too_small = self.pos.y < INDIVIDUAL_RADIUS;
        let y_too_big = self.pos.y + INDIVIDUAL_RADIUS >= height;

        if x_too_small || x_too_big {
            self.vel.x *= -1.0;

            if x_too_small {
                self.pos.x = INDIVIDUAL_RADIUS;
            }
            if x_too_big {
                self.pos.x = width - INDIVIDUAL_RADIUS;
            }
        }
        if y_too_small || y_too_big {
            self.vel.y *= -1.0;

            if y_too_small {
                self.pos.y = INDIVIDUAL_RADIUS;
            }
            if y_too_big {
                self.pos.y = height - INDIVIDUAL_RADIUS;
            }
        }
    }

    fn touches(&self, other: &Individual) -> bool {
        (self.pos - other.pos).mag() < INDIVIDUAL_RADIUS * 2.0
    }
}

#[wasm_bindgen]
pub struct Simulation {
    width: f64,
    height: f64,
    individuals: Vec<Individual>,
    ctx: web_sys::CanvasRenderingContext2d,

    time_to_heal: f64,

    color_healthy: JsValue,
    color_infected: JsValue,
    color_cured: JsValue,
    color_dead: JsValue,
    color_text: JsValue
}

fn random_normalized_vec() -> Vector {
    let x: f64 = js_sys::Math::random() * 2.0 - 1.0;
    let y: f64 = js_sys::Math::random() * 2.0 - 1.0;

    let result = Vector { x, y, };
    result.normalized()
}

fn find_partition(n: i32) -> (usize, usize) {
    (n as usize, n as usize)
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(width: f64, height: f64, time_to_heal: f64, scale: f64, population: i32) -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        
        ctx.set_font("20px monospace");

        let (rows, cols) = find_partition(population);
        let mut individuals = Vec::with_capacity(rows * cols);

        for row in 0..rows {
            for col in 0..cols {
                let vel = random_normalized_vec();

                let state = if row == rows / 2 && col == cols / 2 {
                    State::Infected
                } else {
                    State::Healthy
                };

                let pos = Vector {
                    x: 50.0 + (width  - 100.0) * (col as f64 / cols as f64),
                    y: 50.0 + (height - 100.0) * (row as f64 / rows as f64),
                };

                individuals.push(Individual {
                    pos,
                    vel,
                    state,
                    infected_for: 0.0
                })
            }
        }

        Simulation {
            width,
            height,
            individuals,
            ctx,

            time_to_heal,

            color_cured: JsValue::from_str("#00ff00"),
            color_dead: JsValue::from_str("#000000"),
            color_healthy: JsValue::from_str("#0099ff"),
            color_infected: JsValue::from_str("#ff3300"),
            color_text: JsValue::from_str("#555555")
        }
    }

    pub fn step(&mut self, dt: f64) -> Vec<i32> {
        let mut total_healthy = 0;
        let mut total_infected = 0;
        let mut total_cured = 0;

        for individual in &mut self.individuals {
            individual.update_position(dt, self.width, self.height);
            if individual.state == State::Infected {
                individual.infected_for += dt;
                if individual.infected_for >= self.time_to_heal {
                    let r = js_sys::Math::random();
                    if r <= DEATH_RATE {
                        individual.state = State::Dead;
                    } else {
                        individual.state = State::Cured;
                    }
                }
            }
        }

        for i in 0..self.individuals.len() {
            for j in 0..i {
                if self.individuals[i].touches(&self.individuals[j]) {
                    let state_a = self.individuals[i].state;
                    let state_b = self.individuals[j].state;
                    if (state_a == State::Infected || state_b == State::Infected) && (state_a == State::Healthy || state_b == State::Healthy) {
                        self.individuals[i].state = State::Infected;
                        self.individuals[j].state = State::Infected;
                    }

                    // elastic collision
                    if state_a != State::Dead && state_b != State::Dead {
                        let e = (self.individuals[j].pos - self.individuals[i].pos).normalized();

                        // split velocities into parallel and perpendicular portions
                        let a_par = self.individuals[i].vel.dot(&e); // parallel scalar
                        self.individuals[i].vel -= e * a_par; // perpendicular vector
                        let b_par = self.individuals[j].vel.dot(&e); // parallel scalar
                        self.individuals[j].vel -= e * b_par; // perpendicular vector
                       
                        // simply switch the parralel parts (because all masses are equal)
                        // and assemble the new velocity vectors
                        self.individuals[i].vel += e * b_par;
                        self.individuals[j].vel += e * a_par;
                    } else {
                        // TODO: dead people are ghosts
                    }

                    self.individuals[i].update_position(dt, self.width, self.height);
                    self.individuals[j].update_position(dt, self.width, self.height);
                }
            }
        }

        self.ctx.clear_rect(0.0, 0.0, self.width, self.height);

        for individual in &self.individuals {
            self.ctx.set_fill_style(match individual.state {
                State::Healthy => {
                    total_healthy += 1;
                    &self.color_healthy
                },
                State::Infected => {
                    total_infected += 1;
                    &self.color_infected
                },
                State::Cured => {
                    total_cured += 1;
                    &self.color_cured
                },
                State::Dead => {
                    &self.color_dead
                }
            });

            self.ctx.begin_path();
            self.ctx.arc(individual.pos.x, individual.pos.y, INDIVIDUAL_RADIUS, 0.0, 2.0 * f64::consts::PI).unwrap();
            self.ctx.fill();
        }

        let text_x = 10.0;
        let text_y = self.height - 70.0;
        let text = format!("#healthy: {}, #infected: {}, #cured: {}", total_healthy, total_infected, total_cured);
        self.ctx.set_fill_style(&self.color_text);
        self.ctx.fill_text(text.as_ref(), text_x, text_y).unwrap();

        vec![total_healthy, total_infected, total_cured]
    }

}

#[wasm_bindgen(start)]
pub fn start() {}
