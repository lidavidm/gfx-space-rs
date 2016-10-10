use ::std::f32;

use cgmath;
use gfx;

use cgmath::Rotation3;

use sprite::Sprite;
use types::*;

pub struct Player<'a, R>
    where R: gfx::Resources {
    sprite: Sprite<'a, R>,
    barrel: Sprite<'a, R>,
}

impl<'a, R> Player<'a, R>
    where R: gfx::Resources {
    pub fn new(mut sprite: Sprite<'a, R>, mut barrel: Sprite<'a, R>) -> Player<'a, R> {
        sprite.position.x = 56.0;
        sprite.position.y = 28.0;
        barrel.position.x = 64.0 - 1.2;
        barrel.position.y = 36.0;
        barrel.rotation_center = cgmath::vec3(1.2, 0.0, 0.0);
        Player {
            sprite: sprite,
            barrel: barrel,
        }
    }

    pub fn update(&mut self) {

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
