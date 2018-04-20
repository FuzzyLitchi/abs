extern crate ggez;
use ggez::*;
use ggez::event::{Keycode, Mod};
use ggez::graphics;

mod map;
use map::Map;

mod player;
use player::Player;

mod camera;
use camera::Camera;

mod entity;

struct MainState {
    map: Map,
    player: Player,
    camera: Camera,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            map: Map::new(ctx),
            player: Player::default(ctx),
            camera: Camera::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = timer::get_delta(ctx).subsec_nanos() as f32 / 1_000_000_000.0;

        self.player.update(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.map.draw(ctx, &self.camera);

        self.player.draw(ctx, &self.camera);

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        //Player movement code
        if keycode == Keycode::W {
            self.player.movement.enable_up();
        }
        if keycode == Keycode::A {
            self.player.movement.enable_left();
        }
        if keycode == Keycode::S {
            self.player.movement.enable_down();
        }
        if keycode == Keycode::D {
            self.player.movement.enable_right();
        }
    }
    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool
    ) {
        //Player movement code
        if keycode == Keycode::W {
            self.player.movement.disable_up();
        }
        if keycode == Keycode::A {
            self.player.movement.disable_left();
        }
        if keycode == Keycode::S {
            self.player.movement.disable_down();
        }
        if keycode == Keycode::D {
            self.player.movement.disable_right();
        }
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_mode.width = 512;
    c.window_mode.height = 512;

    let ctx = &mut Context::load_from_conf("shana", "FuzzyLitchi", c).unwrap();
    //Non blurry for pixel art
    graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
    //Scale
    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 112.0, 112.0)).expect("This should really work");

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
