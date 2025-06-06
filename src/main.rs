use argh::FromArgs;
use ggez::GameResult;
use ggez::conf::{FullscreenType, WindowMode};
use ggez::event;

use rboid::state::MainState;

#[derive(Debug, FromArgs)]
#[argh(help_triggers("-h", "--help", "help"), description = "Boids simulator")]
struct Flags {
    #[argh(option, description = "boids to simulate", default = "5000")]
    boids: usize,
    #[argh(option, description = "max speed (float)", default = "3.0")]
    max_speed: f32,
    #[argh(option, description = "min speed (float)", default = "0.5")]
    min_speed: f32,
    #[argh(
        option,
        description = "visible range of boids (float)",
        default = "20.0"
    )]
    visible_range: f32,
}

pub fn main() -> GameResult {
    let args: Flags = argh::from_env();
    if args.max_speed < args.min_speed {
        return Err(ggez::GameError::ConfigError(String::from(
            "Max speed < min speed",
        )));
    }

    let window_mode = WindowMode::default().fullscreen_type(FullscreenType::Desktop);

    let cb = ggez::ContextBuilder::new("boids", "ggez").window_mode(window_mode);
    let (ctx, event_loop) = cb.build()?;
    let (width, height) = ctx.gfx.drawable_size();
    // go for a 5% of width margin
    let margin = width * 0.005;
    let state = MainState::new(
        args.boids,               // boid count
        width,                    // width
        height,                   // height
        margin,                   // margin
        args.max_speed,           // max_speed
        args.min_speed,           // min_speed
        2.0,                      // protected_range
        args.visible_range,       // visible_range
        0.10,                     // avoid_factor
        0.05,                     // matching_factor
        0.0005,                   // centering_factor
        0.2,                      // turn_factor
        args.visible_range * 1.1, // cell_size
        3.0,                      // draw_radius
    )?;
    event::run(ctx, event_loop, state)
}
