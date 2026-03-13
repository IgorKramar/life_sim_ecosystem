mod agent;
mod render;
mod world;

use macroquad::prelude::*;
use world::World;

const INIT_POPULATION: usize = 100;

#[macroquad::main("Life Sim: Ecosystem")]
async fn main() {
    let mut world = World::new(110, 70);
    world.spawn_initial_agents(INIT_POPULATION);

    let mut paused = false;
    let mut tick_interval = 0.05f32;
    let mut accumulator = 0.0f32;

    loop {
        accumulator += get_frame_time();

        // Управление
        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            world = World::new(110, 70);
            world.spawn_initial_agents(INIT_POPULATION);
        }
        if is_key_pressed(KeyCode::Equal) {
            tick_interval = (tick_interval * 0.75).max(0.01f32);
        }
        if is_key_pressed(KeyCode::Minus) {
            tick_interval = (tick_interval * 1.35).min(0.5f32);
        }

        // Логика
        if !paused && accumulator >= tick_interval {
            world.tick();
            accumulator -= tick_interval;
        }

        // Рендер
        render::render_world(&world);

        // Статистика
        let stats = format!(
            "Tick: {:4} | Agents: {:4} | Plants: {:4} | Avg Energy: {:.1} | Speed: {:.0}ms",
            world.tick_count,
            world.agents.len(),
            world.plants.len(),
            world.avg_energy(),
            tick_interval * 1000.0
        );
        draw_text(&stats, 20.0, 35.0, 20.0, WHITE);

        if paused {
            draw_text(
                "PAUSED — Press Space to resume",
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0,
                30.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}
