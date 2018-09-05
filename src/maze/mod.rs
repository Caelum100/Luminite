//! Module for the creation and manipulation of mazes.

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
pub struct Maze {
    height: u32,
    width: u32,
    walls: Vec<u8>,
}

impl Maze {
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
    pub fn has_wall_at(&self, column: u32, row: u32) -> bool {
        assert!(column < self.width && row < self.height);
        let byte_index = self.byte_index(column, row);
        let byte_offset = self.byte_offset(column);

        let byte = self.walls[byte_index];
        (byte >> (7 - byte_offset)) & 0b0000001 == 1
    }

    /// Sets whether a wall exists at the specified position, returning
    /// the old value.
    /// The coordinates are as described in the struct documentation.
    pub fn set_wall_at(&mut self, column: u32, row: u32, value: bool) -> bool {
        assert!(column < self.width && row < self.height);
        let old_value = self.has_wall_at(column, row);

        let byte_index = self.byte_index(column, row);
        let byte_offset = self.byte_offset(column);

        println!("{}, {}", byte_index, byte_offset);

        if value {
            self.walls[byte_index] |= (value as u8) << (7 - byte_offset);
        } else {
            self.walls[byte_index] &= !(1 << (7 - byte_offset));
        }

        old_value
    }
}

#[cfg(test)]
mod tests {
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
            walls: vec![0b10101010, 0b10101010, 0b10101010, 0b10101010],
        };

        assert!(maze.has_wall_at(0, 0));
        assert!(!maze.has_wall_at(1, 0));
        assert!(maze.has_wall_at(0, 1));
        assert!(!maze.has_wall_at(1, 1));

        assert!(!maze.has_wall_at(3, 3));
        assert!(maze.has_wall_at(2, 3));
    }

    #[test]
    #[should_panic]
    fn has_wall_at_out_of_bounds() {
        let maze = maze();
        maze.has_wall_at(64, 67);
    }

    #[test]
    fn set_wall_at() {
        let mut maze = maze();
        assert!(!maze.set_wall_at(0, 0, true));
        assert!(maze.has_wall_at(0, 0));
        assert!(maze.set_wall_at(0, 0, false));
        assert!(!maze.has_wall_at(0, 0));
        assert!(!maze.set_wall_at(63, 29, true));
        assert!(maze.has_wall_at(63, 29));
        assert!(maze.set_wall_at(63, 29, false));
        assert!(!maze.has_wall_at(63, 29));
    }

    #[test]
    #[should_panic]
    fn set_wall_at_out_of_bounds() {
        let mut maze = maze();
        maze.set_wall_at(9039, 983, true);
    }

    fn maze() -> Maze {
        Maze {
            height: 64,
            width: 64,
            walls: vec![0; 64 * 64],
        }
    }
}
