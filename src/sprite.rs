use cgmath::{self, Rotation, SquareMatrix};
use gfx;
use gfx::traits::FactoryExt;

// gfx_defines! creates a submodule, so we need `pub use` to make sure
// the import here is visible.
pub use types::*;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
        uv: [f32; 2] = "a_Uv",
    }

    constant Locals {
        proj: UniformMat4 = "u_Proj",
        view: UniformMat4 = "u_View",
        model: UniformMat4 = "u_Model",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

const TRIANGLE: [Vertex; 4] = [
    Vertex { pos: [0.0, 0.0], color: [1.0, 1.0, 1.0], uv: [0.0, 1.0] },
    Vertex { pos: [1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [1.0, 1.0] },
    Vertex { pos: [0.0, 1.0], color: [1.0, 1.0, 1.0], uv: [0.0, 0.0] },
    Vertex { pos: [1.0, 1.0], color: [1.0, 1.0, 1.0], uv: [1.0, 0.0] },
];

const TRIANGLE_INDICES: [u16; 6] = [
    0, 1, 3,
    0, 3, 2,
];

pub fn load_texture<F, R, P>(factory: &mut F, path: P)
    -> Result<Texture<R>, String>
    where F: gfx::Factory<R>,
          R: gfx::Resources,
          P: AsRef<::std::path::Path> {
    use gfx::tex as t;

    let img = ::image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as u16, height as u16, t::AaMode::Single);
    let (_texture, resource) = factory.create_texture_const_u8::<ColorFormat>(kind, &[&img]).unwrap();

    Ok(resource)
}

pub struct SpriteFactory<R: gfx::Resources> {
    sampler: gfx::handle::Sampler<R>,
    pso: gfx::PipelineState<R, pipe::Meta>,
    vbuf: gfx::handle::Buffer<R, Vertex>,
    slice: gfx::Slice<R>,
}

impl<R> SpriteFactory<R>
    where R: gfx::Resources {
    pub fn new<F>(factory: &mut F) -> SpriteFactory<R>
        where F: gfx::Factory<R> {
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(
            &TRIANGLE, &TRIANGLE_INDICES as &[u16]);
        SpriteFactory {
            sampler: factory.create_sampler_linear(),
            pso: factory.create_pipeline_simple(
                include_bytes!("shader/sprite_150.glslv"),
                include_bytes!("shader/sprite_150.glslf"),
                pipe::new()).unwrap(),
            vbuf: vertex_buffer,
            slice: slice,
        }
    }

    pub fn create<F>(
        &self,
        factory: &mut F,
        target: gfx::handle::RenderTargetView<R, ColorFormat>,
        texture: Texture<R>,
        width: f32, height: f32) -> Sprite<R>
        where F: gfx::Factory<R> {
        Sprite::new(
            factory, &self.pso,
            self.vbuf.clone(), self.slice.clone(),
            self.sampler.clone(), target, texture,
            width, height)
    }
}

pub struct Sprite<'a, R: gfx::Resources> {
    pso: &'a gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    pub position: cgmath::Vector3<f32>,
    pub scale: f32,
    pub rotation: cgmath::Basis3<f32>,
    pub rotation_center: cgmath::Vector3<f32>,
    pub width: f32,
    pub height: f32,
}

impl<'a, R: gfx::Resources> Sprite<'a, R> {
    pub fn new<F>(
        factory: &mut F,
        pso: &'a gfx::PipelineState<R, pipe::Meta>,
        vbuf: gfx::handle::Buffer<R, Vertex>,
        slice: gfx::Slice<R>,
        sampler: gfx::handle::Sampler<R>,
        target: gfx::handle::RenderTargetView<R, ColorFormat>,
        texture: Texture<R>,
        width: f32,
        height: f32) -> Sprite<'a, R>
        where F: gfx::Factory<R> {
        let data = pipe::Data {
            vbuf: vbuf,
            texture: (texture, sampler),
            locals: factory.create_constant_buffer(1),
            out: target,
        };

        Sprite {
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
