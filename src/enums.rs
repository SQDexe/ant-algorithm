use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Selection {
    Random,
    Roulette,
    Greedy
    }

#[derive(ValueEnum, Clone, Debug)]
pub enum Preference {
    Distance,
    Pheromone,
    Compound
    }