use crate::model::{Direction, Robot, World};

use std::time::Duration;

const PAUSE: Duration = Duration::from_millis(100);

pub trait SimpleRobot {
    fn step(&mut self);
    fn turn_clockwise(&mut self);
    fn pick_crab_up(&mut self);
    fn put_crab_down(&mut self);
    fn wall_ahead(&self) -> bool;
    fn on_crab(&self) -> bool;
    fn facing_north(&self) -> bool;
}

pub struct MonoRobotWorld {
    pub world: World,
    pub robot: Robot,
    tty: crate::tty_view::TTYView,
}

impl SimpleRobot for MonoRobotWorld {
    fn step(&mut self) {
        if self.wall_ahead() {
            panic!("Karel crashed into a wall!");
        }
        let (y, x) = self.robot.pos;
        self.robot.pos = match self.robot.dir {
            Direction::North => (y - 1, x),
            Direction::South => (y + 1, x),
            Direction::East => (y, x + 1),
            Direction::West => (y, x - 1),
        };

        self.update();
    }

    fn turn_clockwise(&mut self) {
        self.robot.dir = match self.robot.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        self.update();
    }

    fn pick_crab_up(&mut self) {
        if !self.on_crab() {
            return;
        }
        self.robot.in_hold += 1;
        self.world.set_shells([(self.robot.pos, false)]);
        self.update();
    }

    fn put_crab_down(&mut self) {
        if self.robot.in_hold == 0 {
            return;
        } else if self.on_crab() {
            panic!("There is already a crab shell here!");
        }
        self.robot.in_hold -= 1;
        self.world.set_shells([(self.robot.pos, true)]);
        self.update();
    }

    fn wall_ahead(&self) -> bool {
        self.world
            .walls(self.robot.pos.0, self.robot.pos.1)
            .has(self.robot.dir)
    }

    fn on_crab(&self) -> bool {
        self.world.has_shell(self.robot.pos.0, self.robot.pos.1)
    }

    fn facing_north(&self) -> bool {
        matches!(self.robot.dir, Direction::North)
    }
}

impl MonoRobotWorld {
    pub fn from(world: World, robot: Robot) -> MonoRobotWorld {
        let this = MonoRobotWorld {
            world,
            robot,
            tty: crate::tty_view::new(),
        };
        this.update();

        this
    }

    fn update(&self) {
        crossterm::execute! {
            std::io::stdout(),
            crossterm::cursor::MoveTo(0,0),
        }
        .unwrap();

        self.tty.draw(&self.world, [&self.robot]);
        std::thread::sleep(PAUSE);
    }
}
