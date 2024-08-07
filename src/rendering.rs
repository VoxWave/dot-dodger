use std::collections::HashMap;
use std::convert::From;

use ggez::{
    graphics::{draw, spritebatch::SpriteBatch, Canvas, DrawParam, Image},
    Context,
};
use specs::{Component, DenseVecStorage, Entities, Join, ReadStorage};

use ggez::mint::Point2;

use crate::physics::Position;
use crate::rendering::Visual::*;
use crate::collision::Invul;

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
            Image::new(c.gfx.wgpu(), "/sprites/bullet.png").unwrap(),
        );
        sprites.insert(String::from("spiral_turret"), Image::new(c, "/sprites/spiral_turret.png").unwrap());
        sprites.insert(String::from("player"), Image::new(c, "/sprites/ship.png").unwrap());

        Renderer { sprites }
    }
    pub fn render(
        &self,
        canvas: &mut Canvas,
        (entities, positions, visuals, invuls): (Entities, ReadStorage<Position>, ReadStorage<Visual>, ReadStorage<Invul>),
    ) {
        let bullet_sprite = self.sprites.get("bullet").unwrap().clone();
        let mut bullet_batch = SpriteBatch::new(bullet_sprite);
        (&entities, &positions, &visuals).join().for_each(|(id, pos, vis)| {
            let screen_rect = canvas.screen_coordinates();
            let x = (pos.0.x + (screen_rect.w as f64) / 2.) as f32;
            let y = (-pos.0.y + (screen_rect.h as f64) / 2.) as f32;
            match vis {
                Circle(color, radius) => {}
                Sprite(img, scale) => {
                    match img.as_ref() {
                        "bullet" => {
                            bullet_batch.add(DrawParam::default().offset([0.5, 0.5]).scale([*scale, *scale]).dest([x, y]));
                        }
                        _ => {
                            let sprite = self.sprites.get(img).unwrap();
                            match invuls.get(id) {
                                Some(invul) => {
                                    if invul.0 % 2 == 0 {
                                        draw(ctx, sprite, DrawParam::default().offset([0.5, 0.5]).scale([*scale, *scale]).dest([x, y])).unwrap();
                                    }
                                },
                                None => draw(ctx, sprite, DrawParam::default().offset([0.5, 0.5]).scale([*scale, *scale]).dest([x, y])).unwrap(),
                            };
                        }
                    };
                }
            }
        });
        draw(ctx, &bullet_batch, DrawParam::default()).unwrap();
    }
}
