#![allow(unused)]

use karel::{facing_north, on_crab, pick_crab_up, put_crab_down, step, turn_clockwise, wall_ahead};

fn first_steps() {
    step();
    turn_clockwise();
    step();
    turn_clockwise();
    step();
    turn_clockwise();
    step();
    turn_clockwise();
}

fn turn_counter_clockwise() {
    todo!();
}

fn ahead_four_steps() {
    todo!();
}

fn make_line_of_our_crabs() {
    todo!();
}

fn main() {
    let world = karel::World::new(30, 50).fenced();

    let robot = karel::Robot {
        pos: (15, 24),
        in_hold: 1000,
        ..Default::default()
    };

    karel::run(world, robot, first_steps);
}
