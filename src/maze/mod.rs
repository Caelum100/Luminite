//! Module for the creation and manipulation of mazes.

use render;
use render::RenderBackend;
use std::fmt::Error;
use std::fmt::Formatter;
use world::{Location, Object, World};

pub mod gen;

/// A maze is represented
/// as a vector of bytes. Each wall is stored
/// as a single bit in a byte: a place with a wall
/// is 1, and a place without is 0. The vector
/// stores each row of walls in order. The first row
/// is the first set of horizantal walls, the next
/// row is the first set of vertical walls, and so on. A maze like this:
/// ```
/// __ __ __ __
///|  |  |  |  |
/// ```
/// would be stored like this:
/// ```
/// 0b0110110110110_1001001001001
/// ```
///
/// It is required that the length and width
/// of a maze are multiples of 8. Undefined behavior
/// may occur of this is not the case.
///
/// Positions passed to functions start at 0
/// and go from left to right and top to bottom
/// respectively.
///
/// Mazes should be generated using the `gen` module.
#[derive(Debug)]
pub struct Maze {
    height: u32,
    width: u32,
    horizontal_walls: Vec<u8>,
    vertical_walls: Vec<u8>,
}

impl Maze {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            height,
            width,
            horizontal_walls: vec![0; (height * width / 8) as usize],
            vertical_walls: vec![0; (height * width / 8) as usize],
        }
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn byte_index(&self, column: u32, row: u32) -> usize {
        ((row * (self.width >> 3)) + (column >> 3)) as usize
    }

    /// Finds the offset from the left side
    /// of the byte for the specified position.
    fn byte_offset(&self, column: u32) -> u32 {
        column % 8
    }

    /// Returns whether or not there is a wall
    /// at the specified position. The coordinates
    /// are as described in the struct documentation.
    pub fn has_wall_at(&self, column: u32, row: u32, dir: WallDir) -> bool {
        assert!(column < self.width && row < self.height);
        let byte_index = self.byte_index(column, row);
        let byte_offset = self.byte_offset(column);

        let byte = match dir {
            WallDir::VERTICAL => self.vertical_walls[byte_index],
            WallDir::HORIZONTAL => self.horizontal_walls[byte_index],
        };
        (byte >> (7 - byte_offset)) & 0b0000001 == 1
    }

    /// Sets whether a wall exists at the specified position, returning
    /// the old value.
    /// The coordinates are as described in the struct documentation.
    pub fn set_wall_at(&mut self, column: u32, row: u32, dir: WallDir, value: bool) -> bool {
        assert!(column < self.width && row < self.height);
        let old_value = self.has_wall_at(column, row, dir);

        let byte_index = self.byte_index(column, row);
        let byte_offset = self.byte_offset(column);

        let wall_vec = match dir {
            WallDir::VERTICAL => &mut self.vertical_walls,
            WallDir::HORIZONTAL => &mut self.horizontal_walls,
        };

        if value {
            wall_vec[byte_index] |= (value as u8) << (7 - byte_offset);
        } else {
            wall_vec[byte_index] &= !(1 << (7 - byte_offset));
        }

        old_value
    }

    /// Turns the walls of this maze into a vector
    /// of objects.
    pub fn get_objects<B: RenderBackend>(
        &self,
        world: &mut World<B>,
        render: &mut B::RenderContext,
    ) -> Vec<Object<B>> {
        let mut result = Vec::new();

        result
    }
}

impl super::std::fmt::Display for Maze {
    fn fmt<'a>(&self, f: &mut Formatter<'a>) -> Result<(), Error> {
        for byte in &self.horizontal_walls {
            writeln!(f, "{:b}", byte)?;
        }
        for byte in &self.vertical_walls {
            writeln!(f, "{:b}", byte)?;
        }
        write!(f, "height: {}, width: {}", self.height, self.width)?;

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum WallDir {
    VERTICAL,
    HORIZONTAL,
}

#[cfg(test)]
mod tests {
    use super::WallDir::*;
    use super::*;
    #[test]
    fn byte_index() {
        let maze = maze();

        assert_eq!(maze.byte_index(0, 0), 0);
        assert_eq!(maze.byte_index(1, 0), 0);
        assert_eq!(maze.byte_index(8, 0), 1);
        assert_eq!(maze.byte_index(0, 1), 8);
        assert_eq!(maze.byte_index(1, 1), 8);
        assert_eq!(maze.byte_index(9, 1), 9);
    }

    #[test]
    fn byte_offset() {
        let maze = maze();

        assert_eq!(maze.byte_offset(0), 0);
        assert_eq!(maze.byte_offset(1), 1);
        assert_eq!(maze.byte_offset(8), 0);
    }

    #[test]
    fn has_wall_at() {
        let maze = Maze {
            height: 8,
            width: 8,
            horizontal_walls: vec![0b10101010, 0b10101010, 0b10101010, 0b10101010],
            vertical_walls: vec![0b10101010, 0b10101010, 0b10101010, 0b10101010],
        };

        assert!(maze.has_wall_at(0, 0, VERTICAL));
        assert!(!maze.has_wall_at(1, 0, HORIZONTAL));
        assert!(maze.has_wall_at(0, 1, VERTICAL));
        assert!(!maze.has_wall_at(1, 1, VERTICAL));

        assert!(!maze.has_wall_at(3, 3, VERTICAL));
        assert!(maze.has_wall_at(2, 3, VERTICAL));
    }

    #[test]
    #[should_panic]
    fn has_wall_at_out_of_bounds() {
        let maze = maze();
        maze.has_wall_at(64, 67, VERTICAL);
    }

    #[test]
    fn set_wall_at() {
        let mut maze = maze();
        assert!(!maze.set_wall_at(0, 0, VERTICAL, true));
        assert!(maze.has_wall_at(0, 0, VERTICAL));
        assert!(maze.set_wall_at(0, 0, VERTICAL, false));
        assert!(!maze.has_wall_at(0, 0, VERTICAL));
        assert!(!maze.set_wall_at(63, 29, VERTICAL, true));
        assert!(maze.has_wall_at(63, 29, VERTICAL));
        assert!(maze.set_wall_at(63, 29, VERTICAL, false));
        assert!(!maze.has_wall_at(63, 29, VERTICAL));
    }

    #[test]
    #[should_panic]
    fn set_wall_at_out_of_bounds() {
        let mut maze = maze();
        maze.set_wall_at(9039, 983, VERTICAL, true);
    }

    fn maze() -> Maze {
        Maze {
            height: 64,
            width: 64,
            horizontal_walls: vec![0; 64 * 64],
            vertical_walls: vec![0; 64 * 64],
        }
    }
}
