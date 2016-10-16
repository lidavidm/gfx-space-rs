extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
extern crate tiled;
extern crate time;

pub mod game;
pub mod input;
pub mod sprite;
pub mod tilemap;
pub mod types;
pub mod player;

use types::*;

use cgmath::{SquareMatrix};

const BG_COLOR: [f32; 4] = [0.529, 0.808, 0.980, 1.0];

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

const WORLD_WIDTH: f32 = 1280.0;
const WORLD_HEIGHT: f32 = 720.0;

type R = gfx_device_gl::Resources;
struct TankGame {
    input: input::Input,
    proj: UniformMat4,
    view: UniformMat4,
    sprite_factory: sprite::SpriteFactory<R>,
    player: player::Player<R>,
    tilemap: tilemap::Tilemap<R>,
    layers: Vec<tilemap::TilemapLayer<R>>,
}

impl game::Game for TankGame {
    fn init(factory: &mut gfx_device_gl::Factory, main_color: &RenderTarget, _main_depth: &DepthTarget) -> TankGame {
        let proj: UniformMat4 = cgmath::ortho(0.0, WORLD_WIDTH, 0.0, WORLD_HEIGHT, 0.0, 1.0).into();
        let view: UniformMat4 = cgmath::Matrix4::identity().into();

        let sprite_factory = sprite::SpriteFactory::new(factory);

        let texture = sprite::load_texture(factory, std::path::Path::new("assets/textures/tankBlue_outline.png")).unwrap();
        let barrel_texture = sprite::load_texture(factory, std::path::Path::new("assets/textures/barrelBlue_outline.png")).unwrap();
        let tileset = sprite::load_texture(factory, std::path::Path::new("assets/textures/mapPack_tilesheet.png")).unwrap();
        let tilemap = tilemap::load_tilemap(std::path::Path::new("assets/maps/test.tmx")).unwrap();

        let sprite = sprite_factory.create(factory, main_color.clone(), texture.clone(), 64.0, 64.0);
        let barrel = sprite_factory.create(factory, main_color.clone(), barrel_texture.clone(), 24.0, 52.0);

        let player = player::Player::new(sprite, barrel);
        let tilemap = tilemap::Tilemap::new(factory, tilemap, tileset);
        let layers = tilemap.create_layers(factory, main_color.clone());

        let input = input::Input::new();

        TankGame {
            input: input,
            proj: proj,
            view: view,
            sprite_factory: sprite_factory,
            player: player,
            tilemap: tilemap,
            layers: layers,
        }
    }

    fn tick(&mut self) {
        self.player.update(&self.input);
        if self.input.action {
            let (x, y) = self.player.position();
            self.view[3][0] = -x + WORLD_WIDTH / 2.0 - 16.0;
            self.view[3][1] = -y + WORLD_HEIGHT / 2.0 - 16.0;
        }
    }

    fn handle_event(&mut self, event: &glutin::Event) {
        match *event {
            glutin::Event::MouseMoved(mx, my) => {
                let my = WINDOW_HEIGHT as i32 - my;
                let x = (mx as f32 / WINDOW_WIDTH as f32) * WORLD_WIDTH - self.view[3][0];
                let y = (my as f32 / WINDOW_HEIGHT as f32) * WORLD_HEIGHT - self.view[3][1];
                self.input.mouse_moved(mx, my as i32, x, y);
            }
            glutin::Event::KeyboardInput(state, code, vcode) => {
                self.input.key_pressed(state, code, vcode);
            }
            _ => {},
        }
    }

    fn render(&mut self, encoder: &mut GLEncoder, target: &RenderTarget) {
        encoder.clear(target, BG_COLOR);

        for layer in self.layers.iter() {
            layer.render(encoder, self.proj, self.view);
        }

        self.player.render(encoder, self.proj, self.view);
    }
}

pub fn main() {
    let mut game = game::App::<TankGame>::new("Test", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
