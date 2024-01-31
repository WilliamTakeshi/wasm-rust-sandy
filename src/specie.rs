use crate::Cell;
use crate::SandApi;
use crate::EMPTY_CELL;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
    Lava = 4,
    Stone = 5,
}

impl Species {
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Stone => {}
            Species::Sand => update_sand(cell, api),
            Species::Water => update_water(cell, api),
            Species::Lava => update_lava(cell, api),
        }
    }
}

pub fn update_sand(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir_2();

    let cell_below = api.get(0, 1);
    if cell_below.specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if cell_below.specie == Species::Water {
        api.set(0, 0, cell_below);
        api.set(0, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_water(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir_2();

    let cell_below = api.get(0, 1);
    if cell_below.specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }
}

pub fn update_lava(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir_2();

    let cell_below = api.get(0, 1);
    if cell_below.specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else if api.get(dx, 0).specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 0, cell);
    } else {
        api.set(0, 0, cell);
    }

    let (rand_dx, rand_dy) = api.rand_vec_8();
    let random_near_cell = api.get(rand_dx, rand_dy);
    if random_near_cell.specie == Species::Water {
        // If water and lava touch, both become stone
        api.set(
            0,
            0,
            Cell {
                specie: Species::Stone,
                ra: 0,
                rb: 0,
                gen: 0,
            },
        );
        api.set(
            rand_dx,
            rand_dy,
            Cell {
                specie: Species::Stone,
                ra: 0,
                rb: 0,
                gen: 0,
            },
        );
    }
}
