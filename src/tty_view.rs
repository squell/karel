use crate::model::{Direction, Robot, World};

/* ╋ ━ ┏ ┗ ┓ ┛ ┳ ┻ ┫ ┣ */

fn draw_world<'a>(w: &World, bots: impl IntoIterator<Item = &'a Robot>) {
    use nu_ansi_term::{Color, Style};

    let robots: Vec<_> = bots.into_iter().collect();

    fn draw_horizontal_wall(w: &World, y: usize, dir: Direction) {
        for x in 0..w.width() {
            let walls = w.walls(y, x);
            if walls.has(dir) {
                print!("####");
            } else {
                print!("    ");
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
        draw_horizontal_wall(w, y, Direction::North);
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
    }
    println!();

    draw_horizontal_wall(w, w.height() - 1, Direction::South);
}

pub struct TTYView;

pub fn new() -> TTYView {
    use crossterm::{cursor, terminal};
    crossterm::execute! {
        std::io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0,0),
    }
    .unwrap();

    TTYView
}

impl TTYView {
    pub fn draw<'a>(&self, w: &World, bots: impl IntoIterator<Item = &'a Robot>) {
        draw_world(w, bots)
    }
}
