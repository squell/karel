#![allow(unused_imports)]
#![allow(unused)]

use karel::{facing_north, on_crab, pick_crab_up, put_crab_down, step, turn_clockwise, wall_ahead};

fn robot_program() {
    for _ in 0..5 {
        while !wall_ahead() {
            put_crab_down();
            step();
        }

        turn_clockwise();
    }

    turn_clockwise();
    turn_clockwise();
    step();
    turn_clockwise();
    turn_clockwise();
    turn_clockwise();

    while !wall_ahead() {
        put_crab_down();
        step();
    }
}

fn main() {
    let world = karel::World::new(10, 10)
        .fenced()
        .add_wall((1, 0), karel::Direction::East, 5);

    let robot = karel::Robot {
        pos: (5, 5),
        in_hold: 1000_0000,
        ..Default::default()
    };

    karel::run(world, robot, robot_program);
}
