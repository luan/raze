extern crate raze;

use std::env;
use raze::maze;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (width, height): (i32, i32) = if args.len() >= 3 {
        (
            match args[1].parse() {
                Ok(n) => n,
                Err(_) => 5,
            },
            match args[2].parse() {
                Ok(n) => n,
                Err(_) => 5,
            },
        )
    } else {
        (5, 5)
    };

    let maze = maze::Maze::gen(width, height);

    for row in maze.tiles {
        for c in row {
            print!("{}", c.tile);
        }
        println!();
    }
}
