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
use gfx::Factory;

use mgmm::circle::Circle;
pub use mgmm::types::*;

const BG_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;

const WORLD_WIDTH: f32 = 480.0;
const WORLD_HEIGHT: f32 = 320.0;

type R = gfx_device_gl::Resources;

gfx_defines! {
    vertex BlurVertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant BlurLocals {
        proj: UniformMat4 = "u_Proj",
        model: UniformMat4 = "u_Model",
    }

    pipeline blur {
        vbuf: gfx::VertexBuffer<BlurVertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        locals: gfx::ConstantBuffer<BlurLocals> = "Locals",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

struct Game {
    proj: UniformMat4,
    view: UniformMat4,
    circle: Circle<R>,
}

impl mgmm::game::Game for Game {
    fn init(factory: &mut gfx_device_gl::Factory, main_color: &RenderTarget, _main_depth: &DepthTarget) -> Game {
        let proj: UniformMat4 = cgmath::ortho(0.0, WORLD_WIDTH, 0.0, WORLD_HEIGHT, 0.0, 1.0).into();
        let view: UniformMat4 = cgmath::Matrix4::identity().into();

        let (buf_width, buf_height, _, _) = main_color.get_dimensions();
        // TODO: want a non-sRGB intermediate buffer
        let (_, srv, rtv) = factory.create_render_target::<ColorFormat>(buf_width, buf_height).unwrap();

        let circle = Circle::new(
            factory,
            main_color.clone(),
            [1.0, 0.0, 0.0],
            10.0,
        );

        Game {
            proj: proj,
            view: view,
            circle: circle,
        }
    }

    fn tick(&mut self) {

    }

    fn handle_event(&mut self, event: &glutin::Event) {

    }

    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget) {
        encoder.clear(target, BG_COLOR);
        self.circle.render(encoder, self.proj, self.view);
    }
}

pub fn main() {
    let mut game = mgmm::game::App::<Game>::new("Blur", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
