use ggez::Context;
use ggez::graphics;
use ggez::graphics::Point2;

use camera::Camera;

pub struct Entity {
    x: i64,
    y: i64,

    //Probably some textures and maybe some interaction thing
}

impl Entity {
    pub fn new(x: i64, y: i64) -> Self {
        Entity {
            x,
            y,
        }
    }

    pub fn draw(&self, ctx: &mut Context, camera: &Camera) {
        let (mut x, mut y) = (self.x as f32, self.y as f32);
        x += -camera.x as f32 + 0.5;
        y += -camera.y as f32 + 0.5;

        x *= 16.0;
        y *= 16.0;

        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            Point2::new(x, y),
            8.0,
            1.0,
        ).unwrap();
    }
}
