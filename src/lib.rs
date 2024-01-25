mod specie;
mod utils;
extern crate rand_xoshiro;

use rand::{Rng, SeedableRng};
use rand_xoshiro::SplitMix64;
use specie::Species;
use wasm_bindgen::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    specie: Species,
    ra: u8,
    rb: u8,
    clock: u8,
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

impl Cell {
    pub fn new(specie: Species) -> Self {
        Self {
            specie: specie,
            ra: 0,
            rb: 0,
            clock: 0,
        }
    }

    pub fn update(&self, api: SandApi) {
        self.specie.update(*self, api);
    }
}

static EMPTY_CELL: Cell = Cell {
    specie: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

#[wasm_bindgen]
pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    rng: SplitMix64,
}

impl Universe {
    fn get_index(&self, row: i32, column: i32) -> usize {
        (column * self.height + row) as usize
        // (row * self.width + column) as usize
    }

    fn get_cell(&self, row: i32, column: i32) -> Cell {
        let idx = self.get_index(row, column);
        self.cells[idx]
    }

    fn update_cell(cell: Cell, api: SandApi) {
        // if cell.clock - api.universe.generation == 1 {
        //     return;
        // }
        cell.update(api);
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        // let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                Universe::update_cell(
                    cell,
                    SandApi {
                        universe: self,
                        x: row,
                        y: col,
                    },
                );
            }
        }

        // self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 100;
        let height = 100;

        let cells = (0..width * height)
            .map(|_| {
                Cell::new(Species::Empty)
            })
            .collect();
        let rng: SplitMix64 = SeedableRng::seed_from_u64(0x797867893cc);

        Universe {
            width,
            height,
            cells,
            rng,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn paint(&mut self, x: i32, y: i32, size: i32, specie: Species) {
        let radius: f64 = (size as f64) / 2.0;

        let floor = radius as i32;
        let ceil = radius as i32;

        for dx in -floor..ceil {
            for dy in -floor..ceil {
                if (((dx * dx) + (dy * dy)) as f64) > (radius * radius) {
                    continue; // ignore outside the circle
                };
                let px = x + dx;
                let py = y + dy;
                let i = self.get_index(px, py);

                if px < 0 || px > self.width - 1 || py < 0 || py > self.height - 1 {
                    continue;
                }
                if self.get_cell(px, py).specie == Species::Empty || specie == Species::Empty {
                    self.cells[i] = Cell {
                        specie: specie,
                        ra: 0,
                        rb: 0,
                        clock: 0,
                    }
                }
            }
        }
    }
}

pub struct SandApi<'a> {
    x: i32,
    y: i32,
    universe: &'a mut Universe,
}

impl<'a> SandApi<'a> {
    pub fn get(&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return Cell {
                specie: Species::Wall,
                ra: 0,
                rb: 0,
                clock: 0,
            };
        }
        self.universe.get_cell(nx, ny)
    }

    pub fn set(&mut self, dx: i32, dy: i32, v: Cell) {
        if dx > 2 || dx < -2 || dy > 2 || dy < -2 {
            panic!("oob set");
        }
        let nx = self.x + dx;
        let ny = self.y + dy;

        if nx < 0 || nx > self.universe.width - 1 || ny < 0 || ny > self.universe.height - 1 {
            return;
        }
        let i = self.universe.get_index(nx, ny);
        self.universe.cells[i] = v;
        self.universe.cells[i].clock = 0;
    }

    pub fn rand_int(&mut self, n: i32) -> i32 {
        self.universe.rng.gen_range(0..n)
    }

    pub fn once_in(&mut self, n: i32) -> bool {
        self.rand_int(n) == 0
    }
    pub fn rand_dir(&mut self) -> i32 {
        let i = self.rand_int(1000);
        (i % 3) - 1
    }
    pub fn rand_dir_2(&mut self) -> i32 {
        let i = self.rand_int(1000);
        if (i % 2) == 0 {
            -1
        } else {
            1
        }
    }

    pub fn rand_vec(&mut self) -> (i32, i32) {
        let i = self.rand_int(2000);
        match i % 9 {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            7 => (0, 1),
            _ => (0, 0),
        }
    }

    pub fn rand_vec_8(&mut self) -> (i32, i32) {
        let i = self.rand_int(8);
        match i {
            0 => (1, 1),
            1 => (1, 0),
            2 => (1, -1),
            3 => (0, -1),
            4 => (-1, -1),
            5 => (-1, 0),
            6 => (-1, 1),
            _ => (0, 1),
        }
    }
}
