use ggez::Context;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::{Point2, Vector2};

//Pixels per second
const SPEED: f32 = 100.0;
const SPEED_SQUARED: f32 = SPEED*SPEED;
//Pixels per second per second
const ACC: f32 = 1000.0;
//Percent of velocity kept per second
const FRICTION: f32 = 0.1;

pub struct Player {
    pos: Point2,
    velocity: Vector2,
    texture: Image,
    pub movement: Movement,
}

impl Player {
    pub fn new(pos: Point2, texture: Image) -> Self {
        Player {
            pos,
            velocity: Vector2::new(0.0, 0.0),
            texture,
            movement: Movement::new(),
        }
    }

    pub fn default(ctx: &mut Context) -> Self {
        let texture = Image::new(ctx, "/player.png").expect("Missing player.png");

        Player::new(Point2::new(0.0, 0.0), texture)
    }

    pub fn update(&mut self, dt: f32) {
        //Acceleration and stuff
        self.velocity += self.movement.vec()*ACC*dt;

        //Make sure we're not moving faster than we're allowed
        if self.velocity.length_squared() > SPEED_SQUARED {
            self.velocity = self.velocity * SPEED / self.velocity.length()
        }

        //Apply out spreed
        self.pos += self.velocity*dt;

        //Apply friction
        self.velocity *= FRICTION.powf(dt);
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(
            ctx,
            &self.texture,
            self.pos,
            0.0
        ).unwrap();
    }
}

pub struct Movement {
    up:    bool,
    right: bool,
    down:  bool,
    left:  bool,
}

impl Movement {
    fn new() -> Self {
        Movement {
            up: false,
            right: false,
            down: false,
            left: false,
        }
    }

    pub fn vec(&self) -> Vector2 {
        Vector2::new(self.x(), self.y())
    }

    pub fn x(&self) -> f32 {
        match (self.right, self.left) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
    }

    pub fn y(&self) -> f32 {
        match (self.down, self.up) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
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

trait Length {
    fn length(&self) -> f32;
    fn length_squared(&self) -> f32;
}

impl Length for Vector2{
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x.powi(2)+self.y.powi(2)
    }
}
