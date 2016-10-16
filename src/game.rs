use gfx::{self, Device};
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use time;

use types::*;

pub const TICK_TIME: u64 = 20 * 1000000;

pub trait Game {
    fn init(factory: &mut gfx_device_gl::Factory, main_color: &RenderTarget, main_depth: &DepthTarget) -> Self;
    fn tick(&mut self);
    fn handle_event(&mut self, _event: &glutin::Event) {}
    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget);
}

pub struct App<G>
    where G: Game {
    game: G,
    main_color: RenderTarget,
    main_depth: DepthTarget,
    device: gfx_device_gl::Device,
    factory: gfx_device_gl::Factory,
    window: glutin::Window,
}

impl<G> App<G>
    where G: Game {
    pub fn new(title: &str, width: u32, height: u32) -> App<G> {
        let builder = glutin::WindowBuilder::new()
            .with_title(title.to_string())
            .with_dimensions(width, height)
            .with_vsync();

        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
        App {
            game: G::init(&mut factory, &main_color, &main_depth),
            main_color: main_color,
            main_depth: main_depth,
            device: device,
            factory: factory,
            window: window,
        }
    }

    pub fn run(&mut self) {
        let mut encoder: gfx::Encoder<_, _> = self.factory.create_command_buffer().into();
        let mut prev = time::precise_time_ns();
        let mut accum = 0;

        'outer: loop {
            for event in self.window.poll_events() {
                match event {
                    glutin::Event::Closed => break 'outer,
                    _ => self.game.handle_event(&event),
                }
            }

            let cur = time::precise_time_ns();
            accum += cur - prev;
            prev = cur;
            while accum > TICK_TIME {
                accum -= TICK_TIME;
                self.game.tick();
            }

            self.game.render(&mut encoder, &self.main_color);

            encoder.flush(&mut self.device);
            self.window.swap_buffers().unwrap();
            self.device.cleanup();
        }
    }
}
