use std::cell::{Cell, RefCell};
use rand::Rng;
use std::borrow::Borrow;

pub struct World {
    width: usize,
    height: usize,
    cells: Cell<Vec<Vec<bool>>>
}

impl World {

    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width,
            height,
            cells: Cell::new(World::create_cells(width, height))
        }
    }

    fn create_cells(width: usize, height: usize) -> Vec<Vec<bool>> {

        let mut cells = vec![vec![false; width]; height];

        for i in 0..width {
            for j in 0..height {
                let num = rand::thread_rng().gen_range(0..5);
                cells[j][i] = num == 0;
            }
        }

        cells
    }

    pub fn print(&self) {

        let cells = self.cells.get_mut();

        for i in 0..self.height {
            self.print_map_line(&cells[i]);
        }
    }

    fn print_map_line(&self, cell_row: &Vec<bool>) {
        let mut row = String::new();
        for i in 0..self.width {
            let cell = if cell_row[i] { 'x' } else { '.' };
            row.push(cell);
        }
        println!("{}", row);
    }

    pub fn advance_round(&self) {
        let mut new_cells = vec![vec![false; self.width]; self.height];

        for i in 0..self.width {
            for j in 0..self.height {
                new_cells[j][i] = self.is_cell_alive_next_round(i, j);
            }
        }

        self.cells.set(new_cells)
    }

    /// Determines whether the cell at (x, y) should be alive next round. Conway's Game of Life
    /// specifies this condition as follows:
    ///
    ///   1. Any live cell with two or three live neighbors survives. (survival)
    ///   2. Any dead cell with exactly three live neighbors becomes a live cell. (reproduction)
    ///   3. All other cells are dead in the next round. (under/over-population)
    fn is_cell_alive_next_round(&mut self, x: usize, y: usize) -> bool {
        let neighbors = self.count_neighbors(x, y);

        let cells = self.cells.get_mut();

        return if cells[y][x] {
            neighbors == 2 || neighbors == 3
        } else {
            neighbors == 3
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut num_neighbors: u8 = 0;

        if x > 0 && y > 0 {
            if self.is_live_cell(x - 1, y - 1) { num_neighbors += 1 }
        }

        if y > 0 {
            if self.is_live_cell(x, y - 1) { num_neighbors += 1 }
        }

        if x < self.width - 1 && y > 0 {
            if self.is_live_cell(x + 1, y - 1) { num_neighbors += 1 }
        }

        if x > 0 {
            if self.is_live_cell(x - 1, y) { num_neighbors += 1 }
        }

        if x < self.width - 1 {
            if self.is_live_cell(x + 1, y) { num_neighbors += 1 }
        }

        if x > 0 && y < self.height - 1 {
            if self.is_live_cell(x - 1, y + 1) { num_neighbors += 1 }
        }

        if y < self.height - 1 {
            if self.is_live_cell(x, y + 1) { num_neighbors += 1 }
        }

        if x < self.width - 1 && y < self.height - 1 {
            if self.is_live_cell(x + 1, y + 1) { num_neighbors += 1 }
        }

        num_neighbors
    }

    fn is_live_cell(&self, x: usize, y: usize) -> bool {
        if x < 0 || x >= self.width {
            return false;
        }

        if y < 0 || y >= self.height {
            return false;
        }

        let cells = self.cells.borrow().get();

        return cells[y][x];
    }
}