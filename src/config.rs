//! Конфигурация симуляции экосистемы
//!
//! Загружается из config.toml при старте.
//! Если файл не найден — создаётся дефолтный.

use macroquad::color::Color;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// ============================================================================
// 📄 СТРУКТУРА КОНФИГА
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub world: WorldConfig,
    pub population: PopulationConfig,
    pub plants: PlantsConfig,
    pub herbivore: HerbivoreConfig,
    pub predator: PredatorConfig,
    pub common: CommonConfig,
    pub timing: TimingConfig,
    pub render: RenderConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationConfig {
    pub init_population: usize,
    pub herbivore_spawn_ratio: f64,
    pub max_density_per_type: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantsConfig {
    pub growth_attempts: usize,
    pub growth_min: u32,
    pub growth_max: u32,
    pub max_energy: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HerbivoreConfig {
    pub initial_energy: i32,
    pub birth_energy: i32,
    pub reproduce_min_energy: i32,
    pub max_age: u32,
    pub move_cost: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredatorConfig {
    pub initial_energy: i32,
    pub birth_energy: i32,
    pub reproduce_min_energy: i32,
    pub max_age: u32,
    pub move_cost: i32,
    pub kill_reward: i32,
    pub hunt_fail_penalty: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonConfig {
    pub migration_cost: i32,
    pub migration_range: i32,
    pub mate_search_radius: u32,
    pub min_energy_after_reproduce: i32,
    pub reproduction_cost: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingConfig {
    pub default_tick_interval: f32,
    pub min_tick_interval: f32,
    pub max_tick_interval: f32,
    pub speed_up_factor: f32,
    pub slow_down_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderConfig {
    pub cell_size: f32,
    pub agent_size_min: f32,
    pub agent_size_max: f32,
    pub agent_size_divisor: f32,
    pub plant_alpha_divisor: f32,
}

// ============================================================================
// 🔄 ЗАГРУЗКА КОНФИГА
// ============================================================================

const CONFIG_PATH: &str = "config.toml";

impl Config {
    /// Загружает конфиг из файла или создаёт дефолтный
    pub fn load() -> Self {
        if Path::new(CONFIG_PATH).exists() {
            match fs::read_to_string(CONFIG_PATH) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => {
                        println!("✅ Конфиг загружен из {}", CONFIG_PATH);
                        config
                    }
                    Err(e) => {
                        eprintln!("⚠️ Ошибка парсинга {}: {}", CONFIG_PATH, e);
                        eprintln!("📝 Используем дефолтный конфиг");
                        Self::default()
                    }
                },
                Err(e) => {
                    eprintln!("⚠️ Ошибка чтения {}: {}", CONFIG_PATH, e);
                    eprintln!("📝 Используем дефолтный конфиг");
                    Self::default()
                }
            }
        } else {
            let config = Self::default();
            config.save();
            println!("📝 Создан дефолтный {}", CONFIG_PATH);
            config
        }
    }

    /// Сохраняет конфиг в файл
    pub fn save(&self) {
        match toml::to_string_pretty(self) {
            Ok(content) => {
                if let Err(e) = fs::write(CONFIG_PATH, content) {
                    eprintln!("⚠️ Ошибка записи {}: {}", CONFIG_PATH, e);
                }
            }
            Err(e) => {
                eprintln!("⚠️ Ошибка сериализации конфига: {}", e);
            }
        }
    }

    /// Возвращает дефолтный конфиг
    pub fn default() -> Self {
        Self {
            world: WorldConfig {
                width: 110,
                height: 110,
            },
            population: PopulationConfig {
                init_population: 200,
                herbivore_spawn_ratio: 0.5,
                max_density_per_type: 3,
            },
            plants: PlantsConfig {
                growth_attempts: 50,
                growth_min: 20,
                growth_max: 40,
                max_energy: 100,
            },
            herbivore: HerbivoreConfig {
                initial_energy: 120,
                birth_energy: 100,
                reproduce_min_energy: 70,
                max_age: 200,
                move_cost: 1,
            },
            predator: PredatorConfig {
                initial_energy: 150,
                birth_energy: 120,
                reproduce_min_energy: 120,
                max_age: 250,
                move_cost: 1,
                kill_reward: 250,
                hunt_fail_penalty: 2,
            },
            common: CommonConfig {
                migration_cost: 1,
                migration_range: 2,
                mate_search_radius: 6,
                min_energy_after_reproduce: 40,
                reproduction_cost: 40,
            },
            timing: TimingConfig {
                default_tick_interval: 0.05,
                min_tick_interval: 0.01,
                max_tick_interval: 0.5,
                speed_up_factor: 0.75,
                slow_down_factor: 1.35,
            },
            render: RenderConfig {
                cell_size: 10.0,
                agent_size_min: 0.3,
                agent_size_max: 1.0,
                agent_size_divisor: 100.0,
                plant_alpha_divisor: 50.0,
            },
        }
    }
}

// ============================================================================
// 🎨 ЦВЕТА (остались константами, т.к. toml не поддерживает Color)
// ============================================================================

pub const COLOR_HERBIVORE_MALE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
pub const COLOR_HERBIVORE_FEMALE: Color = Color::new(0.53, 0.81, 0.98, 1.0);
pub const COLOR_PREDATOR_MALE: Color = Color::new(1.0, 0.0, 0.0, 1.0);
pub const COLOR_PREDATOR_FEMALE: Color = Color::new(0.5, 0.0, 0.0, 1.0);
pub const COLOR_PLANT: Color = Color::new(0.0, 0.8, 0.0, 1.0);
pub const COLOR_TEXT: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const COLOR_PAUSED: Color = Color::new(1.0, 1.0, 0.0, 1.0);
pub const COLOR_AGENT_OUTLINE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const COLOR_AGENT_OUTLINE_WIDTH: f32 = 1.0;
