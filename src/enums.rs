use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Selection {
    Random,
    Roulette,
    Greedy
    }

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Preference {
    Distance,
    Pheromone,
    Compound
    }

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Dispersion {
    Linear,
    Exponential,
    Relative
    }