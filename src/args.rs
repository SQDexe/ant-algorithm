use {
    clap::Parser,
    crate::consts::{
        Selection,
        DEFAULT_NUM_OF_ANTS,
        DEFAULT_NUM_OF_CYCLES,
        DEFAULT_NUM_OF_DECISION_POINTS,
        DEFAULT_PHERO_STRENGTH,
        DEFAULT_SELECT_METHOD
        }
    };

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(short, long, default_value_t = DEFAULT_NUM_OF_ANTS)]
    pub ants: usize,
    #[clap(short, long, default_value_t = DEFAULT_NUM_OF_CYCLES)]
    pub cycles: usize,
    #[clap(short, long, default_value_t = DEFAULT_NUM_OF_DECISION_POINTS)]
    pub decision: usize,
    #[clap(short, long, value_enum, default_value_t = DEFAULT_SELECT_METHOD)]
    pub method: Selection,
    #[clap(short, long, default_value_t = DEFAULT_PHERO_STRENGTH)]
    pub phero: f64,
    #[clap(long, short, action, default_value_t = false)]
    pub quiet: bool
    }