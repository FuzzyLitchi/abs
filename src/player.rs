use ggez::Context;
use ggez::graphics;
use ggez::graphics::Image;
use ggez::graphics::{Point2, Vector2};

//Pixels per second
const SPEED: f32 = 500.0;
const SPEED_SQUARED: f32 = SPEED*SPEED;
//Pixels per second per second
const ACC: f32 = 1000.0;
//Percent of velocity kept per second
const FRICTION: f32 = 0.05;

pub struct Player {
    pos: Point2,
    velocity: Vector2,
    textures: Vec<Image>,
    pub movement: Movement,
}

impl Player {
    pub fn new(pos: Point2, textures: Vec<Image>) -> Self {
        Player {
            pos,
            velocity: Vector2::new(0.0, 0.0),
            textures,
            movement: Movement::new(),
        }
    }

    pub fn default(ctx: &mut Context) -> Self {
        let textures = vec![
            Image::new(ctx, "/player0.png").unwrap(),
            Image::new(ctx, "/player1.png").unwrap(),
            Image::new(ctx, "/player2.png").unwrap(),
        ];

        Player::new(Point2::new(0.0, 0.0), textures)
    }

    pub fn update(&mut self, dt: f32) {
        //Acceleration and stuff
        self.velocity += self.movement.vec()*ACC*dt;

        //Make sure we're not moving faster than we're allowed
        if self.velocity.length_squared() > SPEED_SQUARED {
            self.velocity = self.velocity * SPEED / self.velocity.length();
        }

        //Apply our speed
        self.pos += self.velocity*dt;

        //Apply friction
        self.velocity *= FRICTION.powf(dt);
    }

    pub fn draw(&self, ctx: &mut Context) {
        let texture;

        if self.velocity.x > 100.0 {
            texture = &self.textures[1]
        } else if self.velocity.x < -100.0 {
            texture = &self.textures[2]
        } else {
            texture = &self.textures[0]
        }

        graphics::draw(
            ctx,
            texture,
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
        let vec = Vector2::new(self.x(), self.y());
        if vec.length() == 0.0 {
            return vec;
        }
        vec / vec.length()
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

impl Length for Vector2 {
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x.powi(2)+self.y.powi(2)
    }
}
