use crate::agent::{Agent, AgentType, Position, Sex};
use crate::config::Config;
use rand::seq::IndexedRandom;
use rand::RngExt;
use std::collections::{HashMap, HashSet};

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
    config: Config, // ✅ Добавили конфиг
}

impl World {
    pub fn new(config: &Config) -> Self {
        Self {
            width: config.world.width,
            height: config.world.height,
            plants: HashMap::new(),
            agents: Vec::new(),
            tick_count: 0,
            last_id: 999,
            config: config.clone(),
        }
    }

    pub fn spawn_initial_agents(&mut self) {
        let mut rng = rand::rng();
        let count = self.config.population.init_population;

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
            let a_type = if rng.random_bool(self.config.population.herbivore_spawn_ratio) {
                AgentType::Herbivore
            } else {
                AgentType::Predator
            };
            let mut agent = Agent::new(self.last_id + 1 + i as u64, pos, sex, a_type, &self.config);

            let max_age = match agent.agent_type {
                AgentType::Herbivore => self.config.herbivore.max_age,
                AgentType::Predator => self.config.predator.max_age,
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
        for _ in 0..self.config.plants.growth_attempts {
            let pos = (
                rng.random_range(0..self.width),
                rng.random_range(0..self.height),
            );
            *self.plants.entry(pos).or_insert(0) = (*self.plants.get(&pos).unwrap_or(&0)
                + rng.random_range(self.config.plants.growth_min..self.config.plants.growth_max))
            .min(self.config.plants.max_energy);
        }

        // 2. Движение и старение
        for agent in &mut self.agents {
            if !agent.is_dead(&self.config) {
                agent.age += 1;
                agent.move_randomly((self.width, self.height), &self.config);
            }
        }

        // 3. Питание — ДВУХФАЗНЫЙ ПОДХОД
        let mut agents_by_pos: AgentsByPos = HashMap::new();
        for (idx, agent) in self.agents.iter().enumerate() {
            if !agent.is_dead(&self.config) {
                agents_by_pos.entry(agent.pos).or_default().push((
                    idx,
                    agent.agent_type,
                    agent.is_eaten,
                    agent.sex,
                ));
            }
        }

        // Фаза 3.2: Растения
        let mut eaten_plants = Vec::new();
        for (pos, agents) in &agents_by_pos {
            if let Some(&plant_energy) = self.plants.get(pos) {
                if plant_energy > 0 {
                    if let Some((herb_idx, _, _eaten, _)) = agents
                        .iter()
                        .find(|(_, t, eaten, _)| *t == AgentType::Herbivore && !eaten)
                    {
                        let herb_idx = *herb_idx;
                        if !self.agents[herb_idx].is_dead(&self.config) {
                            self.agents[herb_idx].energy += plant_energy as i32;
                            eaten_plants.push(*pos);
                        }
                    }
                }
            }
        }
        for pos in eaten_plants {
            self.plants.remove(&pos);
        }

        // Фаза 3.3: Охота хищников
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
                    if !self.agents[pred_idx].is_dead(&self.config) {
                        self.agents[pred_idx].energy -= self.config.predator.hunt_fail_penalty;
                    }
                } else if let Some(&prey_idx) = prey_indices.choose(&mut rng) {
                    if !self.agents[pred_idx].is_dead(&self.config)
                        && !self.agents[prey_idx].is_dead(&self.config)
                    {
                        self.agents[prey_idx].is_eaten = true;
                        self.agents[pred_idx].energy += self.config.predator.kill_reward;
                        prey_indices.retain(|&p| p != prey_idx);
                    }
                }
            }
        }

        // 4. Миграция
        for agents in agents_by_pos.values() {
            for agent_type in [AgentType::Herbivore, AgentType::Predator] {
                let type_indices: Vec<usize> = agents
                    .iter()
                    .filter(|(_, t, _, _)| *t == agent_type)
                    .map(|(i, _, _, _)| *i)
                    .collect();

                if type_indices.len() > self.config.population.max_density_per_type {
                    let excess = type_indices.len() - self.config.population.max_density_per_type;
                    for _ in 0..excess {
                        if let Some(&idx) = type_indices.choose(&mut rng) {
                            if !self.agents[idx].is_dead(&self.config) {
                                let dx = rng.random_range(
                                    -self.config.common.migration_range
                                        ..=self.config.common.migration_range,
                                );
                                let dy = rng.random_range(
                                    -self.config.common.migration_range
                                        ..=self.config.common.migration_range,
                                );
                                self.agents[idx].pos.0 =
                                    (self.agents[idx].pos.0 + dx).clamp(0, self.width - 1);
                                self.agents[idx].pos.1 =
                                    (self.agents[idx].pos.1 + dy).clamp(0, self.height - 1);
                                self.agents[idx].energy -= self.config.common.migration_cost;
                            }
                        }
                    }
                }
            }
        }

        // 5. Размножение
        let mut reproduction_plan = Vec::new();
        let mut used_partners: HashSet<usize> = HashSet::new();

        for i in 0..self.agents.len() {
            if used_partners.contains(&i) {
                continue;
            }
            if !self.agents[i].can_reproduce(&self.config) {
                continue;
            }

            let partner = (i + 1..self.agents.len()).find(|&j| {
                !used_partners.contains(&j)
                    && !self.agents[j].is_dead(&self.config)
                    && self.agents[i].pos == self.agents[j].pos
                    && self.agents[i].sex != self.agents[j].sex
                    && self.agents[i].agent_type == self.agents[j].agent_type
                    && self.agents[j].can_reproduce(&self.config)
            });

            let partner = partner.or_else(|| {
                self.find_partner_in_radius(
                    i,
                    self.config.common.mate_search_radius,
                    &used_partners,
                    &self.config,
                )
            });

            if let Some(j) = partner {
                reproduction_plan.push((i, j));
                used_partners.insert(i);
                used_partners.insert(j);
            }
        }

        let mut new_agents = Vec::new();
        for (i, j) in reproduction_plan {
            if self.agents[i].is_dead(&self.config)
                || self.agents[j].is_dead(&self.config)
                || self.agents[i].energy < self.config.common.min_energy_after_reproduce
                || self.agents[j].energy < self.config.common.min_energy_after_reproduce
            {
                continue;
            }

            self.agents[i].energy -= self.config.common.reproduction_cost;
            self.agents[j].energy -= self.config.common.reproduction_cost;

            let child_pos = self.agents[i].pos;
            let child_sex = if rng.random_bool(0.5) {
                Sex::Male
            } else {
                Sex::Female
            };
            let child_type = self.agents[i].agent_type;
            let child_energy = match child_type {
                AgentType::Herbivore => self.config.herbivore.birth_energy,
                AgentType::Predator => self.config.predator.birth_energy,
            };

            self.last_id += 1;
            let mut child =
                Agent::new(self.last_id, child_pos, child_sex, child_type, &self.config);
            child.energy = child_energy;
            new_agents.push(child);
        }

        self.agents.retain(|a| !a.is_dead(&self.config));
        self.agents.extend(new_agents);
    }

    fn find_partner_in_radius(
        &self,
        agent_idx: usize,
        radius: u32,
        used_partners: &HashSet<usize>,
        config: &Config,
    ) -> Option<usize> {
        let agent = &self.agents[agent_idx];
        if !agent.can_reproduce(config) {
            return None;
        }

        for (j, other) in self.agents.iter().enumerate() {
            if j == agent_idx
                || used_partners.contains(&j)
                || other.is_dead(config)
                || agent.sex == other.sex
                || agent.agent_type != other.agent_type
                || !other.can_reproduce(config)
            {
                continue;
            }

            if agent.manhattan_distance(other.pos) <= radius {
                return Some(j);
            }
        }
        None
    }

    pub fn get_render_data(&self) -> (AgentRenderData, &PlantsData) {
        (
            self.agents
                .iter()
                .filter(|a| !a.is_dead(&self.config))
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
