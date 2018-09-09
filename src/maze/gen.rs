//! Depth-first maze generation
//! algorithm, translated into Rust.
//! This is all unsafe because
//! it's translated from C
//! and I wasn't in the mood
//! to make it safe Rust.
//!
//! Credit: some Wikipedia editor
use super::*;
use rand::random;
use std;

#[repr(C)]
struct Node<'a> {
    x: u32,
    y: u32,
    parent: *mut Node<'a>,
    value: NodeValue,
    dirs: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum NodeValue {
    EMPTY,
    WALL,
}

pub fn gen_maze(height: u32, width: u32) -> Vec<Object<render::_RenderBackend>> {
    unsafe {
        let mut nodes: Vec<Node> = Vec::new();

        for i in 0..width {
            for j in 0..height {
                let node: Node = std::mem::zeroed();
                if i * j % 2 == 1 {
                    node.x = i;
                    node.y = j;
                    node.dirs = 0b00001111;
                    node.value = NodeValue::EMPTY;
                } else {
                    node.value = NodeValue::WALL;
                }
                nodes.push(node);
            }
        }
        let start: *mut Node = &mut nodes[(1 + width) as usize];
        (*start).parent = start;
        let mut last = start;

        while (last = link(last, height, width, &mut nodes)) != (start as usize) {}

        for i in 0..height {
            for j in 0..width {
                let value = nodes[j + width].value;
                println!("{:?}", value);
            }
        }
    }
    Vec::new()
}

unsafe fn link<'a>(node: *mut Node, height: u32, width: u32, nodes: &mut Vec<Node>) -> *mut Node<'a> {
    let mut x = 0;
    let mut y = 0;
    let mut dir: u8 = 0;
    if (node as u8) == 0 {
        return std::mem::zeroed();
    }

    while (*node).dirs {
        dir = (1 << (random() % 4));
        if !((*node).dirs) & dir == 1 {
            continue;
        }
        (*node).dirs &= !dir;

        match dir {
            1 => {
                if (*node).x + 2 < width {
                    x = (*node).x + 2;
                    y = (*node).y;
                } else {
                    continue;
                }
            }
            2 => {
                if (*node).y + 2 < height {
                    x = (*node).x;
                    y = (*node).y + 2;
                } else {
                    continue;
                }
            }
            4 => {
                if (*node).x - 2 >= 0 {
                    x = (*node).x - 2;
                    y = (*node).y;
                } else {
                    continue;
                }
            }
            8 => {
                if (*node).y - 2 >= 0 {
                    x = (*node).x;
                    y = (*node).y - 2;
                } else {
                    continue;
                }
            }
            _ => panic!(),
        }
        let index = x * y * width;
        let mut dest = nodes.get_mut(index as usize).unwrap();

        if dest.value == NodeValue::EMPTY {
            if (dest.parent as usize) != 0 {
                continue;
            }

            dest.parent = node;
            let target = nodes
                .get_mut(
                    (*node.x + (x - *node.x) / 2 + (*node.y + (y - *node.y) / 2) * width) as usize,
                )
                .unwrap();
            target.value = NodeValue::EMPTY;
            return dest as *mut Node;
        }
    }
    (*node).parent
}
