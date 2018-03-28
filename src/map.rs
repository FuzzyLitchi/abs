use ggez::Context;
use ggez::graphics;
use ggez::graphics::{Image, Point2};

pub struct Map {
    textures: Vec<Image>,
    map: [[usize; 7]; 7]
}

impl Map {
    pub fn new(ctx: &mut Context) -> Map {
        Map {
            textures: vec![
                Image::new(ctx, "/grass0.png").unwrap(),
                Image::new(ctx, "/grass1.png").unwrap(),
                Image::new(ctx, "/grass2.png").unwrap(),
                Image::new(ctx, "/grass3.png").unwrap(),
                Image::new(ctx, "/grass0.png").unwrap(), //gress4 looks bad
            ],
            map: [
                [0,1,0,4,2,0,1],
                [2,0,2,4,3,2,0],
                [1,0,3,4,2,2,0],
                [0,2,0,4,1,2,0],
                [2,1,3,4,2,1,3],
                [0,3,1,4,0,2,0],
                [2,1,3,4,2,1,3],
            ]
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        for (y, rows) in self.map.iter().enumerate() {
            for (x, id) in rows.iter().enumerate() {
                graphics::draw(
                    ctx,
                    &self.textures[*id],
                    Point2::new((x*16) as f32, (y*16) as f32),
                    0.0
                ).unwrap();
            }
        }
    }
}
