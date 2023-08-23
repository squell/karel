use crate::{
    interface::{MonoRobotWorld, SimpleRobot},
    model, tty_view,
};
use std::sync::Mutex;

static KAREL: Mutex<Option<MonoRobotWorld>> = Mutex::new(None);

pub fn run(world: model::World, robot: model::Robot, mut user_program: impl FnMut()) {
    // this is a hack to ensure that TTYView::Drop is called
    let _restore_cursor = tty_view::new();

    *KAREL.lock().unwrap() = Some(MonoRobotWorld::from(world, robot));

    user_program()
}

macro_rules! forward {
    ($($method:ident $(-> $type:ty)?),*) => {
	$(
	    pub fn $method() $(-> $type)? {
		KAREL.lock().unwrap().as_mut().unwrap().$method()
	    }
	)*
    }
}

forward! { step, turn_clockwise, pick_crab_up, put_crab_down, wall_ahead -> bool, on_crab -> bool, facing_north -> bool }
