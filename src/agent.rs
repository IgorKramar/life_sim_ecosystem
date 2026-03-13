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
}
