extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate time;

extern crate mgmm;

use cgmath::SquareMatrix;
use gfx::Bundle;
use gfx::Factory;
use gfx::traits::FactoryExt;

use mgmm::circle::Circle;
use mgmm::blur::Blur;
pub use mgmm::types::*;

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
    blur: Blur<R>,
}

impl mgmm::game::Game for Game {
    fn init(factory: &mut gfx_device_gl::Factory, main_color: &RenderTarget, _main_depth: &DepthTarget) -> Game {
        let proj: UniformMat4 = cgmath::ortho(0.0, WORLD_WIDTH, 0.0, WORLD_HEIGHT, 0.0, 1.0).into();
        let view: UniformMat4 = cgmath::Matrix4::identity().into();

        let blur = Blur::new(factory, main_color, WORLD_WIDTH, WORLD_HEIGHT);
        let circle = Circle::new(
            factory,
            blur.rtv.clone(),
            [1.0, 0.0, 0.0],
            10.0,
        );

        Game {
            proj: proj,
            view: view,
            circle: circle,
            blur: blur,
        }
    }

    fn tick(&mut self) {

    }

    fn handle_event(&mut self, event: &glutin::Event) {

    }

    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget) {
        encoder.clear(&self.blur.rtv, [0.0, 0.0, 0.0, 0.0]);
        encoder.clear(target, BG_COLOR);
        self.circle.render(encoder, self.proj, self.view);
        self.blur.render(encoder, self.proj, self.view);
    }
}

pub fn main() {
    let mut game = mgmm::game::App::<Game>::new("Blur", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
