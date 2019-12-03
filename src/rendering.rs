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
    Sprite(String),
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
        let mut bullet_batch = SpriteBatch::new(self.sprites.get("bullet").unwrap().clone());
        (&positions, &visuals).join().for_each(|(pos, vis)| {
            let x = pos.0.x as f32;
            let y = pos.0.y as f32;
            match vis {
                Circle(color, radius) => {}
                Sprite(img) => {
                    match img.as_ref() {
                        "bullet" => {
                            bullet_batch.add(DrawParam::default().dest([x, y]));
                        }
                        _ => {
                            let sprite = self.sprites.get(img).unwrap();
                            draw(ctx, sprite, DrawParam::default().dest([x, y]));
                            println!("player");
                        }
                    };
                }
            }
        });
        draw(ctx, &bullet_batch, DrawParam::default());
    }
}
