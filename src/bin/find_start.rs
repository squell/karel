#![allow(unused_imports)]

use karel::{facing_north, on_crab, pick_crab_up, put_crab_down, step, turn_clockwise, wall_ahead};

fn robot_program() {
    while !facing_north() {
        turn_clockwise();
    }

    while !wall_ahead() {
        step();
    }

    for _ in 0..3 {
        turn_clockwise();
    }

    while !wall_ahead() {
        step();
    }

    turn_clockwise();
    turn_clockwise();
}

fn main() {
    let world = karel::World::new(30, 50).fenced();

    let robot = karel::Robot {
        pos: (15, 24),
        in_hold: 1000,
        ..Default::default()
    };

    karel::run(world, robot, robot_program);
}
