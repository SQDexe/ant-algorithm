use {
    clap::Parser,
    crate::{
        consts::default::{
            NUM_OF_ANTS,
            NUM_OF_CYCLES,
            NUM_OF_DECISION_POINTS,
            PHERO_STRENGTH,
            PRINT_BEHAVIOUR,
            PREFERENCE_METHOD,
            RETURN_BEHAVIOUR,
            SELECT_METHOD
            },
        enums::{
            Dispersion,
            Preference,
            Selection
            }
        }
    };

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(short, long, default_value_t = NUM_OF_CYCLES)]
    pub cycles: usize,
    #[clap(short, long, default_value_t = NUM_OF_ANTS)]
    pub ants: usize,
    #[clap(short, long, default_value_t = PHERO_STRENGTH)]
    pub pheromone: f64,
    #[clap(short, long, default_value_t = NUM_OF_DECISION_POINTS)]
    pub decision: usize,
    #[clap(short, long, action, default_value_t = RETURN_BEHAVIOUR)]
    pub returns: bool,
    #[clap(short = 'S', long, value_enum, default_value_t = SELECT_METHOD)]
    pub select: Selection,
    #[clap(short = 'P', long, value_enum, default_value_t = PREFERENCE_METHOD)]
    pub preference: Preference,
    #[clap(short = 'D', long, value_enum, requires = "factor")]
    pub dispersion: Option<Dispersion>,
    #[clap(short = 'f', long)]
    pub factor: Option<f64>,
    #[clap(short, long, action, default_value_t = PRINT_BEHAVIOUR)]
    pub quiet: bool
    }