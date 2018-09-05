//! This module implements the pseudo-random generation
//! of mazes.
use super::*;
use rand;
use std::collections::HashMap;

struct MazeGen {
    maze: Maze,
    /// A map mapping keys (x | (y << 32)) to values
    /// which define certain properties of each
    /// wall, e.g. unbreakable walls like the edge of the maze.
    wall_props: HashMap<u64, WallProperty>,
}

struct WallProperty {
    unbreakable: bool,
}

/// Generates a maze with the specified
/// width and height. Both `width` and `height`
/// must be multiples of 8.
/// This may take a long time.
pub fn gen_maze(width: u32, height: u32) {
    assert!(width % 8 == 0 && height % 8 == 0);
    let mut ctx = MazeGen {
        maze: Maze {
            width,
            height,
            walls: vec![0; (width * height) as usize],
        },
        wall_props: HashMap::new(),
    };

    // Initialize maze with empty borders
    init_maze(&mut ctx);

    // Run the tracing iteration
    trace(&mut ctx);
}

/// Sets border walls to `true` on this maze
fn init_maze(ctx: &mut MazeGen) {
    let mut pos: (u32, u32) = (0, 0);
    let width = ctx.maze.width;
    let height = ctx.maze.height;
    let maze = &mut ctx.maze;
    let mut current_dir = 0; // 0 = right, 1 = down, 2 = left, 3 = up

    loop {
        maze.set_wall_at(pos.0, pos.1, true);
        ctx.wall_props.insert(
            wall_prop_key(pos.0, pos.1),
            WallProperty { unbreakable: true },
        );

        match current_dir {
            0 => {
                if pos.0 < width - 1 {
                    pos.0 += 1;
                } else {
                    current_dir = 1;
                    continue;
                }
            }
            1 => {
                if pos.1 < height - 1 {
                    pos.1 += 1;
                } else {
                    current_dir = 2;
                    continue;
                }
            }
            2 => {
                if pos.0 > 0 {
                    pos.0 -= 1;
                } else {
                    current_dir = 3;
                    continue;
                }
            }
            3 => {
                if pos.1 > 0 {
                    pos.1 -= 1;
                } else {
                    break;
                }
            }
            _ => panic!(),
        }

        // Check if finished
        if pos == (0, 0) {
            break;
        }
    }
}

fn wall_prop_key(x: u32, y: u32) -> u64 {
    (x as u64) | ((y as u64) << 32)
}

/// Goes through the entire maze,
/// randomizing paths and generating
/// a maze. There is not guarantee that
/// certain areas of the maze will become
/// inaccessible from the rest, so care
/// must be taken to detect these closed-off sections
/// and repair them.
fn trace(ctx: &mut MazeGen) {
    let pos: (u32, u32) = (0, 0);
}

#[cfg(test)]
mod tests {
    extern crate fastcmp;
    use super::*;
    use self::fastcmp::Compare;
    #[test]
    #[should_panic]
    fn _gen_maze_not_multiple_of_8() {
        gen_maze(9, 27);
    }

    #[test]
    fn _init_maze() {
        let mut ctx = MazeGen {
            maze: Maze {
                height: 8,
                width: 8,
                walls: vec![0; 8 * 8],
            },
            wall_props: HashMap::new(),
        };

        init_maze(&mut ctx);

        for byte in ctx.maze.walls.iter() {
            println!("{:b}", byte);
        }

        assert!(
            ctx.maze.walls.feq(
                & [
                    0b11111111, 0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b10000001,
                    0b10000001, 0b11111111,
                ]
            )
        );

    }
}
