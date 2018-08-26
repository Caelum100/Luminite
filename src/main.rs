extern crate glm;
extern crate winit;
#[macro_use]
extern crate log;
extern crate num;
extern crate rand;
extern crate simple_logger;
extern crate tobj;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(all(feature = "vulkan", not(any(feature = "dx12", feature = "metal"))))]
extern crate gfx_backend_vulkan as back;

extern crate gfx_hal;

pub use glm::*;
use std::cell::RefCell;
use std::rc::Rc;
use winit::{Event, WindowEvent};
use world::{Location, Object, World};

pub mod render;
pub mod world;

pub struct Game {
    pub render: render::context::RenderContext<back::Backend>,
    pub world: World,
    pub running: bool,
}

fn main() {
    simple_logger::init().unwrap();
    let mut game = Game {
        render: render::create_context(),
        world: World::new(),
        running: true,
    };

    game.world.add_object(Object {
        id: 0,
        model_index: 0,
        location: Location::new(0.0, 0.0, 0.0),
    });

    main_loop(&mut game);
}

fn main_loop(game: &mut Game) {
    while game.running {
        poll_events(game);
        render::render(&mut game.render, &game.world);
    }
}

/// Polls events
fn poll_events(game: &mut Game) {
    let mut running = true;
    let events_loop = &mut game.render.events_loop;
    events_loop.poll_events(|event| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => running = false,
            _ => (),
        },
        _ => (),
    });
    game.running = running;
}
