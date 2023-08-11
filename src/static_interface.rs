use crate::{
    interface::{MonoRobotWorld, SimpleRobot},
    model,
};
use std::sync::Mutex;

static KAREL: Mutex<Option<MonoRobotWorld>> = Mutex::new(None);

pub fn start(world: model::World, robot: model::Robot) {
    *KAREL.lock().unwrap() = Some(MonoRobotWorld::from(world, robot))
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

forward! { step, turn_clockwise, pick_crab_up, put_crab_down, is_wall_ahead -> bool, is_on_crab -> bool }
