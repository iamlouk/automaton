
use bincode;
use base64;
use serde::{Serialize, Deserialize};

mod vector;

use std::{u32, f64};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

pub use crate::vector::Vector;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn get_radius_from_mass(radius: f64) -> f64 {
    radius.powf(1./3.)
}

#[wasm_bindgen]
pub struct Simulation {
    width: f64,
    height: f64,
    ctx: web_sys::CanvasRenderingContext2d,
    
    particles: Vec<Particle>,
    k: f64, // the force constant (-G for gravity, 1/(2*tau*epsilon_0) for Coulomb's law)

    speed: f64,

    color_pos_charge: JsValue,
    color_neg_charge: JsValue
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Particle {
    radius: f64,
    pos: Vector,
    vel: Vector,
    charge: f64,
    mass: f64,
}

impl Particle {
    fn get_force_from(&self, other: &Particle, k: f64) -> Vector {
        let dpos = other.pos - self.pos;
        -k * self.charge * other.charge * dpos.normalized() / dpos.squared()
    }

    fn get_acc_from(&self, other: &Particle, k: f64) -> Vector {
        self.get_force_from(other, k) / self.mass
    }
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
        ctx.translate(width * 0.5, height * 0.5).unwrap();

        Simulation {
            width,
            height,
            ctx,
            particles: vec![],
            k: 1.0,
            speed: 1.0,
            color_pos_charge: JsValue::from_str("#ff3300"),
            color_neg_charge: JsValue::from_str("#0099ff")
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.ctx.clear_rect(-self.width * 0.5, -self.height * 0.5, self.width, self.height);

        for i in 0..self.particles.len() {
            // calc acceleration for particle i
            // by adding up the accelerations from each particle j!=i
            let mut acc = Vector { x: 0.0, y: 0.0,};
            for j in 0..self.particles.len() {
                if i == j { continue; } // particle can't influence itself
                acc += self.particles[i].get_acc_from(&self.particles[j], self.k);
            }

            // pos = 0.5*acc*dt^2 + vel*dt + pos_0
            // vel = acc*dt + vel_0
            acc *= dt;
            let vel = self.particles[i].vel;
            self.particles[i].pos += 0.5 * acc * dt + vel * dt;
            self.particles[i].vel += acc;

            self.render_particle(self.particles[i].pos,
                self.particles[i].radius, self.particles[i].charge);
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_force_constant(&mut self, k: f64) {
        self.k = k;
    }

    pub fn add_particle(&mut self, x: f64, y: f64, vel_x: f64, vel_y: f64, mass: f64, charge: f64) {
        // TODO
        let particle = Particle {
            pos: Vector { x, y },
            vel: Vector { x: vel_x, y: vel_y },
            mass,
            radius: get_radius_from_mass(mass),
            charge
        };

        self.particles.push(particle);
    }

    fn render_particle(&mut self, pos: Vector, radius: f64, charge: f64) {
        self.ctx.set_fill_style(if charge > 0.0 { &self.color_pos_charge } else { &self.color_neg_charge });

        self.ctx.begin_path();
        self.ctx.arc(pos.x, pos.y, radius, 0.0, 2.0 * f64::consts::PI).unwrap();
        self.ctx.fill();
    }

    pub fn decode_state_and_apply(&mut self, encoded: String) -> bool {
        self.particles.clear();
        match base64::decode_config(encoded, base64::URL_SAFE) {
            Ok(data) => match bincode::deserialize(&data) {
                Ok(particles) => {
                    self.particles = particles;
                    true
                },
                Err(e) => {
                    log!("error while decoding: {:?}", e);
                    false
                }
            },
            Err(e) => {
                log!("error while decoding: {:?}", e);
                false
            }
        }
    }

    pub fn encode_state(&mut self) -> String {
        let data: Vec<u8> = bincode::serialize(&self.particles).unwrap();
        base64::encode_config(data, base64::URL_SAFE)
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    log!("Hello Browser-Console!");
}
