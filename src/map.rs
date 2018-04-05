use ggez::Context;
use ggez::graphics;
use ggez::graphics::{Image, Point2};

use camera::Camera;

pub struct Map {
    textures: Vec<Image>,
    map: Vec<Vec<usize>>,
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map {
        Map {
            textures: vec![
                Image::new(ctx, "/grass0.png").unwrap(),
                Image::new(ctx, "/grass1.png").unwrap(),
                Image::new(ctx, "/grass2.png").unwrap(),
                Image::new(ctx, "/grass3.png").unwrap(),
                Image::new(ctx, "/grass0.png").unwrap(), //grass4 looks bad
            ],
            map: vec![
                vec![1,1,1,1,1,1,1],
                vec![2,0,2,4,3,2,0],
                vec![1,0,3,4,2,2,0],
                vec![0,2,0,4,1,2,0],
                vec![2,1,3,4,2,1,3],
                vec![0,3,1,4,0,2,0],
                vec![0,1,3,4,2,1,0],
            ]
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        match self.map.get(y) {
            Some(some) => some.get(x).cloned(),
            None => None,
        }
    }

    pub fn draw(&self, ctx: &mut Context, camera: &Camera) {
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
    }
}
