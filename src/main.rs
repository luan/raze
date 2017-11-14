extern crate rand;

use std::env;
use std::collections::HashSet;
use rand::Rng;

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

    let maze = generate_maze(width, height);

    for row in maze {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn generate_maze(width: i32, height: i32) -> Vec<Vec<char>> {
    let mut tile_sets = Vec::new();
    let mut tiles = Vec::new();
    let max_point = width * height;

    for y in 0..(2 * height + 1) {
        tiles.push(Vec::new());

        for x in 0..(2 * width + 1) {
            if x % 2 == 1 && y % 2 == 1 {
                let i = ((y - 1) / 2) * width + (x - 1) / 2;

                let mut set = HashSet::new();
                set.insert(i);
                tile_sets.push(set);
                tiles[y as usize].push(' ');
            } else {
                tiles[y as usize].push('#');
            }
        }
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    while tile_sets.len() > 1 {
        let a = rand::thread_rng().gen_range(0, max_point);
        let side = rand::thread_rng().gen_range(0, 3);

        let (diff_x, diff_y) = directions[side];
        let (x_a, y_a) = (a % width, a / width);
        let (x_b, y_b) = (x_a + diff_x, y_a + diff_y);
        if x_b < 0 || x_b >= width || y_b < 0 || y_b >= height {
            continue;
        }
        let b = y_b * width + x_b;

        let mut a_index = 0;
        let mut b_index = 0;

        for (i, set) in tile_sets.iter().cloned().enumerate() {
            if set.contains(&a) {
                a_index = i;
            }
            if set.contains(&b) {
                b_index = i;
            }
        }

        if a_index != b_index {
            let (x, y) = ((x_a * 2 + 1) + diff_x, (y_a * 2 + 1) + diff_y);
            tiles[y as usize][x as usize] = ' ';

            tile_sets[a_index] = &tile_sets[a_index] | &tile_sets[b_index];
            tile_sets.remove(b_index);
        }
    }

    let two: i32 = 2;
    for y in 0..tiles.len() {
        for x in 0..tiles[y].len() {
            if tiles[y][x] != ' ' {
                let mut walls = 0;
                for (i, &(xdiff, ydiff)) in directions.iter().enumerate() {
                    let (oy, ox) = (y as i32 + ydiff, x as i32 + xdiff);
                    if oy < 0 || oy >= 2 * height + 1 || ox < 0 || ox >= 2 * width + 1 {
                        continue;
                    }
                    if tiles[oy as usize][ox as usize] != ' ' {
                        walls += two.pow(i as u32);
                    }
                }

                tiles[y][x] = match walls {
                    1 => '╸',
                    2 => '╺',
                    3 => '━',
                    4 => '╹',
                    5 => '┛',
                    6 => '┗',
                    7 => '┻',
                    8 => '╻',
                    9 => '┓',
                    10 => '┏',
                    11 => '┳',
                    12 => '┃',
                    13 => '┫',
                    14 => '┣',
                    15 => '╋',
                    _ => '#',
                }
            }
        }
    }

    let goal = rand::thread_rng().gen_range(0, max_point);
    let (goal_x, goal_y) = (goal % width, goal / width);
    let goal_x = goal_x * 2 + 1;
    let goal_y = goal_y * 2 + 1;
    tiles[goal_y as usize][goal_x as usize] = '⚑';

    let start = rand::thread_rng().gen_range(0, max_point);
    let (start_x, start_y) = (start % width, start / width);
    let start_x = start_x * 2 + 1;
    let start_y = start_y * 2 + 1;
    tiles[start_y as usize][start_x as usize] = '⚉';

    tiles
}
