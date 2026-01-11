use {
    clap::Parser,
    std::path::PathBuf,
    crate::{
        consts::default::*,
        tech::*,
        utils::Point
        }
    };

/** **Technical part** - parsing, and storing of the CL arguments. */
#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Sets number of cycles  
    #[clap(short, long, default_value_t = NUM_OF_CYCLES)]
    pub cycles: usize,
    /// Sets number of ants  
    #[clap(short, long, default_value_t = NUM_OF_ANTS)]
    pub ants: usize,
    /// Sets the strength of pheromones  
    #[clap(short, long, default_value_t = PHERO_STRENGTH)]
    pub pheromone: f64,
    /// Sets the number of decision points  
    #[clap(short, long, default_value_t = NUM_OF_DECISION_POINTS)]
    pub decision: usize,

    /// Sets whether, and how much food is consumed  
    #[clap(short, long, default_value_t = CONSUME_RATE)]
    pub rate: u32,
    /// Sets whether ants return to the anthill  
    #[clap(short = 'R', long, action, default_value_t = RETURN_BEHAVIOUR)]
    pub returns: bool,

    /// Sets how points are selected  
    #[clap(short = 'S', long, value_enum, default_value_t = SELECT_METHOD)]
    pub select: Selection,
    /// Sets how the point preference is calculated  
    #[clap(short = 'P', long, value_enum, default_value_t = PREFERENCE_METHOD)]
    pub preference: Preference,
    /// Sets how the distance between points is calculated  
    #[clap(short = 'M', long, value_enum, default_value_t = METRIC)]
    pub metric: Metric,

    /// Sets the dispersion mode  
    #[clap(short = 'D', long, value_enum, requires = "factor")]
    pub dispersion: Option<Dispersion>,
    /// Sets the coefficient of the dispersion,  
    /// linear      - 0 <= factor,  
    /// exponential - 1 <= factor,  
    /// relative    - 0 <= factor <= 1  
    #[clap(short = 'f', long, requires = "dispersion", verbatim_doc_comment)]
    pub factor: Option<f64>,
    
    /// Sets new world grid,  
    /// must contain at least 2 points,  
    /// the first point is automatically chosen as anthill,  
    /// format 'id,x,y[,food]'  
    /// [default: a,6,1;b,13,1;c,4,3;d,4,5;e,8,5;f,6,8;g,10,8,15]  
    #[clap(short = 'G', long, value_delimiter = ';', verbatim_doc_comment)]
    pub grid: Option<Vec<Point>>,
    /// Sets food at existing points during runtime,  
    /// format 'cycle,id,amount'  
    #[clap(short = 'A', long, value_delimiter = ';', verbatim_doc_comment)]
    pub actions: Option<Vec<Action>>,

    /// Run program in quite mode  
    #[clap(short, long, action, default_value_t = QUIET)]
    pub quiet: bool,
    /// Run program with a seed
    #[clap(short, long)]
    pub seed: Option<u64>,
    /// Mesure how long did the simulation execute 
    #[clap(short, long, action, default_value_t = TIMING)]
    pub timing: bool,
    /// Sets how many times to run the simulation  
    #[clap(short, long, default_value_t = BATCH_SIZE)]
    pub batch: usize,
    /// A file to write statistics to in JSON format,  
    /// will create, or append/truncate existing file,  
    /// search path from current working directory  
    #[clap(short, long, verbatim_doc_comment)]
    pub output: Option<PathBuf>
    }