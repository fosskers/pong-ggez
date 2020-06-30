use ggez::event::EventHandler;
use ggez::{Context, ContextBuilder, GameResult};

struct State {}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Pong", "Colin Woodbury").build()?;
    let mut state = State {};
    ggez::event::run(&mut ctx, &mut event_loop, &mut state)
}
