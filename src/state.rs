use ggez::glam::*;
use ggez::{Context, GameResult};
use ggez::{event, graphics};
use rand::prelude::*;

use crate::boids::{Boid, update_boids};
use crate::colour::get_colour_by_width;

#[derive(Debug)]
pub struct MainState {
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub max_speed: f32,
    pub min_speed: f32,
    pub protected_range: f32,
    pub visible_range: f32,
    pub avoid_factor: f32,
    pub matching_factor: f32,
    pub centering_factor: f32,
    pub turn_factor: f32,
    pub cell_size: f32,
    pub draw_radius: f32,
    pub boids: Vec<Boid>,
}

impl MainState {
    pub fn new(
        boid_count: usize,
        width: f32,
        height: f32,
        margin: f32,
        max_speed: f32,
        min_speed: f32,
        protected_range: f32,
        visible_range: f32,
        avoid_factor: f32,
        matching_factor: f32,
        centering_factor: f32,
        turn_factor: f32,
        cell_size: f32,
        draw_radius: f32,
    ) -> GameResult<Self> {
        let mut rng = rand::rng();
        Ok(MainState {
            width,
            height,
            margin,
            max_speed,
            min_speed,
            protected_range,
            visible_range,
            avoid_factor,
            matching_factor,
            centering_factor,
            turn_factor,
            cell_size,
            draw_radius,
            boids: (0..boid_count)
                .map(|id| {
                    let x = rng.random_range(0..(width - margin).round() as u32) as f32;
                    Boid::new(
                        id,
                        Vec2::new(
                            x,
                            rng.random_range(0..(height - margin).round() as u32) as f32,
                        ),
                        Vec2::new(
                            rng.random_range(-max_speed / 2.0..max_speed / 2.0),
                            rng.random_range(-max_speed / 2.0..max_speed / 2.0),
                        ),
                        0.0,
                        get_colour_by_width(x, width.round() as u32),
                    )
                })
                .collect(),
        })
    }
}

const BLACK: graphics::Color = graphics::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        update_boids(self);
        let (width, height) = ctx.gfx.drawable_size();
        self.width = width;
        self.height = height;
        // Add error handling?
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, BLACK);
        for boid in &self.boids {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                Vec2::new(0.0, 0.0),
                self.draw_radius,
                1.0,
                boid.colour,
            )?;
            canvas.draw(&circle, boid.pos);
        }
        canvas.finish(ctx)?;
        Ok(())
    }
}
