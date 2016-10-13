use gfx;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type UniformMat4 = [[f32; 4]; 4];
pub type Texture<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;
