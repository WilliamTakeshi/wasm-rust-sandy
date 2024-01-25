// use super::utils::*;
use crate::Cell;
use crate::SandApi;
use crate::EMPTY_CELL;

// use std::cmp;
// use std::mem;
use wasm_bindgen::prelude::*;
// use web_sys::console;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
}

impl Species {
    pub fn update(&self, cell: Cell, api: SandApi) {
        match self {
            Species::Empty => {}
            Species::Wall => {}
            Species::Sand => update_sand(cell, api),
        }
    }
}

pub fn update_sand(cell: Cell, mut api: SandApi) {
    let dx = api.rand_dir_2();

    let nbr = api.get(0, 1);
    if nbr.specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(0, 1, cell);
    } else if api.get(dx, 1).specie == Species::Empty {
        api.set(0, 0, EMPTY_CELL);
        api.set(dx, 1, cell);
    } else {
        api.set(0, 0, cell);
    }
}
