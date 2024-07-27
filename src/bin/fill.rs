#![allow(unused_imports)]
#![allow(unused)]

use karel::{facing_north, on_crab, pick_crab_up, put_crab_down, step, turn_clockwise, wall_ahead};

fn robot_program() {
    todo!();
}

fn main() {
    let world = karel::World::new(30, 50).fenced();

    let robot = karel::Robot {
        pos: (15, 24),
        in_hold: 1000_0000,
        ..Default::default()
    };

    karel::run(world, robot, robot_program);
}
