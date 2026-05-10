use {
    clap::{
        value_parser,
        Parser
        },
    std::path::PathBuf,
    crate::{
        consts::{
            bias,
            default::*,
            limits::*
            },
        tech::*,
        utils::Point
        }
    };



/** **Technical part** - parsing, and storing of the CL arguments. */
#[derive(Debug, Clone, Parser)]
#[command(author, version, about)]
pub struct Args {
    /** Sets number of cycles. */
    #[arg(short, long, default_value_t = NUM_OF_CYCLES, value_parser = value_parser!(u64).range(CYCLES_RANGE))]
    cycles: u64,
    /** Sets number of ants. */
    #[arg(short, long, default_value_t = NUM_OF_ANTS, value_parser = value_parser!(u64).range(ANTS_RANGE))]
    ants: u64,
    /** Sets the strength of pheromones. */
    #[arg(short, long, default_value_t = PHERO_STRENGTH)]
    pheromone: f64,
    /** Sets the number of decision points. */
    #[arg(short, long, default_value_t = NUM_OF_DECISION_POINTS, value_parser = value_parser!(u64).range(DECSISION_POINTS_RANGE))]
    decision: u64,

    /** Sets whether, and how much food is consumed. */
    #[arg(short, long, default_value_t = CONSUME_RATE)]
    rate: u32,
    /** Sets whether ants return to the anthill. */
    #[arg(short = 'R', long, action, default_value_t = RETURN_BEHAVIOUR)]
    returns: bool,

    /** Sets how points are selected. */
    #[arg(short = 'S', long, value_enum, default_value_t = SELECT_METHOD)]
    select: Selection,
    /** Sets how the point preference is calculated. */
    #[arg(short = 'P', long, value_enum, default_value_t = PREFERENCE_METHOD)]
    preference: Preference,
    /** Sets how the distance between points is calculated. */
    #[arg(short = 'M', long, value_enum, default_value_t = METRIC)]
    metric: Metric,

    /** Sets the dispersion mode. */
    #[arg(short = 'D', long, value_enum, requires = "factor")]
    dispersion: Option<Dispersion>,
    /**
    Sets the coefficient of the dispersion,  
    linear      - 0 <= factor,  
    exponential - 1 <= factor,  
    relative    - 0 <= factor <= 1
    */
    #[arg(short = 'f', long, requires = "dispersion", verbatim_doc_comment)]
    factor: Option<f64>,
    
    /**
    Sets new world grid,  
    must contain at least 2 points,  
    the first point is automatically chosen as anthill,  
    format `id,x,y[,food]`  
    [default: a,6,1 b,13,1 c,4,3 d,4,5 e,8,5 f,6,8 g,10,8,15]
    */
    #[arg(short = 'G', long, num_args = POINTS_RANGE, verbatim_doc_comment)]
    grid: Option<Vec<Point>>,
    /**
    Sets food at existing points during runtime,  
    format `cycle,id,amount`  
    */
    #[arg(short = 'A', long, verbatim_doc_comment)]
    actions: Option<Vec<Action>>,

    /** Run program in quite mode. */
    #[arg(short, long, action, default_value_t = QUIET)]
    quiet: bool,
    /** Run program with a seed. */
    #[arg(short, long)]
    seed: Option<u64>,
    /** Mesure how long did the simulation execute. */
    #[arg(short, long, action, default_value_t = TIMING)]
    timing: bool,
    /// Sets how many times to run the simulation  
    #[arg(short, long, default_value_t = BATCH_SIZE, value_parser = value_parser!(u64).range(BATCH_RANGE))]
    batch: u64,
    /**
    A file to write statistics to in JSON format,  
    will create, or append/truncate existing file,  
    search path from current working directory
    */
    #[arg(short, long, verbatim_doc_comment)]
    output: Option<PathBuf>
    }

/** **Technical part** - trait implementation for unpacking CLI arguments into config objects. */
impl From<Args> for (ActionsConfig, Config, DisjointConfig) {
    fn from(value: Args) -> Self {
        /* Unpack arguments */
        let Args {
            timing, output,
            cycles, ants, pheromone, decision, rate, returns, select, preference, metric, dispersion, factor, seed,
            grid, actions, quiet, batch
            } = value;

        /* Set actions config */
        let actions_config = ActionsConfig {
            counts_time: timing,
            output_path: output
            };

        /* Set main config */
        let config = Config {
            cycles: cycles as usize,
            ants: ants as usize,
            pheromone,
            decision: decision as usize,
            rate,
            returns,
            select,
            preference,
            metric,
            dispersion,
            factor: factor.unwrap_or(bias::UNKOWN),
            seed
            };

        /* Set disjoint config */
        let disjoint_config = DisjointConfig {
            no_logging: quiet,
            batch_size: batch as usize,
            grid: grid.unwrap_or_else(|| Vec::from(GRID)),
            actions: actions.unwrap_or_default()
            };

        /* Create tuple with configs */
        (actions_config, config, disjoint_config)
        }
    }