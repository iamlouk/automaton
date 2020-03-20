
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

    speed: f64
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

        Simulation {
            width,
            height,
            ctx,
            speed: 1.0
        }
    }

    pub fn step(&mut self, dt: f64) {
        self.ctx.clear_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed;
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    log!("Hello Browser-Console!");
}
