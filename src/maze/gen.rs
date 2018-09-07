//! This module implements the pseudo-random generation
//! of mazes.
use self::rand::distributions::{Distribution, Standard};
use self::rand::Rng;
use super::*;
use rand;
use std::collections::HashMap;
use std::ops::Add;

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
pub fn gen_maze(width: u32, height: u32) -> Maze {
    assert!(width % 8 == 0 && height % 8 == 0);
    let mut ctx = MazeGen {
        maze: Maze {
            width,
            height,
            vertical_walls: vec![0; (width * height / 8) as usize],
            horizontal_walls: vec![0; (width * height / 8) as usize],
        },
        wall_props: HashMap::new(),
    };

    // Initialize maze with empty borders
    init_maze(&mut ctx);

    // Run the tracing iteration
    trace(&mut ctx);

    ctx.maze
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
    use self::rand::random;
    use self::Direction::*;

    let mut pos = Pos::new(0, 0);
    let height = ctx.maze.height;
    let width = ctx.maze.width;
    let total_cycles = width * height * 4;
    let mut direction = RIGHT;
    let mut cycles_run = 0;
    let mut faces;

    while cycles_run < total_cycles {
        {
            faces = {
                let mut left = false;
                let mut right = false;
                let mut top = false;
                let mut bottom = false;

                match direction {
                    RIGHT | LEFT => {
                        top = true;
                        bottom = true;
                    }
                    DOWN | UP => {
                        left = true;
                        right = true;
                    }
                }

                Faces::new(left, right, top, bottom)
            };
        }
        println!("({}, {})", pos.x, pos.y);

        set_walls_at_face(ctx, pos.x as u32, pos.y as u32, faces);

        if random::<u8>() < 64 {
            direction = random();
        }

        // Make sure we don't go out of bounds
        let mut new_pos = pos + move_in_direction(direction);
        println!("{:?}", new_pos);
        while new_pos.x >= ((width - 1) as i32)
            || new_pos.x < 0
            || new_pos.y >= ((height - 1) as i32)
            || new_pos.y < 0
        {
            direction = random();
            new_pos = pos + move_in_direction(direction);
            println!("HEIGHT {}, {}", height, width);
        }

        pos = new_pos;

        cycles_run += 1;
    }
}

// Moves in the specified direction, returning
// the change in position for both `x` and `y`.
fn move_in_direction(direction: Direction) -> Pos {
    let mut x = 0;
    let mut y = 0;
    match direction {
        Direction::UP => y -= 1,
        Direction::DOWN => y += 1,
        Direction::LEFT => x -= 1,
        Direction::RIGHT => x += 1,
    };

    Pos::new(x, y)
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

/// Sets the walls of a maze around the specified
/// cell to the specified values. This will not
/// modify walls marked as unbreakable.
fn set_walls_at_face(ctx: &mut MazeGen, cell_x: u32, cell_y: u32, faces: Faces) {
    let wall_props = &ctx.wall_props;
    let maze = &mut ctx.maze;
    let key = PropKey::new(wall_prop_key(cell_x, cell_y), WallDir::VERTICAL);
    if wall_props.get(&key).is_none() || !wall_props.get(&key).unwrap().unbreakable {
        maze.set_wall_at(cell_x, cell_y, WallDir::VERTICAL, faces.left);
    }
    let key = PropKey::new(wall_prop_key(cell_x + 1, cell_y), WallDir::VERTICAL);
    if wall_props.get(&key).is_none() || !wall_props.get(&key).unwrap().unbreakable {
        maze.set_wall_at(cell_x + 1, cell_y, WallDir::VERTICAL, faces.right);
    }
    let key = PropKey::new(wall_prop_key(cell_x, cell_y), WallDir::HORIZONTAL);
    if wall_props.get(&key).is_none() || !wall_props.get(&key).unwrap().unbreakable {
        maze.set_wall_at(cell_x, cell_y, WallDir::HORIZONTAL, faces.top);
    }
    let key = PropKey::new(wall_prop_key(cell_x, cell_y + 1), WallDir::HORIZONTAL);
    if wall_props.get(&key).is_none() || !wall_props.get(&key).unwrap().unbreakable {
        maze.set_wall_at(cell_x, cell_y + 1, WallDir::HORIZONTAL, faces.bottom);
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: Pos) -> <Self as Add<Pos>>::Output {
        Pos {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
        }
    }
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

/// A direction
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::UP,
            1 => Direction::DOWN,
            2 => Direction::LEFT,
            3 => Direction::RIGHT,
            _ => panic!(),
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

    #[test]
    fn _set_walls_at_face() {
        let maze = Maze {
            width: 8,
            height: 8,
            vertical_walls: vec![0; 8 * 8],
            horizontal_walls: vec![0; 8 * 8],
        };
        let mut ctx = MazeGen {
            maze,
            wall_props: HashMap::new(),
        };
        ctx.wall_props.insert(
            PropKey::new(wall_prop_key(0, 0), WallDir::VERTICAL),
            WallProperty { unbreakable: true },
        );

        set_walls_at_face(&mut ctx, 0, 0, Faces::new(true, true, false, false));
        assert!(!ctx.maze.has_wall_at(0, 0, WallDir::VERTICAL));
        assert!(ctx.maze.has_wall_at(1, 0, WallDir::VERTICAL));
    }

    #[test]
    fn _move_in_direction() {
        assert_eq!(move_in_direction(Direction::UP), Pos::new(0, -1),);
        assert_eq!(move_in_direction(Direction::DOWN), Pos::new(0, 1),);
        assert_eq!(move_in_direction(Direction::LEFT), Pos::new(-1, 0),);
        assert_eq!(move_in_direction(Direction::RIGHT), Pos::new(1, 0),);
    }
}
