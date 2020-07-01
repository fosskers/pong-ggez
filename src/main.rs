use ggez::conf::WindowMode;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, Rect};
use ggez::input::keyboard::{self, KeyCode};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameError, GameResult};
use rand::prelude::ThreadRng;
use rand::Rng;

type Vector = ggez::mint::Vector2<f32>;

const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH: f32 = 600.0;

const X_OFFSET: f32 = 20.0; // distance from each paddle to their respective walls
const PADDLE_WIDTH: f32 = 12.0;
const PADDLE_HEIGHT: f32 = 75.0;
const PADDLE_SPEED: f32 = 8.0;

const BALL_RADIUS: f32 = 10.0;
const MIN_VEL: f32 = 3.0;
const MAX_VEL: f32 = 5.0;
const TRAIL_RATE: u32 = 7;

/// A piece of a short trail that follows the `Ball`.
struct Trail {
    rect: Rect,
}

impl Trail {
    /// A new `Trail` piece at the centre of the screen.
    fn new() -> Self {
        let x = SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0;
        let y = SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0;

        Trail {
            rect: Rect::new(x, y, BALL_RADIUS / 4.0, BALL_RADIUS / 4.0),
        }
    }

    fn move_to(&mut self, rect: &Rect) {
        self.rect.move_to(Point2 {
            x: rect.x,
            y: rect.y,
        })
    }
}

struct Ball {
    rect: Rect,
    vel: Vector,
}

impl Ball {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x_vel = Ball::rand_velocity(&mut rng);
        let y_vel = Ball::rand_velocity(&mut rng);
        let x = SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0;
        let y = SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0;

        Ball {
            rect: Rect::new(x, y, BALL_RADIUS, BALL_RADIUS),
            vel: Vector { x: x_vel, y: y_vel },
        }
    }

    fn rand_velocity(rng: &mut ThreadRng) -> f32 {
        if rng.gen::<bool>() {
            rng.gen_range(MIN_VEL, MAX_VEL) * -1.0
        } else {
            rng.gen_range(MIN_VEL, MAX_VEL)
        }
    }
}

struct State {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    trail_0: Trail,
    trail_1: Trail,
    trail_2: Trail,
    trail_3: Trail,
    l_score: u32,
    r_score: u32,
    score_text: (graphics::Text, graphics::DrawParam),
    fps: u32,
    fps_text: (graphics::Text, graphics::DrawParam),
    frame: u32,
}

impl State {
    fn new(ctx: &mut Context) -> Self {
        State {
            l_paddle: Rect::new(
                X_OFFSET,
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            r_paddle: Rect::new(
                SCREEN_WIDTH - (X_OFFSET + PADDLE_WIDTH),
                SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            ball: Ball::new(),
            trail_0: Trail::new(),
            trail_1: Trail::new(),
            trail_2: Trail::new(),
            trail_3: Trail::new(),
            l_score: 0,
            r_score: 0,
            score_text: State::new_score(ctx, 0, 0),
            fps: 0,
            fps_text: State::new_fps(ctx),
            frame: 0,
        }
    }

    fn new_score(
        ctx: &mut Context,
        l_score: u32,
        r_score: u32,
    ) -> (graphics::Text, graphics::DrawParam) {
        let mut scoreboard_text = graphics::Text::new(format!("{} \t {}", l_score, r_score));
        scoreboard_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let score_coords = Point2 {
            x: SCREEN_WIDTH / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            y: 10.0,
        };
        let score_params = graphics::DrawParam::default().dest(score_coords);

        (scoreboard_text, score_params)
    }

    fn new_fps(ctx: &mut Context) -> (graphics::Text, graphics::DrawParam) {
        let mut fps = graphics::Text::new(format!("{:.0}", ggez::timer::fps(ctx)));
        fps.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));
        let fps_coords = Point2 {
            x: SCREEN_WIDTH / 2.0 - fps.width(ctx) as f32 / 2.0,
            y: SCREEN_HEIGHT - 24.0,
        };
        let fps_params = graphics::DrawParam::default().dest(fps_coords);

        (fps, fps_params)
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Bump the frame count.
        self.frame += 1;

        // Update the `Trail` pieces, if necessary.
        if self.frame % TRAIL_RATE == 0 {
            self.trail_3.move_to(&self.trail_2.rect);
            self.trail_2.move_to(&self.trail_1.rect);
            self.trail_1.move_to(&self.trail_0.rect);
            self.trail_0.move_to(&self.ball.rect);
        }

        // Move the paddles.
        if keyboard::is_key_pressed(ctx, KeyCode::W) && self.l_paddle.top() > 0.0 {
            self.l_paddle.y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::R) && self.l_paddle.bottom() < SCREEN_HEIGHT {
            self.l_paddle.y += PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Up) && self.r_paddle.top() > 0.0 {
            self.r_paddle.y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) && self.r_paddle.bottom() < SCREEN_HEIGHT {
            self.r_paddle.y += PADDLE_SPEED;
        }

        // Move the ball.
        if (self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.l_paddle))
            || (self.ball.vel.x > 0.0 && self.ball.rect.overlaps(&self.r_paddle))
        {
            self.ball.vel.x *= -1.0;
        }
        if (self.ball.vel.y < 0.0 && self.ball.rect.top() < 0.0)
            || (self.ball.vel.y > 0.0 && self.ball.rect.bottom() > SCREEN_HEIGHT)
        {
            self.ball.vel.y *= -1.0;
        }
        self.ball.rect.translate(self.ball.vel);

        // Check for a goal.
        if self.ball.rect.left() < 0.0 {
            self.r_score += 1;
            self.ball = Ball::new();
            self.score_text = State::new_score(ctx, self.l_score, self.r_score);
        } else if self.ball.rect.right() > SCREEN_WIDTH {
            self.l_score += 1;
            self.ball = Ball::new();
            self.score_text = State::new_score(ctx, self.l_score, self.r_score);
        }

        // FPS updates.
        let fps = ggez::timer::fps(ctx) as u32;
        if fps != self.fps {
            self.fps = fps;
            self.fps_text = State::new_fps(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        let trail_1_mesh = rect_mesh(ctx, &self.trail_1.rect)?;
        let trail_2_mesh = rect_mesh(ctx, &self.trail_2.rect)?;
        let trail_3_mesh = rect_mesh(ctx, &self.trail_3.rect)?;

        let ball_mesh = rect_mesh(ctx, &self.ball.rect)?;

        let l_paddle_mesh = rect_mesh(ctx, &self.l_paddle)?;
        let r_paddle_mesh = rect_mesh(ctx, &self.r_paddle)?;

        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &trail_1_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &trail_2_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &trail_3_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &l_paddle_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &r_paddle_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &self.score_text.0, self.score_text.1)?;
        graphics::draw(ctx, &self.fps_text.0, self.fps_text.1)?;
        graphics::present(ctx) // Handle error better?
    }
}

fn rect_mesh(ctx: &mut Context, rect: &Rect) -> Result<graphics::Mesh, GameError> {
    graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        *rect,
        Color::new(1.0, 1.0, 1.0, 1.0),
    )
}

fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Pong", "Colin Woodbury")
        .window_mode(WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let mut state = State::new(&mut ctx);
    ggez::event::run(&mut ctx, &mut event_loop, &mut state)
}
