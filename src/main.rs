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

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const WORLD_WIDTH: f32 = 1280.0;
const WORLD_HEIGHT: f32 = 720.0;

pub fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Test".to_string())
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_vsync();

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let proj = cgmath::ortho(0.0, WORLD_WIDTH, 0.0, WORLD_HEIGHT, 0.0, 1.0).into();
    let mut view: UniformMat4 = cgmath::Matrix4::identity().into();

    let sprite_factory = sprite::SpriteFactory::new(&mut factory);
    let texture = sprite::load_texture(&mut factory, std::path::Path::new("assets/textures/tankBlue_outline.png")).unwrap();
    let barrel_texture = sprite::load_texture(&mut factory, std::path::Path::new("assets/textures/barrelBlue_outline.png")).unwrap();
    let sprite = sprite_factory.create(&mut factory, main_color.clone(), texture.clone(), 32.0, 32.0);
    let barrel = sprite_factory.create(&mut factory, main_color.clone(), barrel_texture.clone(), 12.0, 27.0);

    let mut player = player::Player::new(sprite, barrel);

    let mut input = input::Input::new();

    let mut prev = time::precise_time_ns();
    let mut accum = 0;
    'outer: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'outer,
                glutin::Event::MouseMoved(mx, my) => {
                    let my = WINDOW_HEIGHT as i32 - my;
                    let x = (mx as f32 / WINDOW_WIDTH as f32) * WORLD_WIDTH - view[3][0];
                    let y = (my as f32 / WINDOW_HEIGHT as f32) * WORLD_HEIGHT - view[3][1];
                    input.mouse_moved(mx, my as i32, x, y);
                }
                glutin::Event::KeyboardInput(state, code, vcode) => {
                    input.key_pressed(state, code, vcode);
                }
                _ => {},
            }
        }

        let cur = time::precise_time_ns();
        accum += cur - prev;
        prev = cur;
        while accum > TICK_TIME {
            accum -= TICK_TIME;
            player.update(&input);
            if input.action {
                let (x, y) = player.position();
                view[3][0] = -x + WORLD_WIDTH / 2.0 - 16.0;
                view[3][1] = -y + WORLD_HEIGHT / 2.0 - 16.0;
            }
        }

        encoder.clear(&main_color, BG_COLOR);
        player.render(&mut encoder, proj, view);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
