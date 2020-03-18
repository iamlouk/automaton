
use std::{u32, f64};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

const INFECTION_ON_CONTACT_RATE: f64 = 0.1;
const DEATH_RATE: f64 = 0.005;
const CURED_RATE: f64 = 0.05;
const CELL_WIDTH: f64 = 10.;
const CELL_HEIGHT: f64 = 10.;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Healthy,
    Infected,
    Cured,
    Dead,
    None
}

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    new_cells: Vec<Cell>,
    ctx: web_sys::CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Universe {

    fn new(width: i32, height: i32, ctx: web_sys::CanvasRenderingContext2d) -> Self {
        let size = (width * height) as usize;
        let mut universe = Universe {
            width,
            height,
            cells: vec![Cell::Healthy; size],
            new_cells: vec![Cell::None; size],
            ctx
        };

        let idx = universe.get_index(width / 2, height / 2);
        universe.cells[idx] = Cell::Infected;

        universe
    }

    /*
    fn random_pos(&self) -> (i32, i32) {
        let a = js_sys::Math::random();
        let b = js_sys::Math::random();
        ((a * self.height as f64) as i32, (b * self.width as f64)  as i32)
    }
    */

    fn get_index(&self, row: i32, col: i32) -> usize {
        (row * self.width + col) as usize
    }

    fn get_neighbour_cells(&self, row: i32, col: i32) -> [Cell; 8] {
        let wrappos = |pos, offset, max| match (pos, offset) {
            (0,  -1)                   => max - 1,
            (pos, 1) if pos == max - 1 => 0,
            (pos, offset)              => pos + offset
        };

        let getcell = |di, dj| self.cells[self.get_index(wrappos(row, di, self.height), wrappos(col, dj, self.width))];

        [
            getcell(-1, -1), getcell(-1,  0), getcell(-1,  1),
            getcell( 0, -1),                  getcell( 0,  1),
            getcell( 1, -1), getcell( 1,  0), getcell( 1,  1)
        ]
    }

    fn get_infected_neighbours(&self, row: i32, col: i32) -> usize {
        let mut infected = 0;

        for cell in self.get_neighbour_cells(row, col).iter() {
            if cell == &Cell::Infected {
                infected += 1;
            }
        }

        infected
    }

    pub fn update(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let r = js_sys::Math::random();
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                if cell == Cell::Healthy {
                    let infected_neighbours = self.get_infected_neighbours(row, col);
                    if r <= infected_neighbours as f64 * INFECTION_ON_CONTACT_RATE {
                        self.new_cells[idx] = Cell::Infected;
                        continue;
                    }
                }

                if cell == Cell::Infected {
                    if r <= DEATH_RATE {
                        self.new_cells[idx] = Cell::Dead;
                        continue;
                    }

                    if r <= CURED_RATE {
                        self.new_cells[idx] = Cell::Cured;
                        continue;
                    }
                }

                /*
                if r >= 0.95 && cell == Cell::Infected {
                    let r = (r * 10000) as u64;
                    let (drow, dcol) = match r & 0b11 {
                        0b00 => ( 0,  1),
                        0b01 => ( 0, -1),
                        0b10 => ( 1,  0),
                        0b11 => (-1,  0),
                        _ => panic!()
                    };

                    let npos = match (row + drow, col + dcol) {
                        (
                    };
                }
                */

                self.new_cells[idx] = cell;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.new_cells);

        /*
        for row in 0..self.height {
            for col in 0..self.width {
                let r = js_sys::Math::random();
                if r < 0.05 {
                    let (nrow, ncol) = self.random_pos();
                    let idx1 = self.get_index(row, col);
                    let idx2 = self.get_index(nrow, ncol);

                    let tmp = self.cells[idx1];
                    self.cells[idx1] = self.cells[idx2];
                    self.cells[idx2] = tmp;
                }
            }
        }
        */
    }

    pub fn tick(&mut self) -> Vec<i32> {
        let color_cured    = JsValue::from_str("#00ff00");
        let color_dead     = JsValue::from_str("#000000");
        let color_healthy  = JsValue::from_str("#0099ff");
        let color_infected = JsValue::from_str("#ff3300");

        let mut healthy = 0;
        let mut infected = 0;
        let mut cured = 0;
        let mut dead = 0;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                if self.new_cells[idx] != cell {
                    self.ctx.set_fill_style(match cell {
                        Cell::Cured    => &color_cured,
                        Cell::Dead     => &color_dead,
                        Cell::Healthy  => &color_healthy,
                        Cell::Infected => &color_infected,
                        Cell::None     => panic!()
                    });

                    self.ctx.fill_rect(col as f64 * CELL_WIDTH, row as f64 * CELL_HEIGHT, CELL_WIDTH, CELL_HEIGHT);
                }

                match cell {
                    Cell::Cured    => cured += 1,
                    Cell::Dead     => dead += 1,
                    Cell::Healthy  => healthy += 1,
                    Cell::Infected => infected += 1,
                    Cell::None     => panic!()
                }
            }
        }

        vec![healthy, infected, cured, dead]
    }

}

#[wasm_bindgen]
pub fn get_universe() -> Universe {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let universe = Universe::new(75, 75, context);

    universe
}

#[wasm_bindgen(start)]
pub fn start() {}

