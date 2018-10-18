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
    stack: Vec<usize>,
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

    loop {
        let index = NodeIndex::new(ctx.pos);
        {
            let cell = ctx.maze.index_mut(index);
            cell.visited = true;
        }
        if !check_edges(&mut ctx.maze, ctx.pos) {
            // Backtrace or finish
            if ctx.pos == 0 {
                break;
            }

            ctx.pos = ctx.stack.pop().unwrap();
            continue;
        }

        ctx.stack.push(ctx.pos);
        let adjacents = find_neighbors(&mut ctx.maze, ctx.pos);
        let num = rand::thread_rng().gen_range(0, adjacents.count());
        let adjacents = find_neighbors(&mut ctx.maze, ctx.pos).enumerate().collect();
        let next_cell = adjacents[num];
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

fn find_neighbors(maze: &mut Graph<Cell, u32, Undirected>, pos: usize) -> ::petgraph::graph::Neighbors<u32> {
    maze.neighbors_undirected(NodeIndex::new(pos))
}

fn check_edges(maze: &mut Graph<Cell, u32, Undirected>, pos: usize) -> bool {
    // Return if there are available adjacent cells
    let neighbors = find_neighbors(maze, pos);
    neighbors.count() > 0
}