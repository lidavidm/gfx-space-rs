use cgmath;
use gfx;
use gfx::traits::FactoryExt;

pub use types::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
    }

    constant Locals {
        proj: UniformMat4 = "u_Proj",
        view: UniformMat4 = "u_View",
        model: UniformMat4 = "u_Model",
        color: [f32; 3] = "u_Color",
        point_size: f32 = "u_PointSize",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

pub struct Circle<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    pub position: cgmath::Vector3<f32>,
    pub r: f32,
    pub color: [f32; 3],
}

impl<R: gfx::Resources> Circle<R> {
    pub fn new<F>(
        factory: &mut F,
        target: gfx::handle::RenderTargetView<R, ColorFormat>,
        color: [f32; 3],
        r: f32) -> Circle<R>
        where F: gfx::Factory<R> {
        let vertices = [
            Vertex { pos: [0.0, 0.0] },
        ];
        let indices: [u16; 1] = [
            0,
        ];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
            &vertices, &indices as &[u16]);
        // Shaders based on
        // http://www.desultoryquest.com/blog/drawing-anti-aliased-circular-points-using-opengl-slash-webgl/
        let shader_set = factory.create_shader_set(
            include_bytes!("shader/circle_150.glslv"),
            include_bytes!("shader/circle_150.glslf")).unwrap();
        let pso = factory.create_pipeline_state(
            &shader_set,
            gfx::Primitive::PointList,
            gfx::state::Rasterizer::new_fill(),
            pipe::new(),
        ).unwrap();
        let data = pipe::Data {
            vbuf: vertex_buffer,
            locals: factory.create_constant_buffer(1),
            out: target,
        };

        Circle {
            pso: pso,
            data: data,
            slice: slice,
            position: cgmath::vec3(0.0, 0.0, 0.0),
            r: r,
            color: color,
        }
    }

    pub fn render<C>(&mut self,
                 encoder: &mut gfx::Encoder<R, C>,
                 proj: UniformMat4,
                 view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        // TODO: cache recomputation of model matrix where possible
        let translate_to_position = cgmath::Matrix4::from_translation(self.position + cgmath::vec3(self.r, self.r, 0.0));

        let model = translate_to_position;

        let locals = Locals {
            proj: proj,
            view: view,
            model: model.into(),
            color: self.color,
            point_size: 4.0 * self.r,
        };

        encoder.update_buffer(&self.data.locals, &[locals], 0).unwrap();
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
