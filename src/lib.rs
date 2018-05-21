#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[allow(dead_code)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(msg: &str);

    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[allow(dead_code)]
macro_rules! log {
    ($($t:tt)*) => {}; // ($($t: tt)*) => (log(&format!($($t)*)))
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: [Vec<Cell>; 2],
    cells_idx: usize,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(mut width: u32, mut height: u32) -> Self {
        if width % 8 != 0 || height % 8 != 0 {
            alert("Width and height of the universe must be multiple of 8!");
            width = width / 8 * 8;
            height = height / 8 * 8;
        }

        let cells = (0..width * height)
            .map(|_| {
                if random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        let cells_back = vec![Cell::Alive; (width * height) as usize];

        Universe {
            width,
            height,
            cells: [cells, cells_back],
            cells_idx: 0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells_ptr(&self) -> *const Cell {
        self.cells().as_ptr()
    }

    pub fn tick(&mut self) {
        let new_cells_idx = self.cells_idx ^ 1;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells()[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, x) if x < 4 => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                log!("   it becomes {:?}", next_cell);

                self.cells[new_cells_idx][idx] = next_cell;
            }
        }

        self.cells_idx = new_cells_idx;
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells_mut()[idx].toggle();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn cells(&self) -> &[Cell] {
        self.cells[self.cells_idx].as_slice()
    }

    fn cells_mut(&mut self) -> &mut [Cell] {
        self.cells[self.cells_idx].as_mut_slice()
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let mut count = 0;

        let nw = self.get_index(north, west);
        count += self.cells()[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells()[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells()[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells()[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells()[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells()[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells()[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells()[se] as u8;

        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {
                    "◼️"
                } else {
                    "◻️"
                };
                write!(f, "{}", symbol).unwrap();
            }

            writeln!(f).unwrap();
        }

        Ok(())
    }
}
