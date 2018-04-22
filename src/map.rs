use ggez::Context;
use ggez::graphics;
use ggez::graphics::{Image, Point2};
use entity::Entity;

use std::collections::HashMap;

use camera::Camera;

pub struct Map {
    textures: Vec<Image>,
    map: Vec<Vec<usize>>,
    entities: Vec<Entity>,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map {
        Map {
            textures: vec![
                Image::new(ctx, "/error.png").unwrap(),
                Image::new(ctx, "/grass0.png").unwrap(),
                Image::new(ctx, "/grass1.png").unwrap(),
                Image::new(ctx, "/grass2.png").unwrap(),
                Image::new(ctx, "/grass3.png").unwrap(),
                Image::new(ctx, "/grass1.png").unwrap(), //grass4 looks bad
            ],
            map: vec![
                vec![2,2,2,2,2,2,2],
                vec![3,1,3,5,4,3,1],
                vec![2,1,4,5,3,3,1],
                vec![1,3,1,5,2,3,1],
                vec![3,2,4,5,3,2,4],
                vec![1,4,2,5,1,3,1],
                vec![1,2,4,5,2,2,1],
            ],
            entities: vec![Entity::new(4,4)],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        match self.map.get(y) {
            Some(some) => some.get(x).cloned(),
            None => None,
        }
    }

    pub fn draw(&self, ctx: &mut Context, camera: &Camera) {
        //Draw background
        for x in 0..camera.width {
            for y in 0..camera.height {
                graphics::draw(
                    ctx,
                    &self.textures[self.get((x + camera.x) as usize, (y + camera.y) as usize).unwrap_or(0)],
                    Point2::new((x*16) as f32, (y*16) as f32),
                    0.0
                ).unwrap();
            }
        }

        //Draw entities
        for entity in &self.entities {
            entity.draw(ctx, &camera);
        }
    }
}
