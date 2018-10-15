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
use std::ops::IndexMut;
use std::collections::HashMap;
use std::ops::Add;
use petgraph::*;
use petgraph::graph::NodeIndex;

struct Ctx {
    maze: Graph<Cell, u32, Undirected>,
    stack: Vec<u32>,
    pos: usize,
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
        let index = NodeIndex::new(ctx.pos);
        {
            let cell = ctx.maze.index_mut(index);
            cell.visited = true;
        }



        if !running {
            break;
        }
    }
}

fn fill_graph(maze: &mut Graph<Cell, u32, Undirected>, width: usize, height: usize) {
    for h in 0..height {
        for w in 0..width {
            maze.add_node(Cell {
                visited: false
            });
        }
    }
    // Connect all cells with walls
    for h in 0..height {
        for w in 0..width {
            let index_num = h * width + w;
            let index = NodeIndex::new(index_num);

            // Above
            if h > 0 {
                maze.add_edge(index, NodeIndex::new(index_num - width), 0);
            }
            // Right
            if w < width - 1 {
                maze.add_edge(index, NodeIndex::new(index_num + 1), 0);
            }
            // Below
            if h < height - 1 {
                maze.add_edge(index, NodeIndex::new(index_num + width), 0);
            }
            // Left
            if w > 0 {
                maze.add_edge(index, NodeIndex::new(index_num - 1), 0);
            }
        }
    }
}

fn check_edges(maze: &mut Graph<Cell, u32, Undirected>, pos: usize) -> bool {
    // Return false if there are no unvisited adjacent cells
    // TODO
    true
}