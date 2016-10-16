use gfx;
use gfx_device_gl;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type UniformMat4 = [[f32; 4]; 4];
pub type Texture<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;

pub type GLEncoder = gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>;
pub type RenderTarget = gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>;
pub type DepthTarget = gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>;
