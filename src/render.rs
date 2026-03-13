use crate::agent::{AgentType, Sex};
use crate::world::World;
use macroquad::prelude::*;

// Настройки масштабирования (размер клетки на экране)
const CELL_SIZE: f32 = 10.0;

pub fn render_world(world: &World) {
    clear_background(BLACK);

    // 1. Отрисовка растений
    for (pos, &amount) in &world.plants {
        // Прозрачность растения зависит от его "плотности" (количества энергии в кусте)
        let alpha = (amount as f32 / 50.0).min(1.0);
        draw_rectangle(
            pos.0 as f32 * CELL_SIZE,
            pos.1 as f32 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            Color::new(0.0, 0.8, 0.0, alpha),
        );
    }

    // 2. Получение данных об агентах
    let (agents, _) = world.get_render_data();

    // 3. Отрисовка агентов
    for (pos, a_type, sex, energy) in agents {
        // Выбираем цвет в зависимости от типа и пола
        let color = match (a_type, sex) {
            (AgentType::Predator, Sex::Male) => RED,
            (AgentType::Predator, Sex::Female) => MAROON, // Темно-красный
            (AgentType::Herbivore, Sex::Male) => BLUE,
            (AgentType::Herbivore, Sex::Female) => SKYBLUE,
        };

        // Размер агента зависит от его энергии (минимум 0.3, максимум 1.0)
        let radius = (energy as f32 / 100.0).clamp(0.3, 1.0) * (CELL_SIZE / 2.0);

        // Рисуем тело агента
        draw_circle(
            pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            radius,
            color,
        );

        // Можно добавить обводку для лучшей видимости
        draw_circle_lines(
            pos.0 as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            pos.1 as f32 * CELL_SIZE + CELL_SIZE / 2.0,
            radius,
            1.0,
            WHITE,
        );
    }

    // 4. Текстовый интерфейс (Статистика)
    draw_text(
        &format!(
            "Ticks: {} | Agents: {} | Avg Energy: {:.1}",
            world.tick_count,
            world.agents.len(),
            world.avg_energy()
        ),
        10.0,
        20.0,
        20.0,
        WHITE,
    );
}
