extern crate raze;
extern crate termion;

use std::env;
use raze::maze::{Maze, Point};
use raze::direction::*;
use termion::{clear, color};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{stdout, Write};
use std::{thread, time};

macro_rules! show_maze {
    ($w:expr, $maze:expr) => {
        for (y, row) in (&$maze.tiles).iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                let pos = Point(x as i32, y as i32);
                if pos == $maze.player_pos  {
                    write!($w, "{}⚉{}", color::Fg(color::Cyan), color::Fg(color::Reset)).unwrap();
                } else if pos == $maze.goal_pos {
                    write!($w, "{}⚑{}", color::Fg(color::Red), color::Fg(color::Reset)).unwrap();
                } else {
                    write!($w, "{}{}{}", color::Fg(color::White), c.tile, color::Fg(color::Reset)).unwrap();
                }
            }
            write!($w, "\n\r").unwrap();
        }
        $w.flush().unwrap();
    };
}

fn game(mut maze: Maze) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();
    write!(stdout, "{}{}", termion::cursor::Hide, clear::All).unwrap();
    let mut win = false;

    loop {
        write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();
        stdout.flush().unwrap();

        let c = stdin.next();
        if let Some(Ok(key)) = c {
            match key {
                Key::Char('q') => break,
                Key::Char('a') => maze.walk(Direction::West),
                Key::Char('d') => maze.walk(Direction::East),
                Key::Char('w') => maze.walk(Direction::North),
                Key::Char('s') => maze.walk(Direction::South),
                _ => (),
            }
        };

        if maze.player_pos == maze.goal_pos {
            win = true;
            break;
        }
        show_maze!(stdout, maze);
        stdout.flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }

    write!(stdout, "{}{}", termion::cursor::Show, clear::All).unwrap();

    if win {
        write!(stdout, "Congratulations, you solved the maze!").unwrap();
    }
}


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

    let maze = Maze::gen(width, height);
    game(maze);
}
