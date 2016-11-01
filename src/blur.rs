use gfx::{self, Bundle, texture};
use gfx::traits::FactoryExt;
pub use types::*;

gfx_defines! {
    vertex BlurVertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant BlurLocals {
        proj: UniformMat4 = "u_Proj",
        strength: f32 = "u_Strength",
    }

    pipeline blur {
        vbuf: gfx::VertexBuffer<BlurVertex> = (),
        texture: gfx::TextureSampler<[f32; 4]> = "t_Texture",
        locals: gfx::ConstantBuffer<BlurLocals> = "Locals",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

pub struct Blur<R: gfx::Resources> {
    bundle: Bundle<R, blur::Data<R>>,
    pub strength: f32,
    pub rtv: gfx::handle::RenderTargetView<R, ColorFormat>,
}

impl<R: gfx::Resources> Blur<R> {
    pub fn new<F>(factory: &mut F, target: &gfx::handle::RenderTargetView<R, ColorFormat>, width: f32, height: f32) -> Blur<R> where F: gfx::Factory<R> {
         let (buf_width, buf_height, _, _) = target.get_dimensions();
        // TODO: want a non-sRGB intermediate buffer
        let (_, srv, rtv) = factory.create_render_target::<ColorFormat>(buf_width, buf_height).unwrap();

        let vertices = [
            BlurVertex { pos: [0.0, 0.0], uv: [0.0, 0.0] },
            BlurVertex { pos: [width, 0.0], uv: [1.0, 0.0] },
            BlurVertex { pos: [0.0, height], uv: [0.0, 1.0] },
            BlurVertex { pos: [width, height], uv: [1.0, 1.0] },
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
            out: target.clone(),
        };
        let bundle = Bundle::new(slice, pso, data);

        Blur {
            bundle: bundle,
            strength: 0.0,
            rtv: rtv,
        }
    }

    pub fn render<C>(&mut self,
                 encoder: &mut gfx::Encoder<R, C>,
                 proj: UniformMat4,
                 view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        let locals = BlurLocals {
            proj: proj,
            strength: self.strength,
        };

        encoder.update_buffer(&self.bundle.data.locals, &[locals], 0).unwrap();
        self.bundle.encode(encoder);
    }
}
