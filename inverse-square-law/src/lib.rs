
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

    color_pos_charge: JsValue,
    color_neg_charge: JsValue
}

#[derive(Debug, Clone, Copy)]
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

        let mut particles: Vec<Particle> = Vec::new();
        let r = 5.0;
        let q = 1.0;
        let m = 1.0;
        let speed_0 = 0.1;
        let particle1 = Particle {
            radius: r,
            pos: Vector { x: 0.5*width - 3.0*r, y: 0.5*height,},
            vel: Vector { x: 0.0, y: -speed_0,},
            charge: q,
            mass: m,
        };
        let particle2 = Particle {
            radius: r,
            pos: Vector { x: 0.5*width + 3.0*r, y: 0.5*height,},
            vel: Vector { x: 0.0, y: speed_0,},
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
            k: 1.0,
            speed: 1.0,
            color_pos_charge: JsValue::from_str("#ff3300"),
            color_neg_charge: JsValue::from_str("#0099ff")
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

            let cur_particle = self.particles[i];
            self.render_particle(&cur_particle);
        }
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }

    pub fn set_electrostatic_constant(&mut self, k: f64) {
        self.k = k;
    }

    pub fn add_particle(&mut self, x: f64, y: f64, mass: f64, radius: f64, charge: f64) {
        // TODO
        let particle = Particle {
            pos: Vector { x, y },
            vel: Vector { x: 0.0, y: 0.0 },
            mass,
            radius,
            charge
        };

        log!("new particle: {:?}", particle);
    }

    fn render_particle(&mut self, particle: &Particle) {
        self.ctx.set_fill_style(if particle.charge > 0.0 { &self.color_pos_charge } else { &self.color_neg_charge });

        self.ctx.begin_path();
        self.ctx.arc(particle.pos.x, particle.pos.y, particle.radius, 0.0, 2.0 * f64::consts::PI).unwrap();
        self.ctx.fill();
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    log!("Hello Browser-Console!");
}
