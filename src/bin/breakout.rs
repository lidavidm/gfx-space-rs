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

const PI: f32 = std::f32::consts::PI;

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
    launch: bool,
}

struct CollisionDirection {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

enum CollisionLocation {
    Hit(f32, f32),
    Miss,
}

impl CollisionLocation {
    fn check(new_x: f32, new_y: f32, r: f32, rect: &Rectangle<R>) -> CollisionLocation {
        let closest_x = f32::max(rect.position.x, f32::min(new_x, rect.position.x + rect.width));
        let closest_y = f32::max(rect.position.y, f32::min(new_y, rect.position.y + rect.height));

        // Check whether the distance is less than the radius
        let d2 = (closest_x - new_x).powi(2) + (closest_y - new_y).powi(2);

        if d2 < r.powi(2) {
            CollisionLocation::Hit(closest_x, closest_y)
        }
        else {
            CollisionLocation::Miss
        }
    }
}

impl CollisionDirection {
    fn check_multiple(new_x: f32, new_y: f32, r: f32, rects: &mut Vec<Rectangle<R>>) -> CollisionDirection {
        let mut top = false;
        let mut bottom = false;
        let mut left = false;
        let mut right = false;

        rects.retain(|ref rect| {
            match CollisionLocation::check(new_x, new_y, r, &rect) {
                CollisionLocation::Hit(closest_x, closest_y) => {
                    if closest_y >= rect.position.y + rect.height {
                        bottom = true;
                    }
                    else if closest_y <= rect.position.y {
                        top = true;
                    }

                    if closest_x <= rect.position.x {
                        right = true;
                    }
                    else if closest_x >= rect.position.x + rect.width {
                        left = true;
                    }
                    false
                },
                CollisionLocation::Miss => true,
            }
        });

        CollisionDirection {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
        }
    }
}

struct Game {
    proj: UniformMat4,
    view: UniformMat4,
    paddle: Paddle,
    blocks: Vec<Rectangle<R>>,
    ball: Circle<R>,
    ball_speed: f32,
    ball_angle: f32,
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
            ball_speed: 0.0,
            ball_angle: 0.0,
            input: Input { left: false, right: false, launch: false, },
        }
    }

    fn tick(&mut self) {
        let delta_paddle = if self.input.left && self.paddle.rect.position.x > 0.0 {
            -PADDLE_VELOCITY
        }
        else if self.input.right && self.paddle.rect.position.x + PADDLE_WIDTH < WORLD_WIDTH {
            PADDLE_VELOCITY
        }
        else {
            0.0
        };
        self.paddle.rect.position.x += delta_paddle;

        // Ball is "sticky" when on the paddle
        if self.ball.position.y <= PADDLE_HEIGHT + 2.0 {
            self.ball.position.x += delta_paddle;
        }

        if self.input.launch {
            self.ball_angle = if delta_paddle == 0.0 {
                std::f32::consts::PI / 2.0
            } else if delta_paddle > 0.0 {
                std::f32::consts::PI / 4.0
            } else {
                0.75 * std::f32::consts::PI
            };
            self.ball_speed = 2.0;
        }

        let ball_dx = self.ball_speed * f32::cos(self.ball_angle);
        let ball_dy = self.ball_speed * f32::sin(self.ball_angle);

        // Figure out where the ball will be
        let new_x = self.ball.position.x + ball_dx;
        let new_y = self.ball.position.y + ball_dy;

        // Check collisions with bricks
        // Add the radius, because the origin of the ball's frame is
        // the lower-left corner of its bounding box
        let mut collisions = CollisionDirection::check_multiple(
            new_x + self.ball.r, new_y + self.ball.r,
            self.ball.r, &mut self.blocks);

        // Check collisions with walls
        if new_x + 2.0 * self.ball.r >= WORLD_WIDTH {
            collisions.right = true;
        }
        if new_x <= 0.0 {
            collisions.left = true;
        }
        if new_y + 2.0 * self.ball.r >= WORLD_HEIGHT {
            collisions.top = true;
        }

        // Check collisions with floor
        if new_y <= 0.0 {
            self.ball_speed = 0.0;
            self.ball.position.x = self.paddle.rect.position.x + PADDLE_WIDTH / 2.0 - BALL_RADIUS;
            self.ball.position.y = PADDLE_HEIGHT;
            return;
        }

        // Check collisions with paddle
        match CollisionLocation::check(new_x + self.ball.r, new_y + self.ball.r, self.ball.r, &self.paddle.rect) {
            CollisionLocation::Hit(closest_x, closest_y) => {
                if self.ball_angle < PI {

                }
                else if self.ball_angle < 1.5 * PI {
                    self.ball_angle -= 0.5 * PI;
                }
                else {
                    self.ball_angle = PI - (self.ball_angle - PI);
                }
            },
            CollisionLocation::Miss => {},
        }

        if collisions.top || collisions.bottom || collisions.left || collisions.right {
            if collisions.top && collisions.bottom {
                self.ball_angle = if self.ball_angle <= PI / 2.0 || self.ball_angle >= 1.5 * PI {
                    0.0
                } else {
                    PI
                }
            }
            else if collisions.top && self.ball_angle < PI {
                self.ball_angle = PI + (PI - self.ball_angle);
            }
            else if collisions.bottom && self.ball_angle > PI {
                self.ball_angle = PI - (self.ball_angle - PI);
            }

            if collisions.left && collisions.right {
                self.ball_angle = if self.ball_angle >= 0.0 && self.ball_angle <= PI {
                    PI / 2.0
                } else {
                    3.0 * PI / 2.0
                }
            }
            else if collisions.left && self.ball_angle > PI / 2.0 && self.ball_angle < 1.5 * PI {
                self.ball_angle = if self.ball_angle <= PI {
                    self.ball_angle - PI / 2.0
                } else {
                    self.ball_angle - PI
                };
            }
            else if collisions.right && (self.ball_angle < PI / 2.0 || self.ball_angle > 1.5 * PI) {
                self.ball_angle = if self.ball_angle < PI / 2.0 {
                    self.ball_angle + PI / 2.0
                } else {
                    self.ball_angle - PI / 2.0
                };
            }

            let ball_dx = self.ball_speed * f32::cos(self.ball_angle);
            let ball_dy = self.ball_speed * f32::sin(self.ball_angle);
            let new_x = self.ball.position.x + ball_dx;
            let new_y = self.ball.position.y + ball_dy;
            self.ball.position.x = new_x;
            self.ball.position.y = new_y;

            // Each bounce also increases speed
            self.ball_speed += 0.5;
        }
        else {
            self.ball.position.x = new_x;
            self.ball.position.y = new_y;
        }
    }

    fn handle_event(&mut self, event: &glutin::Event) {
        match *event {
            glutin::Event::KeyboardInput(state, code, _vcode) => {
                match code {
                    38 => self.input.left = state == glutin::ElementState::Pressed,
                    40 => self.input.right = state == glutin::ElementState::Pressed,
                    65 => self.input.launch = state == glutin::ElementState::Pressed,
                    _ => {},
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
