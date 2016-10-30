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
use gfx::texture;
use gfx::Factory;
use gfx::traits::FactoryExt;

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

    blur: Bundle<R, blur::Data<R>>,
    rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
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
            rtv.clone(),
            [1.0, 0.0, 0.0],
            10.0,
        );

        let vertices = [
            BlurVertex { pos: [0.0, 0.0], uv: [0.0, 0.0] },
            BlurVertex { pos: [WORLD_WIDTH, 0.0], uv: [1.0, 0.0] },
            BlurVertex { pos: [0.0, WORLD_HEIGHT], uv: [0.0, 1.0] },
            BlurVertex { pos: [WORLD_WIDTH, WORLD_HEIGHT], uv: [1.0, 1.0] },
        ];
        let indices: [u16; 6] = [ 0, 1, 3, 0, 3, 2 ];
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, &indices as &[u16]);
        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/blur_150.glslv"),
            include_bytes!("shader/blur_150.glslf"),
            blur::new()).unwrap();
        let sampler = factory.create_sampler(
            texture::SamplerInfo::new(texture::FilterMethod::Scale,
                                      texture::WrapMode::Clamp)
        );
        let data = blur::Data {
            vbuf: vbuf,
            texture: (srv, sampler),
            locals: factory.create_constant_buffer(1),
            out: main_color.clone(),
        };

        Game {
            proj: proj,
            view: view,
            circle: circle,

            blur: Bundle::new(slice, pso, data),
            rtv: rtv,
        }
    }

    fn tick(&mut self) {

    }

    fn handle_event(&mut self, event: &glutin::Event) {

    }

    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget) {
        encoder.clear(&self.rtv, [0.0, 0.0, 0.0, 0.0]);
        encoder.clear(target, BG_COLOR);
        self.circle.render(encoder, self.proj, self.view);
        let locals = BlurLocals {
            proj: self.proj,
        };

        encoder.update_buffer(&self.blur.data.locals, &[locals], 0).unwrap();
        self.blur.encode(encoder);
    }
}

pub fn main() {
    let mut game = mgmm::game::App::<Game>::new("Blur", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
