//! This module implements the pseudo-random generation
//! of mazes.
use super::*;
use rand;
use std::collections::HashMap;

struct MazeGen {
    maze: Maze,
    /// A map mapping keys (x | (y << 32), wall_dir) to values
    /// which define certain properties of each
    /// wall, e.g. unbreakable walls like the edge of the maze.
    wall_props: HashMap<PropKey, WallProperty>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct PropKey {
    coord: u64,
    dir: WallDir,
}

impl PropKey {
    pub fn new(coord: u64, dir: WallDir) -> Self {
        PropKey { coord, dir }
    }
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
            vertical_walls: vec![0; (width * height) as usize],
            horizontal_walls: vec![0; (width * height) as usize],
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
    let width = ctx.maze.width;
    let height = ctx.maze.height;
    assert!(width % 2 == 0 && height % 2 == 0);
    let maze = &mut ctx.maze;

    // Set horizontal walls at bottom and top
    for x in 0..width {
        maze.set_wall_at(x, 0, WallDir::HORIZONTAL, true);
        maze.set_wall_at(x, height - 1, WallDir::HORIZONTAL, true);
        ctx.wall_props.insert(
            PropKey::new(wall_prop_key(x, 0), WallDir::HORIZONTAL),
            WallProperty { unbreakable: true },
        );
        ctx.wall_props.insert(
            PropKey::new(wall_prop_key(x, height - 1), WallDir::HORIZONTAL),
            WallProperty { unbreakable: true },
        );
    }

    // Set vertical walls on the sides
    for y in 0..height {
        maze.set_wall_at(0, y, WallDir::VERTICAL, true);
        maze.set_wall_at(width - 1, y, WallDir::VERTICAL, true);
        ctx.wall_props.insert(
            PropKey::new(wall_prop_key(0, y), WallDir::VERTICAL),
            WallProperty { unbreakable: true },
        );
        ctx.wall_props.insert(
            PropKey::new(wall_prop_key(width - 1, y), WallDir::VERTICAL),
            WallProperty { unbreakable: true },
        );
    }
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

/// Returns the sides of this cell
/// that are not walls.
fn get_open_faces(ctx: &MazeGen, cell_x: u32, cell_y: u32) -> Faces {
    let maze = &ctx.maze;

    let left = !maze.has_wall_at(cell_x, cell_y, WallDir::VERTICAL);
    let right = !maze.has_wall_at(cell_x + 1, cell_y, WallDir::VERTICAL);
    let top = !maze.has_wall_at(cell_x, cell_y, WallDir::HORIZONTAL);
    let bottom = !maze.has_wall_at(cell_x, cell_y + 1, WallDir::HORIZONTAL);

    Faces::new(left, right, top, bottom)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Faces {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
}

impl Faces {
    fn new(left: bool, right: bool, top: bool, bottom: bool) -> Self {
        Faces {
            left,
            right,
            bottom,
            top,
        }
    }
}

fn wall_prop_key(x: u32, y: u32) -> u64 {
    (x as u64) | ((y as u64) << 32)
}

#[cfg(test)]
mod tests {
    use super::*;
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
                vertical_walls: vec![0; 8 * 8],
                horizontal_walls: vec![0; 8 * 8],
            },
            wall_props: HashMap::new(),
        };

        init_maze(&mut ctx);

        assert_eq!(ctx.maze.horizontal_walls[0], 0b11111111);
        assert_eq!(ctx.maze.horizontal_walls[1], 0b00000000);
        assert_eq!(ctx.maze.horizontal_walls[7], 0b11111111);

        assert_eq!(ctx.maze.vertical_walls[0], 0b10000001);
        assert_eq!(ctx.maze.vertical_walls[3], 0b10000001);
    }

    #[test]
    fn _get_open_faces() {
        let mut ctx = MazeGen {
            maze: Maze {
                height: 8,
                width: 8,
                horizontal_walls: vec![0; 8 * 8],
                vertical_walls: vec![0; 8 * 8],
            },
            wall_props: HashMap::new(),
        };

        println!("{:?}", get_open_faces(&ctx, 0, 0));

        assert_eq!(
            get_open_faces(&ctx, 0, 0),
            Faces::new(true, true, true, true),
        );
        ctx.maze.set_wall_at(0, 0, WallDir::VERTICAL, true);
        assert_eq!(
            get_open_faces(&ctx, 0, 0),
            Faces::new(false, true, true, true),
        );
        ctx.maze.set_wall_at(0, 1, WallDir::HORIZONTAL, true);
        assert_eq!(
            get_open_faces(&ctx, 0, 0),
            Faces::new(false, true, true, false),
        );
    }
}
