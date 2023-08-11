mod interface;
mod model;
mod tty_view;
use interface::{MonoRobotWorld, SimpleRobot};

fn do_the_robot(mut karel: MonoRobotWorld) {
    while !karel.is_wall_ahead() {
        karel.step();
    }
    karel.turn_clockwise();
    karel.turn_clockwise();
    karel.step();
    karel.step();
    karel.turn_clockwise();
    while !karel.is_on_crab() {
        karel.step();
    }
    karel.pick_crab_up();
    karel.turn_clockwise();
    karel.turn_clockwise();
    karel.step();
    karel.put_crab_down();

    karel.turn_clockwise();
    loop {
        karel.step();
    }
}

fn main() {
    let world = model::World::new(10, 10)
        .fenced()
        .add_wall((2, 2), model::Direction::South, 5)
        .add_shells([(1, 1), (3, 7)]);

    let robot = model::Robot {
        pos: (8, 1),
        ..Default::default()
    };

    do_the_robot(MonoRobotWorld::from(world, robot));
}
