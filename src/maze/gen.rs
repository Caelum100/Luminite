//! This module implements the pseudo-random generation
//! of mazes using the depth-first search algorithm.
//!
//! The maze is stored as an undirected
//! connected graph with the cells representing spaces
//! in the maze and the edges representing walls between
//! two cells.
use super::*;
use rand;
use self::rand::Rng;
use self::rand::random;
use std::collections::HashMap;
use std::ops::Add;
use petgraph::*;
use petgraph::graph::NodeIndex;

struct Ctx {
    maze: Graph<Cell, u32, Undirected>,
    stack: Vec<u32>,
    pos: u32,
}

#[derive(Clone, Copy, Debug, Hash)]
struct Cell {
    visited: bool,
}

pub fn gen_maze(width: usize, height: usize) {
    let mut ctx = Ctx {
        maze: Graph::new_undirected(),
        stack: Vec::new(),
        pos: 0,
    };

    fill_graph(&mut ctx.maze, width, height);

    let mut running = true;
    loop {
        ctx.maze.find

        if !running {
            break;
        }
    }
}

fn fill_graph(maze: &mut Graph<Cell, u32, Undirected>, width: usize, height: usize) {
    for w in 0..width {
        for h in 0..height {
            maze.add_node(Cell {
                visited: false
            });
        }
    }
    // Connect all cells with walls
    for w in 0..width {
        for h in 0..height {
            let index_num = w * height + h;
            let index = NodeIndex::new(index_num);

            // TODO finish
        }
    }
}