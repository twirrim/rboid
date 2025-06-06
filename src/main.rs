use ggez::GameResult;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::event;

use rboid::state::MainState;

pub fn main() -> GameResult {
    let window_mode = WindowMode::default().fullscreen_type(FullscreenType::Desktop);

    let cb = ggez::ContextBuilder::new("boids", "ggez").window_mode(window_mode);
    let (ctx, event_loop) = cb.build()?;
    let (width, height) = ctx.gfx.drawable_size();
    // go for a 5% of width margin
    let margin = width * 0.005;
    let state = MainState::new(
        5000,   // boid count
        width,  // width
        height, // height
        margin, // margin
        10.0,   // max_speed
        2.5,    // min_speed
        2.0,    // protected_range
        20.0,   // visible_range
        0.10,   // avoid_factor
        0.05,   // matching_factor
        0.0005, // centering_factor
        0.2,    // turn_factor
        22.0,   // cell_size
        5.0,    // draw_radius
    )?;
    event::run(ctx, event_loop, state)
}
