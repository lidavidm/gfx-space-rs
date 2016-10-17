use std::rc::Rc;

use cgmath::{self, Rotation};
use gfx;
use gfx::traits::FactoryExt;

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
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

pub struct Rectangle<R: gfx::Resources> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    pub position: cgmath::Vector3<f32>,
    pub scale: f32,
    pub rotation: cgmath::Basis3<f32>,
    pub rotation_center: cgmath::Vector3<f32>,
    pub width: f32,
    pub height: f32,
}

impl<R: gfx::Resources> Rectangle<R> {
    pub fn new<F>(
        factory: &mut F,
        target: gfx::handle::RenderTargetView<R, ColorFormat>,
        color: [f32; 3],
        width: f32,
        height: f32) -> Rectangle<R>
        where F: gfx::Factory<R> {
        let vertices = [
            Vertex { pos: [0.0, 0.0], color: color },
            Vertex { pos: [1.0, 0.0], color: color },
            Vertex { pos: [0.0, 1.0], color: color },
            Vertex { pos: [1.0, 1.0], color: color },
        ];
        let indices: [u16; 6] = [
            0, 1, 3,
            0, 3, 2,
        ];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
            &vertices, &indices as &[u16]);
        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/rectangle_150.glslv"),
            include_bytes!("shader/rectangle_150.glslf"),
            pipe::new()).unwrap();
        let data = pipe::Data {
            vbuf: vertex_buffer,
            locals: factory.create_constant_buffer(1),
            out: target,
        };

        Rectangle {
            pso: pso,
            data: data,
            slice: slice,
            position: cgmath::vec3(0.0, 0.0, 0.0),
            scale: 1.0,
            rotation: cgmath::Basis3::one(),
            rotation_center: cgmath::vec3(0.0, 0.0, 0.0),
            width: width,
            height: height,
        }
    }

    pub fn render<C>(&mut self,
                 encoder: &mut gfx::Encoder<R, C>,
                 proj: UniformMat4,
                 view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        // TODO: cache recomputation of model matrix where possible
        let translate_to_center = cgmath::Matrix4::from_translation(-self.rotation_center);
        let rotation: cgmath::Matrix4<f32> = cgmath::Decomposed {
            scale: 1.0,
            rot: self.rotation,
            disp: cgmath::vec3(0.0, 0.0, 0.0),
        }.into();
        let translate_from_center = cgmath::Matrix4::from_translation(self.rotation_center);
        let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale * self.width, self.scale * self.height, 1.0);
        let translate_to_position = cgmath::Matrix4::from_translation(self.position);

        let model = translate_to_position * translate_from_center * rotation * translate_to_center * scale;

        let locals = Locals {
            proj: proj,
            view: view,
            model: model.into(),
        };

        encoder.update_buffer(&self.data.locals, &[locals], 0).unwrap();
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
