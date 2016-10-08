use cgmath::{self, SquareMatrix};
use gfx;
use gfx::traits::FactoryExt;

// gfx_defines! creates a submodule, so we need `pub use` to make sure
// the import here is visible.
pub use types::*;

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

pub struct Sprite<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    pub data: pipe::Data<R>,
    slice: gfx::Slice<R>,
}

impl<R: gfx::Resources> Sprite<R> {
    pub fn new<F>(factory: &mut F,
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

    pub fn render<C>(&mut self,
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
