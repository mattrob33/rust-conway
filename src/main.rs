extern crate rand;

use std::thread::sleep;
use std::time::Duration;
use std::env::args;
use std::io::stdin;
use rand::Rng;

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 30;
const NUM_ROUNDS: u16 = 500;
const DELAY_MS: u64 = 20;

fn main() {
    let mut map = create_map();

    for round in 0..NUM_ROUNDS {
        clear_screen();
        let old_map = map.clone();
        map = advance_round(map);
        print_screen(round, map);

        if is_same_as_previous_map(old_map, map) {
            println!("Game over");
            println!();
            break;
        }
        else {
            sleep(Duration::from_millis(DELAY_MS));
        }
    }
}

fn create_map() -> [[bool; MAP_WIDTH]; MAP_HEIGHT] {
    let mut map = [[false; MAP_WIDTH]; MAP_HEIGHT];

    for i in 0..MAP_WIDTH {
        for j in 0..MAP_HEIGHT {
            let num = rand::thread_rng().gen_range(0..5);
            map[j][i] = num == 0;
        }
    }

    map
}

fn advance_round(map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) -> [[bool; MAP_WIDTH]; MAP_HEIGHT] {
    let mut new_map = [[false; MAP_WIDTH]; MAP_HEIGHT];

    for i in 0..MAP_WIDTH {
        for j in 0..MAP_HEIGHT {
            new_map[j][i] = is_cell_alive_next_round(i, j, map);
        }
    }

    new_map
}

fn is_same_as_previous_map(old_map: [[bool; MAP_WIDTH]; MAP_HEIGHT], new_map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) -> bool {
    for i in 0..MAP_WIDTH {
        for j in 0..MAP_HEIGHT {
            if old_map[j][i] != new_map[j][i] { return false }
        }
    }

    return true;
}

/// Determines whether the cell at (x, y) should be alive next round. Conway's Game of Life
/// specifies this condition as follows:
///
///   1. Any live cell with two or three live neighbors survives. (survival)
///   2. Any dead cell with exactly three live neighbors becomes a live cell. (reproduction)
///   3. All other cells are dead in the next round. (under/over-population)
fn is_cell_alive_next_round(x: usize, y: usize, map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) -> bool {
    let neighbors = count_neighbors(x, y, map);

    return if map[y][x] {
        neighbors == 2 || neighbors == 3
    } else {
        neighbors == 3
    }
}

fn count_neighbors(x: usize, y: usize, map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) -> u8 {
    let mut num_neighbors: u8 = 0;

    if x > 0 && y > 0 {
        if is_live_cell(x - 1, y - 1, map) { num_neighbors += 1 }
    }

    if y > 0 {
        if is_live_cell(x, y - 1, map) { num_neighbors += 1 }
    }

    if x < MAP_WIDTH - 1 && y > 0 {
        if is_live_cell(x + 1, y - 1, map) { num_neighbors += 1 }
    }

    if x > 0 {
        if is_live_cell(x - 1, y, map) { num_neighbors += 1 }
    }

    if x < MAP_WIDTH - 1 {
        if is_live_cell(x + 1, y, map) { num_neighbors += 1 }
    }

    if x > 0 && y < MAP_HEIGHT - 1 {
        if is_live_cell(x - 1, y + 1, map) { num_neighbors += 1 }
    }

    if y < MAP_HEIGHT - 1 {
        if is_live_cell(x, y + 1, map) { num_neighbors += 1 }
    }

    if x < MAP_WIDTH - 1 && y < MAP_HEIGHT - 1 {
        if is_live_cell(x + 1, y + 1, map) { num_neighbors += 1 }
    }

    num_neighbors
}

fn is_live_cell(x: usize, y: usize, map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) -> bool {
    if x < 0 || x >= MAP_WIDTH {
        return false;
    }

    if y < 0 || y >= MAP_HEIGHT {
        return false;
    }

    return map[y][x];
}

fn print_screen(round: u16, map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) {
    print_header(round);
    print_map(map);
    println!();
}

fn print_header(round: u16) {
    println!("Conway's Game of Life");
    println!("Round {}", round + 1);
    println!();
}

fn print_map(map: [[bool; MAP_WIDTH]; MAP_HEIGHT]) {
    for i in 0..MAP_HEIGHT {
        print_map_line(map[i]);
    }
}

fn print_map_line(map_line: [bool; MAP_WIDTH]) {
    let mut line = String::new();
    for i in 0..MAP_WIDTH {
        let cell = if map_line[i] { 'x' } else { '.' };
        line.push(cell);
    }
    println!("{}", line);
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}