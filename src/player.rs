use ::std::f32;

use cgmath;
use gfx;

use cgmath::Rotation3;

use input::Input;
use sprite::Sprite;
use types::*;

pub struct Player<'a, R>
    where R: gfx::Resources {
    sprite: Sprite<'a, R>,
    barrel: Sprite<'a, R>,
    velocity: f32,
    angular_velocity: f32,
    angle: f32,
}

impl<'a, R> Player<'a, R>
    where R: gfx::Resources {
    pub fn new(mut sprite: Sprite<'a, R>, mut barrel: Sprite<'a, R>) -> Player<'a, R> {
        barrel.position.x = sprite.width / 2.0 - barrel.width / 2.0;
        barrel.position.y = sprite.height / 2.0;
        barrel.rotation_center = cgmath::vec3(barrel.width / 2.0, 0.0, 0.0);
        sprite.rotation_center = cgmath::vec3(sprite.width / 2.0, sprite.height / 2.0, 0.0);
        Player {
            sprite: sprite,
            barrel: barrel,
            velocity: 0.0,
            angular_velocity: 0.0,
            angle: 0.0,
        }
    }

    pub fn update(&mut self, input: &Input) {
        let angle = self.angle + f32::consts::PI / 2.0;
        let dx = self.velocity * f32::cos(angle);
        let dy = self.velocity * f32::sin(angle);
        self.sprite.position.x += dx;
        self.barrel.position.x += dx;
        self.sprite.position.y += dy;
        self.barrel.position.y += dy;
        self.angle += self.angular_velocity;
        self.sprite.rotation = cgmath::Basis3::from_angle_z(cgmath::Rad { s: self.angle });

        if input.forward || input.backward {
            if input.forward {
                self.velocity = 1.0;
            }
            if input.backward {
                self.velocity = -1.0;
            }
        }
        else if self.velocity > 0.0 {
            self.velocity = f32::max(self.velocity - 0.01, 0.0);
        }
        else if self.velocity < 0.0 {
            self.velocity = f32::min(self.velocity + 0.01, 0.0);
        }

        if input.left {
            self.angular_velocity = 0.01;
        }
        else if input.right {
            self.angular_velocity = -0.01;
        }
        else {
            self.angular_velocity = 0.0;
        }
    }

    pub fn mouse_moved(&mut self, win_x: i32, win_y: i32, world_x: f32, world_y: f32) {
        let cx = self.barrel.position.x + self.barrel.rotation_center.x;
        let cy = self.barrel.position.y;
        let angle = f32::atan2(world_y - cy, world_x - cx) - f32::consts::PI / 2.0;
        self.barrel.rotation = cgmath::Basis3::from_angle_z(cgmath::Rad { s: angle });
    }

    pub fn render<C>(&mut self,
                     encoder: &mut gfx::Encoder<R, C>,
                     proj: UniformMat4,
                     view: UniformMat4)
        where C: gfx::CommandBuffer<R> {
        self.sprite.render(encoder, proj, view);
        self.barrel.render(encoder, proj, view);
    }
}
