extern crate glm;
extern crate winit;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

extern crate gfx_hal;

pub use glm::*;

pub mod render;

struct Game {
    render: render::context::RenderContext<back::Backend>,
}

fn main() {
    println!("Starting Luminite...");
    let mut game = Game {
        render: render::create_context(),
    };

    main_loop(&mut game);
}

fn main_loop(game: &mut Game) {
    let mut running = true;
    while running {

    }
}
