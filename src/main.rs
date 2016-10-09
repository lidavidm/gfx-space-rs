extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate time;

pub mod sprite;
pub mod types;

use types::*;

use cgmath::{Rotation3, SquareMatrix};
use gfx::Device;

const TICK_TIME: u64 = 20 * 1000000;
const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Test".to_string())
        .with_dimensions(1280, 720)
        .with_vsync();

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let proj = cgmath::ortho(0.0, 128.0, 0.0, 72.0, 0.0, 1.0).into();
    let mut view: UniformMat4 = cgmath::Matrix4::identity().into();

    let texture = sprite::load_texture(&mut factory, std::path::Path::new("assets/textures/tankBlue_outline.png")).unwrap();
    let mut sprite = sprite::Sprite::new(&mut factory, main_color.clone(), texture.clone(), 16.0, 16.0);
    let mut sprite2 = sprite::Sprite::new(&mut factory, main_color.clone(), texture.clone(), 16.0, 16.0);
    sprite.scale = 0.5;

    let mut prev = time::precise_time_ns();
    let mut accum = 0;
    let mut angle = 0.0;
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
            sprite.position.x += 0.1;
            sprite2.rotation = cgmath::Basis3::from_angle_z(cgmath::Rad { s: angle });
            angle += std::f32::consts::PI / 40.0;
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
