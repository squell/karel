use karel::{facing_north, on_crab, pick_crab_up, put_crab_down, step, turn_clockwise, wall_ahead};

fn robot_program() {
    while !wall_ahead() {
        step();
    }
    turn_clockwise();
    turn_clockwise();
    step();
    step();
    while !facing_north() {
        turn_clockwise();
    }
    while !on_crab() {
        step();
    }
    pick_crab_up();

    turn_clockwise();
    turn_clockwise();
    step();
    put_crab_down();
    turn_clockwise();

    loop {
        step();
    }
}

mod interface;
mod model;
mod static_interface;
mod tty_view;

use static_interface as karel;

fn main() {
    let world = model::World::new(10, 10)
        .fenced()
        .add_wall((2, 2), model::Direction::South, 5)
        .add_shells([(1, 1), (3, 7)]);

    let robot = model::Robot {
        pos: (8, 1),
        ..Default::default()
    };

    let _dropme = karel::start(world, robot);
    robot_program();
}
