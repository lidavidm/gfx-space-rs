extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate time;

extern crate mgmm;

use mgmm::circle::Circle;
use mgmm::types::*;

use cgmath::{SquareMatrix};

const BG_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;

const WORLD_WIDTH: f32 = 480.0;
const WORLD_HEIGHT: f32 = 320.0;

type R = gfx_device_gl::Resources;

struct Game {
    proj: UniformMat4,
    view: UniformMat4,
    circle: Circle<R>,
}

impl mgmm::game::Game for Game {
    fn init(factory: &mut gfx_device_gl::Factory, main_color: &RenderTarget, _main_depth: &DepthTarget) -> Game {
        let proj: UniformMat4 = cgmath::ortho(0.0, WORLD_WIDTH, 0.0, WORLD_HEIGHT, 0.0, 1.0).into();
        let view: UniformMat4 = cgmath::Matrix4::identity().into();

        let circle = Circle::new(
            factory,
            main_color.clone(),
            [1.0, 0.0, 1.0],
            32.0, 32.0
        );

        Game {
            proj: proj,
            view: view,
            circle: circle,
        }
    }

    fn tick(&mut self) {}

    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget) {
        encoder.clear(target, BG_COLOR);
        self.circle.render(encoder, self.proj, self.view);
    }
}

pub fn main() {
    let mut game = mgmm::game::App::<Game>::new("Circle", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
