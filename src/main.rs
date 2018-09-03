extern crate glm;
extern crate log;
extern crate num;
extern crate rand;
extern crate simple_logger;
extern crate tobj;
extern crate winit;

#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(all(feature = "vulkan", not(any(feature = "dx12", feature = "metal"))))]
extern crate gfx_backend_vulkan as back;
#[cfg(feature = "gl")]
#[macro_use]
extern crate glium;

#[cfg(not(feature = "gl"))]
extern crate gfx_hal;

pub use glm::*;
use world::{Location, Object, World};
// Trait
use render::RenderBackend;
// Type
use render::_RenderBackend;

pub mod maze;
pub mod render;
pub mod world;

pub struct Game<B: RenderBackend> {
    pub render: B::RenderContext,
    pub world: World<B>,
    pub running: bool,
}

fn main() {
    simple_logger::init().unwrap();
    let mut game: Game<_RenderBackend> = Game {
        render: render::create_context::<_RenderBackend>("Luminite", (720, 480)),
        world: World::new(),
        running: true,
    };

    let cube = Object::new(
        &mut game.world,
        render::create_obj_render(0, 0, &mut game.render),
        Location::new(0.0, 0.0, 0.0),
    );

    game.world.add_object(cube);

    let sword = Object::new(
        &mut game.world,
        render::create_obj_render(1, 0, &mut game.render),
        Location::new(0.0, -1.0, 2.0).with_rot(45.0, -90.0),
    );

    game.world.add_object(sword);

    main_loop(&mut game);
}

fn main_loop(game: &mut Game<_RenderBackend>) {
    while game.running {
        poll_events(game);
        game.world.tick();
        render::render(&mut game.render, &mut game.world);
    }
}

/// Polls events
#[cfg(not(feature = "gl"))]
fn poll_events(game: &mut Game<_RenderBackend>) {
    let mut running = true;
    let events_loop = &mut game.render.events_loop;
    events_loop.poll_events(|event| match event {
        winit::Event::WindowEvent { event, .. } => match event {
            winit::WindowEvent::CloseRequested => running = false,
            _ => (),
        },
        _ => (),
    });
    game.running = running;
}

/// Polls events
#[cfg(feature = "gl")]
fn poll_events(game: &mut Game<_RenderBackend>) {
    let mut running = true;
    let events_loop = &mut game.render.events_loop;
    events_loop.poll_events(|event| match event {
        glium::glutin::Event::WindowEvent { event, .. } => match event {
            glium::glutin::WindowEvent::CloseRequested => running = false,
            _ => (),
        },
        _ => (),
    });
    game.running = running;
}
