use crate::interface::Display;
use crate::model::{Direction, Robot, World};

/* ╋ ━ ┏ ┗ ┓ ┛ ┳ ┻ ┫ ┣ */

use std::time::Duration;

const PAUSE: Duration = Duration::from_millis(100);

fn draw_world<'a>(w: &World, bots: impl Iterator<Item = &'a Robot>) {
    use nu_ansi_term::{Color, Style};

    let robots: Vec<_> = bots.into_iter().collect();

    fn draw_walls(w: &World, y: usize, dir: Direction) {
        for x in 0..w.width() {
            let walls = w.walls(y, x);
            if walls.has(dir) {
                print!("####");
            } else {
                if walls.has(Direction::West) {
                    print!("#");
                } else {
                    print!(" ");
                }
                print!("  ");
                if walls.has(Direction::East) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }

    fn bot_icon(dir: Direction) -> &'static str {
        match dir {
            Direction::North => "⬆️",
            Direction::South => "⬇️",
            Direction::West => "⬅️",
            Direction::East => "➡️",
        }
    }

    for y in 0..w.height() {
        draw_walls(w, y, Direction::North);
        for x in 0..w.width() {
            print!(
                "{}",
                if w.walls(y, x).has(Direction::West) {
                    "#"
                } else {
                    " "
                }
            );
            print!(
                "{}",
                robots
                    .iter()
                    .find_map(|bot| if bot.pos == (y, x) {
                        let mut bot = bot_icon(bot.dir).to_owned();
                        bot.push(' ');
                        Some(if w.has_shell(y, x) {
                            Style::new().on(Color::Red).paint(bot).to_string()
                        } else {
                            bot
                        })
                    } else {
                        None
                    })
                    .unwrap_or_else(|| {
                        if w.has_shell(y, x) {
                            "🦀".to_string()
                        } else {
                            "· ".to_string()
                        }
                    })
            );
            print!(
                "{}",
                if w.walls(y, x).has(Direction::East) {
                    "#"
                } else {
                    " "
                }
            );
        }
        println!();
    }

    draw_walls(w, w.height() - 1, Direction::South);
}

pub struct TTYView;

pub fn new() -> TTYView {
    use crossterm::{cursor, terminal};
    crossterm::execute! {
        std::io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0,0),
        cursor::Hide,
    }
    .unwrap();

    TTYView
}

impl Display for TTYView {
    fn draw(&self, w: &World, bots: &mut dyn Iterator<Item = &Robot>) {
        crossterm::execute! {
            std::io::stdout(),
            crossterm::cursor::MoveTo(0,0),
        }
        .unwrap();

        draw_world(w, bots);

        std::thread::sleep(PAUSE);
    }
}

impl Drop for TTYView {
    fn drop(&mut self) {
        crossterm::execute! {
            std::io::stdout(),
            crossterm::cursor::Show,
        }
        .unwrap()
    }
}
