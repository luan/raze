use rand::{thread_rng, Rng};
use std::collections::HashSet;
use direction::Direction;

struct Point(i32, i32);

impl Point {
    fn add(&self, direction: Direction) -> Point {
        let diff = direction.diff();
        Point(self.0 + diff.0, self.1 + diff.1)
    }
}

pub struct Tile {
    pub tile: char,

    walkable: bool,
}

pub struct Maze {
    pub tiles: Vec<Vec<Tile>>,

    height: i32,
    width: i32,
}

impl Maze {
    pub fn gen(width: i32, height: i32) -> Maze {
        let mut tile_sets = Vec::new();
        let mut maze = Maze {
            tiles: Vec::new(),
            height: height * 2 + 1,
            width: width * 2 + 1,
        };
        let max_point = width * height;

        for y in 0..(2 * height + 1) {
            maze.tiles.push(Vec::new());

            for x in 0..(2 * width + 1) {
                if x % 2 == 1 && y % 2 == 1 {
                    let i = ((y - 1) / 2) * width + (x - 1) / 2;

                    let mut set = HashSet::new();
                    set.insert(i);
                    tile_sets.push(set);
                    maze.tiles[y as usize].push(Tile {
                        tile: ' ',
                        walkable: true,
                    });
                } else {
                    maze.tiles[y as usize].push(Tile {
                        tile: '#',
                        walkable: false,
                    });
                }
            }
        }

        while tile_sets.len() > 1 {
            let a = thread_rng().gen_range(0, max_point);

            let (diff_x, diff_y) = Direction::pick().diff();
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
                maze.tiles[y as usize][x as usize] = Tile {
                    tile: ' ',
                    walkable: true,
                };

                tile_sets[a_index] = &tile_sets[a_index] | &tile_sets[b_index];
                tile_sets.remove(b_index);
            }
        }

        for y in 0..maze.tiles.len() {
            for x in 0..maze.tiles[y].len() {
                if !maze.tiles[y][x].walkable {
                    let point = Point(x as i32, y as i32);
                    let walls = (
                        maze.wall_at(point.add(Direction::North)),
                        maze.wall_at(point.add(Direction::South)),
                        maze.wall_at(point.add(Direction::East)),
                        maze.wall_at(point.add(Direction::West)),
                    );

                    maze.tiles[y][x].tile = match walls {
                        (false, false, false, false) => 'E',
                        (false, false, false, true) => '╸',
                        (false, false, true, false) => '╺',
                        (false, false, true, true) => '━',
                        (false, true, false, false) => '╻',
                        (false, true, false, true) => '┓',
                        (false, true, true, false) => '┏',
                        (false, true, true, true) => '┳',
                        (true, false, false, false) => '╹',
                        (true, false, false, true) => '┛',
                        (true, false, true, false) => '┗',
                        (true, false, true, true) => '┻',
                        (true, true, false, false) => '┃',
                        (true, true, false, true) => '┫',
                        (true, true, true, false) => '┣',
                        (true, true, true, true) => '╋',
                    }
                }
            }
        }

        let goal = thread_rng().gen_range(0, max_point);
        let (goal_x, goal_y) = (goal % width, goal / width);
        let goal_x = goal_x * 2 + 1;
        let goal_y = goal_y * 2 + 1;
        maze.tiles[goal_y as usize][goal_x as usize].tile = '⚑';

        let start = thread_rng().gen_range(0, max_point);
        let (start_x, start_y) = (start % width, start / width);
        let start_x = start_x * 2 + 1;
        let start_y = start_y * 2 + 1;
        maze.tiles[start_y as usize][start_x as usize].tile = '⚉';

        maze
    }

    fn wall_at(&self, point: Point) -> bool {
        point.0 >= 0 && point.1 >= 0 && point.0 < self.width && point.1 < self.height
            && !self.tiles[point.1 as usize][point.0 as usize].walkable
    }
}
