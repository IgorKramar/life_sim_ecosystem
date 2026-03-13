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
    #[allow(dead_code)] // ID полезен для отладки, даже если не используется в логике
    pub id: u64,
    pub pos: Position,
    pub energy: i32,
    pub sex: Sex,
    pub age: u32,
    pub agent_type: AgentType,
    pub is_eaten: bool,
}

impl Agent {
    pub fn new(id: u64, pos: Position, sex: Sex, agent_type: AgentType) -> Self {
        let initial_energy = match agent_type {
            AgentType::Herbivore => 120,
            AgentType::Predator => 150,
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

    pub fn move_randomly(&mut self, bounds: (i32, i32)) {
        let mut rng = rand::rng();
        self.pos.0 = (self.pos.0 + rng.random_range(-1..=1)).clamp(0, bounds.0 - 1);
        self.pos.1 = (self.pos.1 + rng.random_range(-1..=1)).clamp(0, bounds.1 - 1);
        self.energy -= 1;
    }

    pub fn is_dead(&self) -> bool {
        let max_age = match self.agent_type {
            AgentType::Herbivore => 200,
            AgentType::Predator => 250,
        };
        self.energy <= 0 || self.age > max_age || self.is_eaten
    }

    pub fn manhattan_distance(&self, other: Position) -> u32 {
        ((self.pos.0 - other.0).abs() + (self.pos.1 - other.1).abs()) as u32
    }

    pub fn can_reproduce(&self) -> bool {
        let min_energy = match self.agent_type {
            AgentType::Predator => 120,
            AgentType::Herbivore => 70,
        };
        !self.is_dead() && self.energy >= min_energy
    }
}
