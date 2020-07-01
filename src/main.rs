use ggez::conf::WindowMode;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, ContextBuilder, GameResult};

type Vector = ggez::mint::Vector2<f32>;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 600.0;

const X_OFFSET: f32 = 20.0; // distance from each paddle to their respective walls
const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 75.0;

const BALL_RADIUS: f32 = 10.0;

struct Ball {
    rect: Rect,
    vel: Vector,
}

struct State {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u32,
    r_score: u32,
}

impl State {
    fn new() -> State {
        State {
            l_paddle: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            r_paddle: Rect::new(
                SCREEN_WIDTH - X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            ball: Ball {
                rect: Rect::new(
                    SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                    SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                    BALL_RADIUS,
                    BALL_RADIUS,
                ),
                vel: Vector { x: 0.0, y: 0.0 },
            },
            l_score: 0,
            r_score: 0,
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.ball.rect,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let l_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.l_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        let r_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.r_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &l_paddle_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &r_paddle_mesh, graphics::DrawParam::default())?;
        graphics::present(ctx) // Handle error better?
    }
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Pong", "Colin Woodbury")
        .window_mode(WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let mut state = State::new();
    ggez::event::run(&mut ctx, &mut event_loop, &mut state)
}
