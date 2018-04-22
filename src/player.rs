use ggez::Context;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::Point2;

use camera::Camera;

pub struct Player {
    x: i64,
    y: i64,
    //width:  16px,
    //height: 16px,
    //textures: Vec<Image>, Later
    pub movement: Movement,
}

impl Player {
    pub fn new(x: i64, y: i64) -> Self {
        Player {
            x,
            y,
            movement: Movement::new(),
        }
    }

    pub fn default(_ctx: &mut Context) -> Self {
        Player::new(0, 0)
    }

    pub fn update(&mut self, dt: f32, camera: &mut Camera) {
        self.movement.update(dt);

        if self.movement.can_move {
            if self.movement.up {
                self.y -= 1;
                self.movement.cant_move();
                camera.update(self.x, self.y);
            } else if self.movement.left {
                self.x -= 1;
                self.movement.cant_move();
                camera.update(self.x, self.y);
            } else if self.movement.down {
                self.y += 1;
                self.movement.cant_move();
                camera.update(self.x, self.y);
            } else if self.movement.right {
                self.x += 1;
                self.movement.cant_move();
                camera.update(self.x, self.y);
            }
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

pub struct Movement {
    up:    bool,
    right: bool,
    down:  bool,
    left:  bool,

    time: f32,
    move_delay: f32,
    can_move: bool
}

impl Movement {
    fn new() -> Self {
        Movement {
            up: false,
            right: false,
            down: false,
            left: false,

            time: 0.0,
            move_delay: 0.2,
            can_move: true,
        }
    }

    fn update(&mut self, dt: f32) {
        self.time += dt;
        if self.time > self.move_delay {
            self.can_move = true;
        }
    }

    fn cant_move(&mut self) {
        self.time = 0.0;
        self.can_move = false;
    }

    pub fn enable_up(&mut self) {
        self.up = true;
    }

    pub fn disable_up(&mut self) {
        self.up = false;
    }

    pub fn enable_right(&mut self) {
        self.right = true;
    }

    pub fn disable_right(&mut self) {
        self.right = false;
    }

    pub fn enable_down(&mut self) {
        self.down = true;
    }

    pub fn disable_down(&mut self) {
        self.down = false;
    }

    pub fn enable_left(&mut self) {
        self.left = true;
    }

    pub fn disable_left(&mut self) {
        self.left = false;
    }
}
