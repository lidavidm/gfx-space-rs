extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate time;

extern crate mgmm;

use mgmm::rectangle::Rectangle;
use mgmm::circle::Circle;
use mgmm::types::*;

use cgmath::{SquareMatrix};

const BG_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32 = 640;

const WORLD_WIDTH: f32 = 480.0;
const WORLD_HEIGHT: f32 = 320.0;

const PADDLE_VELOCITY: f32 = 2.0;
const PADDLE_WIDTH: f32 = 64.0;
const PADDLE_HEIGHT: f32 = 16.0;

const BLOCK_WIDTH: f32 = 32.0;
const BLOCK_HEIGHT: f32 = 16.0;

const BALL_RADIUS: f32 = 8.0;

type R = gfx_device_gl::Resources;

struct Paddle {
    rect: Rectangle<R>,
}

impl Paddle {
    fn new(rect: Rectangle<R>) -> Paddle {
        Paddle {
            rect: rect,
        }
    }

    pub fn render<C>(&mut self,
                     encoder: &mut gfx::Encoder<R, C>,
                     proj: UniformMat4,
                     view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        self.rect.render(encoder, proj, view);
    }
}

struct Input {
    left: bool,
    right: bool,
}

struct Game {
    proj: UniformMat4,
    view: UniformMat4,
    paddle: Paddle,
    blocks: Vec<Rectangle<R>>,
    ball: Circle<R>,
    input: Input,
}

impl mgmm::game::Game for Game {
    fn init(factory: &mut gfx_device_gl::Factory, main_color: &RenderTarget, _main_depth: &DepthTarget) -> Game {
        let proj: UniformMat4 = cgmath::ortho(0.0, WORLD_WIDTH, 0.0, WORLD_HEIGHT, 0.0, 1.0).into();
        let view: UniformMat4 = cgmath::Matrix4::identity().into();

        let rectangle = Rectangle::new(
            factory,
            main_color.clone(),
            [1.0, 0.0, 0.0],
            PADDLE_WIDTH, PADDLE_HEIGHT
        );
        let mut ball = Circle::new(
            factory,
            main_color.clone(),
            [1.0, 0.0, 1.0],
            BALL_RADIUS,
        );
        ball.position.x = PADDLE_WIDTH / 2.0 - BALL_RADIUS;
        ball.position.y = PADDLE_HEIGHT;

        let mut blocks = vec![];
        for y in 0..6 {
            let top = if y % 2 == 0 { 8 } else { 7 };
            let left = (WORLD_WIDTH - (top as f32) * (BLOCK_WIDTH + 4.0) + 4.0) / 2.0;
            for x in 0..top {
                let mut block = Rectangle::new(
                    factory,
                    main_color.clone(),
                    [0.0, 0.0, 1.0],
                    BLOCK_WIDTH, BLOCK_HEIGHT
                );
                block.position.y = WORLD_HEIGHT - (y as f32) * (BLOCK_HEIGHT + 4.0);
                block.position.x = left + (BLOCK_WIDTH + 4.0) * (x as f32);
                blocks.push(block);
            }
        }

        Game {
            proj: proj,
            view: view,
            paddle: Paddle::new(rectangle),
            blocks: blocks,
            ball: ball,
            input: Input { left: false, right: false },
        }
    }

    fn tick(&mut self) {
        if self.input.left && self.paddle.rect.position.x > 0.0 {
            self.paddle.rect.position.x -= PADDLE_VELOCITY;
        }
        if self.input.right && self.paddle.rect.position.x + PADDLE_WIDTH < WORLD_WIDTH {
            self.paddle.rect.position.x += PADDLE_VELOCITY;
        }
    }

    fn handle_event(&mut self, event: &glutin::Event) {
        match *event {
            glutin::Event::KeyboardInput(state, code, _vcode) => {
                match code {
                    38 => self.input.left = state == glutin::ElementState::Pressed,
                    40 => self.input.right = state == glutin::ElementState::Pressed,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget) {
        encoder.clear(target, BG_COLOR);
        self.paddle.render(encoder, self.proj, self.view);
        self.ball.render(encoder, self.proj, self.view);
        for block in self.blocks.iter_mut() {
            block.render(encoder, self.proj, self.view);
        }
    }
}

pub fn main() {
    let mut game = mgmm::game::App::<Game>::new("Breakout", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
