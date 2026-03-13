use crate::config::Config;
use rand::RngExt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AgentType {
    Herbivore,
    Predator,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Sex {
    Male,
    Female,
}

pub type Position = (i32, i32);

// ✅ Генеалогическая информация
#[derive(Clone, Debug, Default)]
pub struct Genealogy {
    /// ID родителя 1 (мать)
    pub parent1_id: Option<u64>,
    /// ID родителя 2 (отец)
    pub parent2_id: Option<u64>,
    /// Поколение (0 для начальной популяции)
    pub generation: u32,
    /// Уникальный семейный корень (для быстрого сравнения)
    pub family_root: u64,
}

#[derive(Clone, Debug)]
pub struct Agent {
    #[allow(dead_code)]
    pub id: u64,
    pub pos: Position,
    pub energy: i32,
    pub sex: Sex,
    pub age: u32,
    pub agent_type: AgentType,
    pub is_eaten: bool,
    pub genealogy: Genealogy, // ✅ Добавлено
}

impl Agent {
    pub fn new(id: u64, pos: Position, sex: Sex, agent_type: AgentType, config: &Config) -> Self {
        let initial_energy = match agent_type {
            AgentType::Herbivore => config.herbivore.initial_energy,
            AgentType::Predator => config.predator.initial_energy,
        };
        Self {
            id,
            pos,
            energy: initial_energy,
            sex,
            age: 0,
            agent_type,
            is_eaten: false,
            genealogy: Genealogy::default(), // Начальная популяция без родителей
        }
    }

    /// Создаёт потомка с правильной генеалогией
    pub fn create_offspring(
        &self,
        partner: &Agent,
        child_id: u64,
        child_pos: Position,
        child_sex: Sex,
        config: &Config,
    ) -> Self {
        let child_energy = match self.agent_type {
            AgentType::Herbivore => config.herbivore.birth_energy,
            AgentType::Predator => config.predator.birth_energy,
        };

        // Определяем семейный корень (берём от старшего поколения)
        let family_root = if self.genealogy.generation >= partner.genealogy.generation {
            self.genealogy.family_root
        } else {
            partner.genealogy.family_root
        };

        Self {
            id: child_id,
            pos: child_pos,
            energy: child_energy,
            sex: child_sex,
            age: 0,
            agent_type: self.agent_type,
            is_eaten: false,
            genealogy: Genealogy {
                parent1_id: Some(self.id),
                parent2_id: Some(partner.id),
                generation: (self.genealogy.generation.max(partner.genealogy.generation) + 1),
                family_root,
            },
        }
    }

    pub fn move_randomly(&mut self, bounds: (i32, i32), config: &Config) {
        let mut rng = rand::rng();
        let move_cost = match self.agent_type {
            AgentType::Herbivore => config.herbivore.move_cost,
            AgentType::Predator => config.predator.move_cost,
        };
        self.pos.0 = (self.pos.0 + rng.random_range(-1..=1)).clamp(0, bounds.0 - 1);
        self.pos.1 = (self.pos.1 + rng.random_range(-1..=1)).clamp(0, bounds.1 - 1);
        self.energy -= move_cost;
    }

    pub fn is_dead(&self, config: &Config) -> bool {
        let max_age = match self.agent_type {
            AgentType::Herbivore => config.herbivore.max_age,
            AgentType::Predator => config.predator.max_age,
        };
        self.energy <= 0 || self.age > max_age || self.is_eaten
    }

    pub fn manhattan_distance(&self, other: Position) -> u32 {
        ((self.pos.0 - other.0).abs() + (self.pos.1 - other.1).abs()) as u32
    }

    pub fn can_reproduce(&self, config: &Config) -> bool {
        let min_energy = match self.agent_type {
            AgentType::Predator => config.predator.reproduce_min_energy,
            AgentType::Herbivore => config.herbivore.reproduce_min_energy,
        };
        !self.is_dead(config) && self.energy >= min_energy
    }

    /// Проверка: является ли другой агент близким родственником
    pub fn is_close_relative(&self, other: &Agent, config: &Config) -> bool {
        if !config.genetics.enable_inbreeding_prevention {
            return false;
        }

        // Одинаковый семейный корень — потенциальные родственники
        if self.genealogy.family_root != other.genealogy.family_root {
            return false;
        }

        // Проверка: родители
        if config.genetics.prevent_parent_child {
            if let Some(p1) = self.genealogy.parent1_id {
                if p1 == other.id {
                    return true;
                }
            }
            if let Some(p2) = self.genealogy.parent2_id {
                if p2 == other.id {
                    return true;
                }
            }
            if let Some(p1) = other.genealogy.parent1_id {
                if p1 == self.id {
                    return true;
                }
            }
            if let Some(p2) = other.genealogy.parent2_id {
                if p2 == self.id {
                    return true;
                }
            }
        }

        // Проверка: братья/сёстры (общие родители)
        if config.genetics.prevent_siblings {
            // ✅ Исправлено: убран .into_iter(), Option уже IntoIterator
            let my_parents: Vec<u64> = self
                .genealogy
                .parent1_id
                .iter()
                .chain(self.genealogy.parent2_id.iter())
                .copied()
                .collect();
            let other_parents: Vec<u64> = other
                .genealogy
                .parent1_id
                .iter()
                .chain(other.genealogy.parent2_id.iter())
                .copied()
                .collect();

            // Если есть хотя бы один общий родитель — siblings
            // ✅ Исправлено: len() > 0 → !is_empty()
            if !my_parents.is_empty() && !other_parents.is_empty() {
                for mp in &my_parents {
                    for op in &other_parents {
                        if mp == op {
                            return true;
                        }
                    }
                }
            }
        }

        // Проверка: внуки/бабушки/дедушки
        if config.genetics.prevent_grandparent {
            // Упрощённая проверка по поколению
            let gen_diff =
                (self.genealogy.generation as i32 - other.genealogy.generation as i32).abs();
            if gen_diff >= 2 {
                // Может быть grandparent-grandchild, нужна deeper проверка
                // Для простоты: если разница >= 2 и same family_root — блокируем
                return true;
            }
        }

        false
    }
}
