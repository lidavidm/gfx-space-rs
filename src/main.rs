extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate time;

pub mod input;
pub mod sprite;
pub mod types;
pub mod player;

use types::*;

use cgmath::{Rotation3, SquareMatrix};
use gfx::Device;

const TICK_TIME: u64 = 20 * 1000000;
const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

struct Input {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

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

    let sprite_factory = sprite::SpriteFactory::new(&mut factory);
    let texture = sprite::load_texture(&mut factory, std::path::Path::new("assets/textures/tankBlue_outline.png")).unwrap();
    let barrel_texture = sprite::load_texture(&mut factory, std::path::Path::new("assets/textures/barrelBlue_outline.png")).unwrap();
    let sprite = sprite_factory.create(&mut factory, main_color.clone(), texture.clone(), 16.0, 16.0);
    let barrel = sprite_factory.create(&mut factory, main_color.clone(), barrel_texture.clone(), 2.4, 5.8);

    let mut player = player::Player::new(sprite, barrel);

    let inputs = Input { forward: false, backward: false, left: false, right: false };

    let mut prev = time::precise_time_ns();
    let mut accum = 0;
    let mut x = 0.0;
    let mut y = 0.0;
    let center_x = 64.0;
    let center_y = 36.0;
    'outer: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'outer,
                glutin::Event::MouseMoved(mx, my) => {
                    let my = 720 - my;
                    x = (mx as f32 / 1280.0) * 128.0 - view[3][0];
                    y = (my as f32 / 720.0) * 72.0 - view[3][1];
                    player.mouse_moved(mx, my, x, y);
                    // let cx = center_x;
                    // let cy = center_y;
                    // let angle = f32::atan2(y - cy, x - cx) - std::f32::consts::PI / 2.0;
                    // barrel.rotation = cgmath::Basis3::from_angle_z(cgmath::Rad { s: angle });
                }
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, code, _) => {
                    match code {
                        25 => view[3][1] -= 1.0,
                        39 => view[3][1] += 1.0,
                        38 => view[3][0] += 1.0,
                        40 => view[3][0] -= 1.0,
                        _ => {},
                    }
                }
                _ => {},
            }
        }

        let cur = time::precise_time_ns();
        accum += cur - prev;
        prev = cur;
        while accum > TICK_TIME {
            accum -= TICK_TIME;
        }

        encoder.clear(&main_color, BG_COLOR);
        player.render(&mut encoder, proj, view);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
