use crate::agent::{Agent, AgentType, Position, Sex};
use rand::seq::IndexedRandom;
use rand::RngExt;
use std::collections::HashMap;

// ✅ Type aliases для читаемости сложных типов
type AgentRenderData = Vec<(Position, AgentType, Sex, i32)>;
type PlantsData = HashMap<Position, u32>;
type AgentsByPos = HashMap<Position, Vec<(usize, AgentType, bool, Sex)>>;

pub struct World {
    pub width: i32,
    pub height: i32,
    pub plants: PlantsData,
    pub agents: Vec<Agent>,
    pub tick_count: u64,
    last_id: u64,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            plants: HashMap::new(),
            agents: Vec::new(),
            tick_count: 0,
            last_id: 999,
        }
    }

    pub fn spawn_initial_agents(&mut self, count: usize) {
        let mut rng = rand::rng();
        for i in 0..count {
            let pos = (
                rng.random_range(0..self.width),
                rng.random_range(0..self.height),
            );
            let sex = if rng.random_bool(0.5) {
                Sex::Male
            } else {
                Sex::Female
            };
            let a_type = if rng.random_bool(0.75) {
                AgentType::Herbivore
            } else {
                AgentType::Predator
            };
            let mut agent = Agent::new(self.last_id + 1 + i as u64, pos, sex, a_type);

            let max_age = match agent.agent_type {
                AgentType::Herbivore => 200,
                AgentType::Predator => 250,
            };
            agent.age = rng.random_range(0..=(max_age / 4));
            self.agents.push(agent);
        }
        self.last_id += count as u64;
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        let mut rng = rand::rng();

        // 1. Рост растений
        for _ in 0..50 {
            let pos = (
                rng.random_range(0..self.width),
                rng.random_range(0..self.height),
            );
            *self.plants.entry(pos).or_insert(0) =
                (*self.plants.get(&pos).unwrap_or(&0) + rng.random_range(20..40)).min(100);
        }

        // 2. Движение и старение
        for agent in &mut self.agents {
            if !agent.is_dead() {
                agent.age += 1;
                agent.move_randomly((self.width, self.height));
            }
        }

        // 3. Питание — ДВУХФАЗНЫЙ ПОДХОД
        let mut agents_by_pos: AgentsByPos = HashMap::new();
        for (idx, agent) in self.agents.iter().enumerate() {
            if !agent.is_dead() {
                agents_by_pos.entry(agent.pos).or_default().push((
                    idx,
                    agent.agent_type,
                    agent.is_eaten,
                    agent.sex,
                ));
            }
        }

        // Фаза 3.2: Растения — ✅ ИСПРАВЛЕНО: pos нужен, поэтому не _pos
        let mut eaten_plants = Vec::new();
        for (pos, agents) in &agents_by_pos {
            if let Some(&plant_energy) = self.plants.get(pos) {
                if plant_energy > 0 {
                    if let Some((herb_idx, _, _eaten, _)) = agents
                        .iter()
                        .find(|(_, t, eaten, _)| *t == AgentType::Herbivore && !eaten)
                    {
                        let herb_idx = *herb_idx;
                        if !self.agents[herb_idx].is_dead() {
                            self.agents[herb_idx].energy += plant_energy as i32;
                            eaten_plants.push(*pos); // pos используется здесь
                        }
                    }
                }
            }
        }
        for pos in eaten_plants {
            self.plants.remove(&pos);
        }

        // Фаза 3.3: Охота хищников — ✅ ИСПРАВЛЕНО: позиция не нужна, используем .values()
        for agents in agents_by_pos.values() {
            let predator_indices: Vec<usize> = agents
                .iter()
                .filter(|(_, t, _, _)| *t == AgentType::Predator)
                .map(|(i, _, _, _)| *i)
                .collect();

            let mut prey_indices: Vec<usize> = agents
                .iter()
                .filter(|(_, t, eaten, _)| *t == AgentType::Herbivore && !eaten)
                .map(|(i, _, _, _)| *i)
                .collect();

            for &pred_idx in &predator_indices {
                if prey_indices.is_empty() {
                    if !self.agents[pred_idx].is_dead() {
                        self.agents[pred_idx].energy -= 2;
                    }
                } else if let Some(&prey_idx) = prey_indices.choose(&mut rng) {
                    if !self.agents[pred_idx].is_dead() && !self.agents[prey_idx].is_dead() {
                        self.agents[prey_idx].is_eaten = true;
                        self.agents[pred_idx].energy += 150;
                        prey_indices.retain(|&p| p != prey_idx);
                    }
                }
            }
        }

        // 4. Миграция — ✅ ИСПРАВЛЕНО: позиция не нужна, используем .values()
        const MAX_DENSITY: usize = 8;
        for agents in agents_by_pos.values() {
            for agent_type in [AgentType::Herbivore, AgentType::Predator] {
                let type_indices: Vec<usize> = agents
                    .iter()
                    .filter(|(_, t, _, _)| *t == agent_type)
                    .map(|(i, _, _, _)| *i)
                    .collect();

                if type_indices.len() > MAX_DENSITY {
                    let excess = type_indices.len() - MAX_DENSITY;
                    for _ in 0..excess {
                        if let Some(&idx) = type_indices.choose(&mut rng) {
                            if !self.agents[idx].is_dead() {
                                let dx = rng.random_range(-2..=2);
                                let dy = rng.random_range(-2..=2);
                                self.agents[idx].pos.0 =
                                    (self.agents[idx].pos.0 + dx).clamp(0, self.width - 1);
                                self.agents[idx].pos.1 =
                                    (self.agents[idx].pos.1 + dy).clamp(0, self.height - 1);
                                self.agents[idx].energy -= 1;
                            }
                        }
                    }
                }
            }
        }

        // 5. Размножение (без изменений)
        let mut reproduction_plan = Vec::new();
        for i in 0..self.agents.len() {
            if !self.agents[i].can_reproduce() {
                continue;
            }
            let partner = (i + 1..self.agents.len()).find(|&j| {
                !self.agents[j].is_dead()
                    && self.agents[i].pos == self.agents[j].pos
                    && self.agents[i].sex != self.agents[j].sex
                    && self.agents[i].agent_type == self.agents[j].agent_type
                    && self.agents[j].can_reproduce()
            });
            if let Some(j) = partner {
                reproduction_plan.push((i, j));
            }
        }

        let mut new_agents = Vec::new();
        for (i, j) in reproduction_plan {
            if self.agents[i].is_dead()
                || self.agents[j].is_dead()
                || self.agents[i].energy < 40
                || self.agents[j].energy < 40
            {
                continue;
            }

            self.agents[i].energy -= 40;
            self.agents[j].energy -= 40;

            let child_pos = self.agents[i].pos;
            let child_sex = if rng.random_bool(0.5) {
                Sex::Male
            } else {
                Sex::Female
            };
            let child_type = self.agents[i].agent_type;
            let child_energy = match child_type {
                AgentType::Herbivore => 100,
                AgentType::Predator => 120,
            };

            self.last_id += 1;
            let mut child = Agent::new(self.last_id, child_pos, child_sex, child_type);
            child.energy = child_energy;
            new_agents.push(child);
        }

        self.agents.retain(|a| !a.is_dead());
        self.agents.extend(new_agents);
    }

    #[allow(dead_code)]
    fn find_partner_in_radius(&self, agent_idx: usize, radius: u32) -> Option<usize> {
        let agent = &self.agents[agent_idx];
        if !agent.can_reproduce() {
            return None;
        }
        for (j, other) in self.agents.iter().enumerate() {
            if j == agent_idx
                || other.is_dead()
                || agent.sex == other.sex
                || agent.agent_type != other.agent_type
                || !other.can_reproduce()
            {
                continue;
            }
            if agent.manhattan_distance(other.pos) <= radius {
                return Some(j);
            }
        }
        None
    }

    // ✅ Упрощённая сигнатура через type aliases
    pub fn get_render_data(&self) -> (AgentRenderData, &PlantsData) {
        (
            self.agents
                .iter()
                .filter(|a| !a.is_dead())
                .map(|a| (a.pos, a.agent_type, a.sex, a.energy))
                .collect(),
            &self.plants,
        )
    }

    pub fn avg_energy(&self) -> f64 {
        if self.agents.is_empty() {
            0.0
        } else {
            self.agents.iter().map(|a| a.energy as f64).sum::<f64>() / self.agents.len() as f64
        }
    }
}
