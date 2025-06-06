use std::collections::HashMap;

use ggez::glam::*;
use ggez::graphics::Color;
use rand::prelude::*;
use rayon::prelude::*;

use crate::state::MainState;

#[derive(Debug, Clone, PartialEq)]
pub struct Boid {
    id: usize,
    pub pos: Vec2,
    vel: Vec2,
    current_speed: f32,
    pub colour: Color,
}

impl Boid {
    pub fn new(id: usize, pos: Vec2, vel: Vec2, current_speed: f32, colour: Color) -> Self {
        Boid {
            id,
            pos,
            vel,
            current_speed,
            colour,
        }
    }
}

pub fn populate_grid(boids: &[Boid], cell_size: f32) -> HashMap<(u32, u32), Vec<usize>> {
    let mut grid: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
    for (index, boid) in boids.iter().enumerate() {
        let cell_x: u32 = (boid.pos.x / cell_size).floor() as u32;
        let cell_y: u32 = (boid.pos.y / cell_size).floor() as u32;
        grid.entry((cell_x, cell_y)).or_default().push(index);
    }
    grid
}

pub fn update_boids(state: &mut MainState) {
    let protected_range_squared = state.protected_range * state.protected_range;
    let visible_range_squared = state.visible_range * state.visible_range;
    let grid = populate_grid(&state.boids, state.cell_size);

    let new_boid_states: Vec<(Vec2, Vec2, f32)> = state
        .boids
        .par_iter()
        .enumerate()
        .map(|(boid_idx, boid)| {
            let mut rng = rand::rng();
            let mut pos_avg = Vec2::ZERO;
            let mut vel_avg = Vec2::ZERO;
            let mut close_offset = Vec2::ZERO;

            let mut neighboring_boids: usize = 0;

            let boid_cell_x: i32 = (boid.pos.x / state.cell_size).floor() as i32;
            let boid_cell_y: i32 = (boid.pos.y / state.cell_size).floor() as i32;
            for x_offset in -1..=1 {
                for y_offset in -1..=1 {
                    let new_x = boid_cell_x + x_offset;
                    let new_y = boid_cell_y + y_offset;
                    if new_x >= 0 && new_y >= 0 {
                        let key = (new_x as u32, new_y as u32);
                        if let Some(near_boids) = grid.get(&key) {
                            for otherboid_idx in near_boids {
                                if *otherboid_idx == boid_idx {
                                    continue;
                                }
                                let otherboid = &state.boids[*otherboid_idx];

                                let offset = boid.pos - otherboid.pos;
                                // Only consider those within our visible box
                                if offset.x.abs() < state.visible_range
                                    && offset.y.abs() < state.visible_range
                                {
                                    let dist_sq = offset.length_squared();
                                    if dist_sq < protected_range_squared {
                                        close_offset += offset;
                                    } else if dist_sq < visible_range_squared {
                                        pos_avg += otherboid.pos;
                                        vel_avg += otherboid.vel;
                                        neighboring_boids += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            let mut next_vel = boid.vel;
            if neighboring_boids > 0 {
                let n = neighboring_boids as f32;
                pos_avg /= n;
                vel_avg /= n;
                next_vel += (pos_avg - boid.pos) * state.centering_factor
                    + (vel_avg - boid.vel) * state.matching_factor;
            }
            next_vel += close_offset * state.avoid_factor;

            // Turn if approaching the edge of the screen
            if boid.pos.y > (state.height - state.margin) as f32 {
                next_vel.y -= state.turn_factor;
            }
            if boid.pos.x > (state.width - state.margin) as f32 {
                next_vel.x -= state.turn_factor;
            }
            if boid.pos.x < state.margin as f32 {
                next_vel.x += state.turn_factor;
            }
            if boid.pos.y < state.margin as f32 {
                next_vel.y += state.turn_factor;
            }

            // Make sure we're within speed limits
            let mut speed = next_vel.length();
            if speed > 0.0 {
                if speed < state.min_speed {
                    next_vel = next_vel.normalize() * state.min_speed;
                    speed = state.min_speed;
                } else if speed > state.max_speed {
                    next_vel = next_vel.normalize() * state.max_speed;
                    speed = state.max_speed;
                }
            } else if state.min_speed > 0.0 {
                // Give it a nudge if stopped
                next_vel = Vec2::new(
                    rng.random_range(-state.min_speed..state.min_speed),
                    rng.random_range(-state.min_speed..state.min_speed),
                );
                speed = state.min_speed;
            }

            let mut next_pos = boid.pos + next_vel;

            // Finally, clamp them so they're in the screen
            if next_pos.x < 0.0 {
                next_pos.x = 0.0;
            } else if next_pos.x >= state.width {
                next_pos.x = state.width - 1.0;
            }
            if next_pos.y < 0.0 {
                next_pos.y = 0.0;
            } else if next_pos.y >= state.height {
                next_pos.y = state.height - 1.0;
            }

            (next_pos, next_vel, speed)
        })
        .collect();

    // apply the changes
    for (i, boid) in state.boids.iter_mut().enumerate() {
        let (new_pos, new_vel, new_speed) = new_boid_states[i];
        boid.pos = new_pos;
        boid.vel = new_vel;
        boid.current_speed = new_speed;
    }
}
