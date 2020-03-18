
use std::{u32, f64};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

const INDIVIDUAL_RADIUS: f64 = 7.5;
const TIME_TO_HEAL: i32 = 500;
const POPULATION_DENSITY: f64 = 15.0;
const SPEED: f64 = 2.5;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Healthy,
    Infected,
    Cured,
    Dead
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Individual {
    x: f64,
    y: f64,
    movement_x: f64,
    movement_y: f64,
    state: State,
    infected_for: i32
}

impl Individual {
    fn update_position(&mut self, width: f64, height: f64) {
        self.x += self.movement_x;
        self.y += self.movement_y;

        if self.x < INDIVIDUAL_RADIUS || self.x + INDIVIDUAL_RADIUS >= width {
            self.movement_x *= -1.;
            self.x += self.movement_x;
        }

        if self.y < INDIVIDUAL_RADIUS || self.y + INDIVIDUAL_RADIUS >= height {
            self.movement_y *= -1.;
            self.y += self.movement_y;
        }
    }

    fn toches(&self, other: &Individual) -> bool {
        let d_x = self.x - other.x;
        let d_y = self.y - other.y;
        let distance = (d_x*d_x + d_y*d_y).sqrt();

        distance < INDIVIDUAL_RADIUS * 2.
    }
}

#[wasm_bindgen]
pub struct Simulation {
    width: f64,
    height: f64,
    individuals: Vec<Individual>,
    ctx: web_sys::CanvasRenderingContext2d,

    color_healthy: JsValue,
    color_infected: JsValue,
    color_cured: JsValue,
    color_dead: JsValue,
    color_text: JsValue
}

fn random_normalized_vec() -> (f64, f64) {
    let x: f64 = js_sys::Math::random() * 2.0 - 1.0;
    let y: f64 = js_sys::Math::random() * 2.0 - 1.0;

    let len = (x*x + y*y).sqrt() * (1.0 / SPEED);

    (x / len, y / len)
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(width: f64, height: f64) -> Self {
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

        let rows = (height / INDIVIDUAL_RADIUS / POPULATION_DENSITY) as usize;
        let cols = (width / INDIVIDUAL_RADIUS / POPULATION_DENSITY) as usize;
        let mut individuals = Vec::with_capacity(rows * cols);
        for row in 0..rows {
            for col in 0..cols {
                let (movement_x, movement_y) = random_normalized_vec();

                let state = if row == rows / 2 && col == cols / 2 {
                    State::Infected
                } else {
                    State::Healthy
                };

                let x = 50.0 + (width - 100.0) * (row as f64 / rows as f64);
                let y = 50.0 + (height - 100.0) * (col as f64 / cols as f64);

                individuals.push(Individual {
                    x, y,
                    movement_x,
                    movement_y,
                    state,
                    infected_for: 0
                })
            }
        }

        Simulation {
            width,
            height,
            individuals,
            ctx,

            color_cured: JsValue::from_str("#00ff00"),
            color_dead: JsValue::from_str("#000000"),
            color_healthy: JsValue::from_str("#0099ff"),
            color_infected: JsValue::from_str("#ff3300"),
            color_text: JsValue::from_str("#555555")
        }
    }

    pub fn step(&mut self) -> Vec<i32> {
        let mut total_healthy = 0;
        let mut total_infected = 0;
        let mut total_cured = 0;
        let mut total_dead = 0;

        for individual in &mut self.individuals {
            individual.update_position(self.width, self.height);
            if individual.state == State::Infected {
                individual.infected_for += 1;
                if individual.infected_for == TIME_TO_HEAL {
                    individual.state = State::Cured;
                }
            }
        }

        for i in 0..self.individuals.len() {
            for j in 0..i {
                if self.individuals[i].toches(&self.individuals[j]) {
                    let state_a = self.individuals[i].state;
                    let state_b = self.individuals[j].state;

                    if (state_a == State::Infected || state_b == State::Infected) && !(state_a == State::Cured || state_b == State::Cured) {
                        self.individuals[i].state = State::Infected;
                        self.individuals[j].state = State::Infected;
                    }

                    let tmp_x = self.individuals[i].movement_x;
                    let tmp_y = self.individuals[i].movement_y;
                    self.individuals[i].movement_x = self.individuals[j].movement_x;
                    self.individuals[i].movement_y = self.individuals[j].movement_y;
                    self.individuals[j].movement_x = tmp_x;
                    self.individuals[j].movement_y = tmp_y;

                    self.individuals[i].update_position(self.width, self.height);
                    self.individuals[j].update_position(self.width, self.height);
                }
            }
        }

        self.ctx.clear_rect(0., 0., self.width, self.height);

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
                    total_dead += 1;
                    &self.color_dead
                }
            });

            self.ctx.begin_path();
            self.ctx.arc(individual.x, individual.y, INDIVIDUAL_RADIUS, 0.0, 2.0 * f64::consts::PI).unwrap();
            self.ctx.fill();
        }

        let text_x = 10.;
        let text_y = self.height - 30.0;
        let text = format!("#healthy: {}, #infected: {}, #cured: {}", total_healthy, total_infected, total_cured);
        self.ctx.set_fill_style(&self.color_text);
        self.ctx.fill_text(text.as_ref(), text_x, text_y).unwrap();

        vec![total_healthy, total_infected, total_cured, total_dead]
    }

}
