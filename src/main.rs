mod world;

extern crate rand;

use std::thread::sleep;
use std::time::Duration;
use std::env::args;
use std::io::stdin;
use world::World;

const NUM_ROUNDS: u16 = 500;
const DELAY_MS: u64 = 20;

fn main() {
    let mut world = World::new(4, 4);

    for round in 0..NUM_ROUNDS {
        clear_screen();
        // world.advance_round();
        print_screen(round, &world);

        sleep(Duration::from_millis(DELAY_MS));
    }
}

fn print_screen(round: u16, world: &World) {
    print_header(round);
    world.print();
    println!();
}

fn print_header(round: u16) {
    println!("Conway's Game of Life");
    println!("Round {}", round + 1);
    println!();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}