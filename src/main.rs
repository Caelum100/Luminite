extern crate glm;
extern crate winit;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(all(feature = "vulkan", not(any(feature = "dx12", feature = "metal"))))]
extern crate gfx_backend_vulkan as back;

extern crate gfx_hal;

pub use glm::*;
use winit::{Event, WindowEvent};

pub mod render;

struct Game {
    render: render::context::RenderContext<back::Backend>,
    running: bool,
}

fn main() {
    println!("Starting Luminite...");
    let mut game = Game {
        render: render::create_context(),
        running: true,
    };

    main_loop(&mut game);
}

fn main_loop(game: &mut Game) {
    while game.running {
        poll_events(game);
        render::render(&mut game.render);
    }
}

fn poll_events(game: &mut Game) {
    let events_loop = &mut game.render.events_loop;
    let mut running = true;
    events_loop.poll_events(move |event| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => running = false,
                _ => (),
            }
            _ => (),
        }
    });
    game.running = running;
}
