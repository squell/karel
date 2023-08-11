#[derive(Debug)]
pub struct World {
    width: usize,
    horizontal_walls: Box<[bool]>,
    vertical_walls: Box<[bool]>,
    shells: Box<[bool]>,
}

#[derive(Debug, Clone, Default)]
pub struct Robot {
    pub pos: (usize, usize),
    pub dir: Direction,
    pub in_hold: usize,
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    #[default]
    East,
}

pub struct DirectionSet([bool; 4]);

impl DirectionSet {
    pub fn has(&self, dir: Direction) -> bool {
        self.0[dir as usize]
    }
}

impl World {
    pub fn new(height: usize, width: usize) -> World {
        World {
            width,
            horizontal_walls: vec![false; (height + 1) * width].into_boxed_slice(),
            vertical_walls: vec![false; height * (width + 1)].into_boxed_slice(),
            shells: vec![false; height * width].into_boxed_slice(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.shells.len() / self.width
    }

    pub fn fenced(mut self) -> World {
        // draw north wall
        for wall in &mut self.horizontal_walls[..self.width] {
            *wall = true
        }
        // draw south wall
        let size = self.horizontal_walls.len();
        for wall in &mut self.horizontal_walls[size - self.width..] {
            *wall = true
        }
        // draw western wall
        for wall in self.vertical_walls.iter_mut().step_by(self.width + 1) {
            *wall = true
        }
        // draw eastern wall
        for wall in self
            .vertical_walls
            .iter_mut()
            .skip(self.width)
            .step_by(self.width + 1)
        {
            *wall = true
        }

        self
    }

    pub fn set_wall(
        &mut self,
        (y, x): (usize, usize),
        dir: Direction,
        length: usize,
        value: bool,
    ) -> &mut World {
        match dir {
            Direction::South => {
                for row in y..y + (length - 1) {
                    self.vertical_walls[row * (self.width + 1) + x] = value
                }
            }
            Direction::East => {
                for col in x..x + (length - 1) {
                    self.horizontal_walls[y * self.width + col] = value
                }
            }
            Direction::West => {
                return self.set_wall((y, x - (length - 1)), Direction::East, length, value)
            }
            Direction::North => {
                return self.set_wall((y - (length - 1), x), Direction::South, length, value)
            }
        }

        self
    }

    pub fn set_shells(
        &mut self,
        shells: impl IntoIterator<Item = ((usize, usize), bool)>,
    ) -> &mut World {
        for ((y, x), value) in shells.into_iter() {
            self.shells[self.width * y + x] = value
        }

        self
    }

    pub fn add_wall(mut self, pos: (usize, usize), dir: Direction, length: usize) -> World {
        self.set_wall(pos, dir, length, true);

        self
    }

    pub fn add_shells(mut self, shells: impl IntoIterator<Item = (usize, usize)>) -> World {
        self.set_shells(shells.into_iter().map(|pos| (pos, true)));

        self
    }

    #[allow(clippy::identity_op)]
    pub fn walls(&self, y: usize, x: usize) -> DirectionSet {
        let width = self.width;
        DirectionSet([
            self.horizontal_walls[(y + 0) * width + x],     // north
            self.horizontal_walls[(y + 1) * width + x],     // south
            self.vertical_walls[y * (width + 1) + (x + 0)], // west
            self.vertical_walls[y * (width + 1) + (x + 1)], // east
        ])
    }

    pub fn has_shell(&self, y: usize, x: usize) -> bool {
        self.shells[y * self.width + x]
    }
}
