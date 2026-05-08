use crate::{
    ggez_view::{GgezView, Karel},
    interface::{MonoRobotWorld, SimpleRobot},
    model, tty_view,
};
use std::sync::Mutex;

static KAREL: Mutex<Option<MonoRobotWorld>> = Mutex::new(None);

pub fn run(
    world: model::World,
    robot: model::Robot,
    mut user_program: impl FnMut() + Send + 'static,
) {
    // to ensure that the TTY is always cleaned up (even if user_program panics)
    struct ClearMutex;
    impl Drop for ClearMutex {
        fn drop(&mut self) {
            *KAREL.lock().unwrap() = None;
        }
    }

    // let output = Box::new(tty_view::new(
    //     world.width() as u16 * 4 + 1,
    //     world.height() as u16 * 2 + 2,
    // ));

    let (output, receiver) = GgezView::spawn();

    *KAREL.lock().unwrap() = Some(MonoRobotWorld {
        world,
        robot,
        output: Box::new(output),
    });

    let handle = std::thread::spawn(move || {
        let _restore = ClearMutex;
        user_program()
    });

    Karel::run(receiver)
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
