use {
    clap::Parser,
    crate::{
        consts::default,
        enums::{
            Preference,
            Selection
            }
        }
    };

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(short, long, default_value_t = default::NUM_OF_CYCLES)]
    pub cycles: usize,
    #[clap(short, long, default_value_t = default::NUM_OF_ANTS)]
    pub ants: usize,
    #[clap(short, long, default_value_t = default::NUM_OF_DECISION_POINTS)]
    pub decision: usize,
    #[clap(short, long, default_value_t = default::PHERO_STRENGTH)]
    pub pheromone: f64,
    #[clap(short = 'S', long, value_enum, default_value_t = default::SELECT_METHOD)]
    pub select: Selection,
    #[clap(short = 'P', long, value_enum, default_value_t = default::PREFERENCE_METHOD)]
    pub preference: Preference,
    #[clap(short, long, action, default_value_t = default::RETURN_BEHAVIOUR)]
    pub returns: bool,
    #[clap(short, long, action, default_value_t = default::PRINT_BEHAVIOUR)]
    pub quiet: bool
    }