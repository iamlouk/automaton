
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
#[wasm_bindgen]
pub struct Simulation {
    width: f64,
    height: f64,
    ctx: web_sys::CanvasRenderingContext2d,
    
    particles: Vec<Particle>,
    k: f64, // the force constant (-G for gravity, 1/(2*tau*epsilon_0) for Coulomb's law)

    speed: f64,
}

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
    pub fn new(width: f64, height: f64, k: f64) -> Self {
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

        let mut particles: Vec<Particle> = Vec::new();
        let r = 5.0;
        let q = 1.0;
        let m = 1.0;
        let particle1 = Particle {
            radius: r,
            pos: Vector { x: -3.0*r, y: 0.0,},
            vel: Vector { x: 0.0, y: -1.0,},
            charge: q,
            mass: m,
        };
        let particle2 = Particle {
            radius: r,
            pos: Vector { x: 3.0*r, y: 0.0,},
            vel: Vector { x: 0.0, y: 1.0,},
            charge: -q,
            mass: m,
        };
        particles.push(particle1);
        particles.push(particle2);

        Simulation {
            width,
            height,
            ctx,
            particles,
            k,
            speed: 1.0,
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.ctx.clear_rect(0.0, 0.0, self.width, self.height);

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
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    log!("Hello Browser-Console!");
}
