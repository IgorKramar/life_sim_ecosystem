use crate::agent::{AgentType, Sex};
use crate::config::{Config, COLOR_HERBIVORE_FEMALE, COLOR_HERBIVORE_MALE, COLOR_PLANT};
use crate::config::{
    COLOR_AGENT_OUTLINE, COLOR_AGENT_OUTLINE_WIDTH, COLOR_PREDATOR_FEMALE, COLOR_PREDATOR_MALE,
};
use crate::world::World;
use macroquad::prelude::*;

pub fn render_world(world: &World, config: &Config) {
    clear_background(BLACK);

    for (pos, &amount) in &world.plants {
        let alpha = (amount as f32 / config.render.plant_alpha_divisor).min(1.0);
        draw_rectangle(
            pos.0 as f32 * config.render.cell_size,
            pos.1 as f32 * config.render.cell_size,
            config.render.cell_size,
            config.render.cell_size,
            Color::new(COLOR_PLANT.r, COLOR_PLANT.g, COLOR_PLANT.b, alpha),
        );
    }

    let (agents, _) = world.get_render_data();
    for (pos, a_type, sex, energy) in agents {
        let color = match (a_type, sex) {
            (AgentType::Predator, Sex::Male) => COLOR_PREDATOR_MALE,
            (AgentType::Predator, Sex::Female) => COLOR_PREDATOR_FEMALE,
            (AgentType::Herbivore, Sex::Male) => COLOR_HERBIVORE_MALE,
            (AgentType::Herbivore, Sex::Female) => COLOR_HERBIVORE_FEMALE,
        };

        let radius = (energy as f32 / config.render.agent_size_divisor)
            .clamp(config.render.agent_size_min, config.render.agent_size_max)
            * (config.render.cell_size / 2.0);

        draw_circle(
            pos.0 as f32 * config.render.cell_size + config.render.cell_size / 2.0,
            pos.1 as f32 * config.render.cell_size + config.render.cell_size / 2.0,
            radius,
            color,
        );
        draw_circle_lines(
            pos.0 as f32 * config.render.cell_size + config.render.cell_size / 2.0,
            pos.1 as f32 * config.render.cell_size + config.render.cell_size / 2.0,
            radius,
            COLOR_AGENT_OUTLINE_WIDTH,
            COLOR_AGENT_OUTLINE,
        );
    }
}
