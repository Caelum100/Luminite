//! This module implements the pseudo-random generation
//! of mazes using the depth-first search algorithm.
use super::*;
use rand;
use self::rand::Rng;
use self::rand::random;
use std::collections::HashMap;
use std::ops::Add;

struct Ctx {
    maze: Maze,
    stack: Vec<Pos>,
    pos: Pos,
}

#[derive(Clone, Copy, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

// Y=Odd number: horizontal, even: vertical
// Y values will range from 0 to height * 2 - 1,
// while X values will range from 0 to width
impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

pub fn gen_maze(width: usize, height: usize) {
    let mut ctx = Ctx {
        maze: Maze::new(height as u32, width as u32),
        stack: Vec::new(),
        pos: Pos::new(0, 0),
    };

    let mut finished = false;
    loop {


        // Push to stack
        ctx.stack.push(ctx.pos);
        move_pos(&mut ctx);
        if finished {
            break;
        }
    }
}

fn move_pos(ctx: &mut Ctx) {
    // Move position randomly
    let incr: i32 = {
        if random() { 1 }
            else { -1 }
    };
    if random() {
        ctx.pos.x = ((ctx.pos.x as i32) + incr) as usize;
    } else {
        ctx.pos.y = ((ctx.pos.y as i32) + incr) as usize;
    }
}