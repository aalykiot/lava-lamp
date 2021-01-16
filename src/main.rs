use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use rand::prelude::*;

// dimensions
const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 600.0;

// colors
const BACKGROUND_COLOR: (u8, u8, u8) = (186, 220, 88);
const BUBBLE_COLOR: (u8, u8, u8) = (106, 176, 76);

struct GameState {
    bubbles: Vec<Bubble>,
}

impl event::EventHandler for GameState {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from(BACKGROUND_COLOR));
        for bubble in &self.bubbles {
            bubble.draw(ctx)?;
        }
        graphics::present(ctx)
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for bubble in &mut self.bubbles {
            bubble.update(ctx)?;
        }
        Ok(())
    }
}

struct Bubble {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
}

impl Bubble {
    fn rand() -> Bubble {
        let mut range = rand::thread_rng();
        let x: f32 = range.gen::<f32>() * SCREEN_WIDTH;
        let y: f32 = range.gen::<f32>() * SCREEN_HEIGHT;
        let radius: f32 = range.gen::<f32>() * 120.0;
        let speed: f32 = range.gen::<f32>();

        Bubble {
            x,
            y,
            radius,
            speed,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.x, self.y),
            self.radius,
            0.10,
            graphics::Color::from(BUBBLE_COLOR),
        )?;
        graphics::draw(ctx, &circle, graphics::DrawParam::default())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::delta(ctx);
        self.y = self.y - self.speed * dt.as_secs_f32() * 16.0;

        if self.y + self.radius < 0.0 {
            self.y = SCREEN_HEIGHT + self.radius;
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("lava_lamp", "aa_studios")
        .window_setup(conf::WindowSetup::default().title("Lava Lamp"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                .resizable(false),
        )
        .build()?;

    let mut bubbles = vec![];

    for _ in 0..15 {
        bubbles.push(Bubble::rand());
    }

    let state = &mut GameState { bubbles };
    event::run(ctx, event_loop, state)
}
