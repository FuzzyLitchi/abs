use ggez::Context;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::Point2;

pub struct Player {
    pos: Point2,
    //width:  16px,
    //height: 16px,
    //textures: Vec<Image>, Later
    pub movement: Movement,
}

impl Player {
    pub fn new(pos: Point2) -> Self {
        Player {
            pos,
            movement: Movement::new(),
        }
    }

    pub fn default(_ctx: &mut Context) -> Self {
        Player::new(Point2::new(0.0, 0.0))
    }

    pub fn update(&mut self, dt: f32) {
        self.movement.update(dt);

        if self.movement.can_move {
            if self.movement.up {
                self.pos.y -= 1.0;
                self.movement.cant_move();
            } else if self.movement.left {
                self.pos.x -= 1.0;
                self.movement.cant_move();
            } else if self.movement.down {
                self.pos.y += 1.0;
                self.movement.cant_move();
            } else if self.movement.right {
                self.pos.x += 1.0;
                self.movement.cant_move();
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let mut point = self.pos.clone() * 16.0;
        point.x += 8.0;
        point.y += 8.0;

        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            point,
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
