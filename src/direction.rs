use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

static DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

impl Direction {
    pub fn pick() -> Direction {
        let side = thread_rng().gen_range(0, DIRECTIONS.len());
        DIRECTIONS[side]
    }

    pub fn diff(&self) -> (i32, i32) {
        match self {
            &Direction::North => (0, -1),
            &Direction::South => (0, 1),
            &Direction::East => (1, 0),
            &Direction::West => (-1, 0),
        }
    }
}
