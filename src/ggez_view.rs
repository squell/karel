use ggez::{
    ContextBuilder,
    conf::WindowMode,
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect},
    mint::Point2,
};

use crate::{Direction, Robot, World, interface};
use std::{
    f32::consts::PI,
    sync::mpsc::{Receiver, Sender, channel},
    time::Duration,
};

const PAUSE: Duration = Duration::from_millis(100);
const SCALE: f32 = 20.0;

pub struct GgezView {
    sender: Option<Sender<(World, Vec<Robot>)>>,
}

impl GgezView {
    pub fn spawn() -> (Self, Receiver<(World, Vec<Robot>)>) {
        let (sender, receiver) = channel();

        (
            Self {
                sender: Some(sender),
            },
            receiver,
        )
    }
}

impl interface::Display for GgezView {
    fn draw(&self, w: &World, bots: &mut dyn Iterator<Item = &Robot>) {
        self.sender
            .as_ref()
            .expect("view is only gone in drop")
            .send((w.clone(), bots.cloned().collect()))
            .expect("ggez view is always around");

        std::thread::sleep(PAUSE);
    }
}

impl Drop for GgezView {
    fn drop(&mut self) {
        drop(self.sender.take());
    }
}

pub struct Karel {
    receiver: Receiver<(World, Vec<Robot>)>,
    world: World,
    robots: Vec<Robot>,

    robo_mesh: Mesh,
    crab_mesh: Mesh,
    wall_mesh: Mesh,
}

impl Karel {
    pub fn run(receiver: Receiver<(World, Vec<Robot>)>) {
        // Wait for initial state
        let Ok((world, robots)) = receiver.recv() else {
            // There is no world...
            return;
        };

        let window = WindowMode {
            width: world.width() as f32 * SCALE,
            height: world.height() as f32 * SCALE,
            ..Default::default()
        };

        // Make a Context.
        let (ctx, event_loop) = ContextBuilder::new("Karel", "Trifecta Tech Foundation")
            .window_mode(window)
            .build()
            .expect("aieee, could not create ggez context!");

        const ROBO_SCALE: f32 = SCALE * 0.9;
        let robo_mesh = graphics::Mesh::new_polygon(
            &ctx,
            DrawMode::fill(),
            &[
                Point2 {
                    x: 0.0 * ROBO_SCALE,
                    y: 0.5 * ROBO_SCALE,
                },
                Point2 {
                    x: 0.5 * ROBO_SCALE,
                    y: -0.5 * ROBO_SCALE,
                },
                Point2 {
                    x: -0.5 * ROBO_SCALE,
                    y: -0.5 * ROBO_SCALE,
                },
            ],
            Color::GREEN,
        )
        .unwrap();

        let crab_bb = Rect::new(0.1 * SCALE, 0.1 * SCALE, 0.8 * SCALE, 0.8 * SCALE);
        let crab_mesh = graphics::Mesh::new_rounded_rectangle(
            &ctx,
            DrawMode::fill(),
            crab_bb,
            0.25 * SCALE,
            Color::RED,
        )
        .unwrap();

        let wall_mesh = graphics::Mesh::new_rounded_rectangle(
            &ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.05 * SCALE, 0.1 * SCALE, 0.9 * SCALE),
            0.1 * SCALE,
            Color::BLACK,
        )
        .unwrap();

        // Create an instance of your event handler.
        // Usually, you should provide it with the Context object to
        // use when setting your game up.
        let my_game = Self {
            receiver,
            world,
            robots,
            robo_mesh,
            crab_mesh,
            wall_mesh,
        };

        // Run!
        event::run(ctx, event_loop, my_game);
    }
}

impl EventHandler for Karel {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        match self.receiver.recv() {
            Ok((world, robots)) => {
                self.world = world;
                self.robots = robots;
            }
            Err(_) => {
                ctx.request_quit();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let gray = Color::from_rgb(0xD0, 0xD0, 0xD0);

        for x in 1..self.world.width() {
            let line = graphics::Mesh::new_line(
                ctx,
                &[
                    [0.0, 1.0],
                    [0.0, self.world.height() as f32 * SCALE + SCALE],
                ],
                1.0,
                gray,
            )
            .unwrap();

            canvas.draw(&line, DrawParam::new().dest([x as f32 * SCALE, 0.0]));
        }

        for y in 1..self.world.height() {
            let line = graphics::Mesh::new_line(
                ctx,
                &[[1.0, 0.0], [self.world.width() as f32 * SCALE + SCALE, 0.0]],
                1.0,
                gray,
            )
            .unwrap();

            canvas.draw(&line, DrawParam::new().dest([0.0, y as f32 * SCALE]));
        }

        for x in 0..self.world.width() {
            if self.world.walls(0, x).has(Direction::North) {
                canvas.draw(
                    &self.wall_mesh,
                    DrawParam::new()
                        .dest([x as f32 * SCALE + SCALE, -0.05 * SCALE])
                        .rotation(PI / 2.0),
                );
            }
        }

        for y in 0..self.world.height() {
            if self.world.walls(y, 0).has(Direction::West) {
                canvas.draw(
                    &self.wall_mesh,
                    DrawParam::new().dest([-0.05 * SCALE, y as f32 * SCALE]),
                );
            }
        }

        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                let walls = self.world.walls(y, x);
                if walls.has(Direction::East) {
                    canvas.draw(
                        &self.wall_mesh,
                        DrawParam::new().dest([x as f32 * SCALE + 0.95 * SCALE, y as f32 * SCALE]),
                    );
                }

                if walls.has(Direction::South) {
                    canvas.draw(
                        &self.wall_mesh,
                        DrawParam::new()
                            .dest([x as f32 * SCALE + SCALE, y as f32 * SCALE + 0.95 * SCALE])
                            .rotation(PI / 2.0),
                    );
                }
            }
        }

        for x in 0..self.world.width() {
            for y in 0..self.world.height() {
                if self.world.has_shell(y, x) {
                    canvas.draw(
                        &self.crab_mesh,
                        DrawParam::new().dest([x as f32 * SCALE, y as f32 * SCALE]), // .scale([SCALE * 0.9, SCALE * 0.9]),
                    );
                }
            }
        }

        for robo in &self.robots {
            canvas.draw(
                &self.robo_mesh,
                DrawParam::new()
                    .dest([
                        robo.pos.1 as f32 * SCALE + SCALE * 0.5,
                        robo.pos.0 as f32 * SCALE + SCALE * 0.5,
                    ])
                    .rotation(match robo.dir {
                        Direction::North => PI,
                        Direction::South => 0.0,
                        Direction::West => PI / 2.0,
                        Direction::East => 3.0 * PI / 2.0,
                    }),
            );
        }

        // Draw code here...
        canvas.finish(ctx)?;

        Ok(())
    }
}
