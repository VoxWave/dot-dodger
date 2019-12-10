use std::collections::HashMap;
use std::convert::From;

use ggez::{
    graphics::{draw, spritebatch::SpriteBatch, DrawParam, Image},
    Context,
};
use specs::{Component, DenseVecStorage, Join, ReadStorage};

use ggez::mint::Point2;

use crate::physics::Position;
use crate::rendering::Visual::*;
#[derive(Component)]
pub enum Visual {
    Circle([f32; 4], f64),
    Sprite(String, f32),
}

pub struct Renderer {
    sprites: HashMap<String, Image>,
}

impl Renderer {
    pub fn new(c: &mut Context) -> Self {
        let mut sprites = HashMap::new();
        sprites.insert(
            String::from("bullet"),
            Image::new(c, "/bullet.png").unwrap(),
        );
        sprites.insert(String::from("player"), Image::new(c, "/ship.png").unwrap());

        Renderer { sprites }
    }
    pub fn render(
        &self,
        ctx: &mut Context,
        (positions, visuals): (ReadStorage<Position>, ReadStorage<Visual>),
    ) {
        let bullet_sprite = self.sprites.get("bullet").unwrap().clone();
        let bullet_offset_x = (bullet_sprite.width() as f32) / 2.;
        let bullet_offset_y = (bullet_sprite.height() as f32) / 2.;
        let mut bullet_batch = SpriteBatch::new(bullet_sprite);
        (&positions, &visuals).join().for_each(|(pos, vis)| {
            let screen_rect = ggez::graphics::screen_coordinates(ctx);
            let x = (pos.0.x + (screen_rect.w as f64) / 2.) as f32;
            let y = (-pos.0.y + (screen_rect.h as f64) / 2.) as f32;
            match vis {
                Circle(color, radius) => {}
                Sprite(img, scale) => {
                    match img.as_ref() {
                        "bullet" => {
                            bullet_batch.add(DrawParam::default().scale([*scale, *scale]).dest([x - scale*bullet_offset_x, y - scale*bullet_offset_y]));
                        }
                        _ => {
                            let sprite = self.sprites.get(img).unwrap();
                            let sprite_offset_x = (sprite.width() as f32) / 2.;
                            let sprite_offset_y = (sprite.height() as f32) / 2.;
                            draw(ctx, sprite, DrawParam::default().scale([*scale, *scale]).dest([x - scale*sprite_offset_x, y - scale*sprite_offset_y])).unwrap();
                        }
                    };
                }
            }
        });
        draw(ctx, &bullet_batch, DrawParam::default()).unwrap();
    }
}
