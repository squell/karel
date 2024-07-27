#![allow(unused_imports)]

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

fn toggle_crab() {
    if on_crab() {
        pick_crab_up();
    } else {
        put_crab_down();
    }
}

fn turn_counter_clockwise() {
    todo!();
}

fn facing_west() -> bool {
    turn_clockwise();
    if facing_north() {
        turn_counter_clockwise();

        true
    } else {
        turn_counter_clockwise();

        false
    }
}

fn steps(num_steps: i32) {
    todo!();
}

fn ahead_four_steps() {
    steps(4)
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
