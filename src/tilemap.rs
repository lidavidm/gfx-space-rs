use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use cgmath::{self, Rotation, SquareMatrix};
use gfx;
use gfx::traits::FactoryExt;
use tiled;

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

pub fn load_tilemap<P>(path: P) -> Result<tiled::Map, String>
    where P: AsRef<Path> {
    let f = File::open(path).unwrap();
    let reader = BufReader::new(f);
    let map = tiled::parse(reader).unwrap();
    Ok(map)
}

pub struct Tilemap<R: gfx::Resources> {
    sampler: gfx::handle::Sampler<R>,
    pso: gfx::PipelineState<R, pipe::Meta>,
    tilemap: tiled::Map,
    tileset: Texture<R>,
}

pub struct TilemapLayer<'a, R: gfx::Resources> {
    pso: &'a gfx::PipelineState<R, pipe::Meta>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
}

impl<R> Tilemap<R>
    where R: gfx::Resources {
    pub fn new<F>(factory: &mut F, tilemap: tiled::Map, tileset: Texture<R>) -> Tilemap<R>
        where F: gfx::Factory<R> {
        Tilemap {
            sampler: factory.create_sampler_linear(),
            pso: factory.create_pipeline_simple(
                include_bytes!("shader/sprite_150.glslv"),
                include_bytes!("shader/sprite_150.glslf"),
                pipe::new()).unwrap(),
            tilemap: tilemap,
            tileset: tileset,
        }
    }

    pub fn create_layers<F>(
        &self,
        factory: &mut F,
        target: gfx::handle::RenderTargetView<R, ColorFormat>)
        -> Vec<TilemapLayer<R>>
        where F: gfx::Factory<R> {
        let mut result = Vec::new();
        for layer in self.tilemap.layers.iter() {
            let mut mesh = Vec::new();
            let mut slice = Vec::new();
            let mut y = 0.0;
            let mut offset: u16 = 0;
            for row in layer.tiles.iter().rev() {
                let mut x = 0.0;
                for tile in row {
                    mesh.push(Vertex { pos: [x, y], color: [1.0, 1.0, 1.0], uv: [0.0, 0.0] });
                    mesh.push(Vertex { pos: [x + 32.0, y], color: [1.0, 1.0, 1.0], uv: [1.0 / 17.0, 0.0] });
                    mesh.push(Vertex { pos: [x, y + 32.0], color: [1.0, 1.0, 1.0], uv: [0.0, 1.0 / 12.0] });
                    mesh.push(Vertex { pos: [x + 32.0, y + 32.0], color: [1.0, 1.0, 1.0], uv: [1.0 / 17.0, 1.0 / 12.0] });
                    slice.push(offset);
                    slice.push(offset + 1);
                    slice.push(offset + 3);
                    slice.push(offset);
                    slice.push(offset + 3);
                    slice.push(offset + 2);
                    x += 32.0;
                    offset += 4;
                }
                y += 32.0;
            }

            let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&mesh, slice.as_slice());

            result.push(TilemapLayer::new(factory, &self.pso, vertex_buffer, slice, self.sampler.clone(), target.clone(), self.tileset.clone()));
        }

        result
    }
}

impl<'a, R> TilemapLayer<'a, R>
    where R: gfx::Resources {
    fn new<F>(
        factory: &mut F,
        pso: &'a gfx::PipelineState<R, pipe::Meta>,
        vbuf: gfx::handle::Buffer<R, Vertex>,
        slice: gfx::Slice<R>,
        sampler: gfx::handle::Sampler<R>,
        target: gfx::handle::RenderTargetView<R, ColorFormat>,
        texture: Texture<R>) -> TilemapLayer<'a, R>
        where F: gfx::Factory<R> {
        let data = pipe::Data {
            vbuf: vbuf,
            texture: (texture, sampler),
            locals: factory.create_constant_buffer(1),
            out: target,
        };

        TilemapLayer {
            pso: pso,
            data: data,
            slice: slice,
        }
    }

    pub fn render<C>(&self,
                 encoder: &mut gfx::Encoder<R, C>,
                 proj: UniformMat4,
                 view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        let locals = Locals {
            proj: proj,
            view: view,
            model: cgmath::Matrix4::identity().into(),
        };

        encoder.update_buffer(&self.data.locals, &[locals], 0).unwrap();
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
