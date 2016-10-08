extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate time;

use cgmath::SquareMatrix;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type UniformMat4 = [[f32; 4]; 4];

const TICK_TIME: u64 = 20 * 1000000;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Locals {
        proj: UniformMat4 = "u_Proj",
        view: UniformMat4 = "u_View",
        model: UniformMat4 = "u_Model",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        model: gfx::Global<UniformMat4> = "u_Model",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 4] = [
    Vertex { pos: [0.0, 0.0], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [10.0, 0.0], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [0.0, 10.0], color: [1.0, 1.0, 1.0] },
    Vertex { pos: [10.0, 10.0], color: [1.0, 1.0, 1.0] },
];

const TRIANGLE_INDICES: [u16; 6] = [
    0, 1, 3,
    0, 3, 2,
];

const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct App {

}

struct Sprite<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
}

impl<R: gfx::Resources> Sprite<R> {
    // TODO: model matrix
    fn new<F>(factory: &mut F,
              target: gfx::handle::RenderTargetView<R, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>) -> Self
        where F: gfx::Factory<R> {
        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/sprite_150.glslv"),
            include_bytes!("shader/sprite_150.glslf"),
            pipe::new(),
        ).unwrap();

        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, &TRIANGLE_INDICES as &[u16]);
        let data = pipe::Data {
            vbuf: vertex_buffer,
            locals: factory.create_constant_buffer(1),
            model: cgmath::Matrix4::identity().into(),
            out: target,
        };

        Sprite {
            pso: pso,
            data: data,
            slice: slice,
        }
    }

    fn render<C>(&mut self,
                 encoder: &mut gfx::Encoder<R, C>,
                 proj: UniformMat4,
                 view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        let locals = Locals {
            proj: proj,
            view: view,
            model: self.data.model,
        };

        encoder.update_buffer(&self.data.locals, &[locals], 0).unwrap();
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Test".to_string())
        .with_dimensions(1280, 720)
        .with_vsync();

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let proj = cgmath::ortho(0.0, 1280.0, 0.0, 720.0, 0.0, 1.0).into();
    let mut view: UniformMat4 = cgmath::Matrix4::identity().into();

    let mut sprite = Sprite::new(&mut factory, main_color.clone());
    let mut sprite2 = Sprite::new(&mut factory, main_color.clone());

    let mut prev = time::precise_time_ns();
    let mut accum = 0;
    'outer: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'outer,
                _ => {},
            }
        }

        let cur = time::precise_time_ns();
        accum += cur - prev;
        prev = cur;
        while accum > TICK_TIME {
            accum -= TICK_TIME;
            sprite.data.model[3][0] += 0.1;
            view[3][1] += 0.1;
        }

        encoder.clear(&main_color, BG_COLOR);
        sprite.render(&mut encoder, proj, view);
        sprite2.render(&mut encoder, proj, view);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
