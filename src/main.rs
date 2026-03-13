mod agent;
mod config;
mod render;
mod world;

use config::Config;
use macroquad::prelude::*;
use world::World;

#[macroquad::main("Life Sim: Ecosystem v0.0.3")]
async fn main() {
    let config = Config::load();

    let mut world = World::new(&config);
    world.spawn_initial_agents();

    let mut paused = false;
    let mut tick_interval = config.timing.default_tick_interval;
    let mut accumulator = 0.0f32;
    let mut show_genealogy = false;

    loop {
        accumulator += get_frame_time();

        if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            world = World::new(&config);
            world.spawn_initial_agents();
        }
        if is_key_pressed(KeyCode::G) {
            show_genealogy = !show_genealogy;
        }
        if is_key_pressed(KeyCode::Equal) {
            tick_interval = (tick_interval * config.timing.speed_up_factor)
                .max(config.timing.min_tick_interval);
        }
        if is_key_pressed(KeyCode::Minus) {
            tick_interval = (tick_interval * config.timing.slow_down_factor)
                .min(config.timing.max_tick_interval);
        }

        if !paused && accumulator >= tick_interval {
            world.tick();
            accumulator -= tick_interval;
        }

        render::render_world(&world, &config);

        // Статистика
        let gen_stats = world.generation_stats();
        let max_gen = gen_stats.keys().max().unwrap_or(&0);

        let stats = format!(
            "Tick: {:4} | Agents: {:4} | Plants: {:4} | Avg Energy: {:.1} | Gen: {}",
            world.tick_count,
            world.agents.len(),
            world.plants.len(),
            world.avg_energy(),
            max_gen,
        );
        draw_text(&stats, 20.0, 35.0, 20.0, config::COLOR_TEXT);

        // Статистика по поколениям
        if show_genealogy {
            let mut y = 60.0;
            draw_text("Generations:", 20.0, y, 16.0, config::COLOR_TEXT);
            y += 20.0;
            for (gen, count) in gen_stats.iter() {
                let line = format!("  Gen {}: {}", gen, count);
                draw_text(&line, 20.0, y, 14.0, config::COLOR_TEXT);
                y += 18.0;
            }
        }

        if paused {
            draw_text(
                "PAUSED — Press Space to resume",
                screen_width() / 2.0 - 150.0,
                screen_height() / 2.0,
                30.0,
                config::COLOR_PAUSED,
            );
        }

        // Подсказка по управлению
        draw_text(
            "G: Toggle genealogy stats",
            20.0,
            screen_height() - 30.0,
            14.0,
            WHITE,
        );

        next_frame().await;
    }
}
