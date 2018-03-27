extern crate ggez;
use ggez::*;
use ggez::event::{Keycode, Mod};

mod player;
use player::Player;

struct MainState {
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { player: Player::default(ctx) };
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

        self.player.draw(ctx);

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
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("shana", "FuzzyLitchi", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}