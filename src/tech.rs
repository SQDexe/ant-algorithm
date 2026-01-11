use {
    anyhow::{ anyhow, Error },
    clap::ValueEnum,
    derive_more::Display,
    serde::{ Deserialize, Serialize },
    tinyvec::ArrayVec,
    sqds_tools::ShowOption,
    core::str::FromStr
    };

/** **Technical part** - alias for distance calculation function. */
pub type DistanceFunction = fn (i16, i16, i16, i16) -> f64;

/* **Technical part** - type of next point selection enum. */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

/**
**Technical part** - ways of calculating preference for the points enum:
- P - Pheromone
- F - Food
- D - Distance
*/
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Preference {
    Distance,
    Pheromone,
    Food,
    PD,
    FD,
    PF,
    PFD
    }

/** **Technical part** - types of metrics for distance calculation enum. */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

/** **Technical part** - types of pheromone dispersion enum. */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Dispersion {
    Linear,
    Exponential,
    Relative
    }

/** **Technical part** - cycle action. */
#[derive(Clone)]
pub struct Action (
    pub usize,
    pub char,
    pub u32
    );

/** **Technical part** - trait implementation for input parsing. */
impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: ArrayVec<[_; 3]> = s.splitn(3, ',').collect();
        if let [cycle, id, food] = parts[..] {
            return Ok(Self(
                cycle.parse()?,
                id.parse()?,
                food.parse()?
                ));
            }

        Err(anyhow!("Action parsing failed"))
        }
    }

/** **Technical part** - structure for holding, and printing simulation's configuration. */
pub struct Config {
    /** Number of cycles. */
    pub cycles: usize,
    /** Number of ants. */
    pub ants: usize,
    /** Pheromone strengths. */
    pub pheromone: f64,
    /** Number of decision points. */
    pub decision: usize,
    /** Consumption rate. */
    pub rate: u32,
    /** Whether ants return, after reaching food. */
    pub returns: bool,
    /** Point selection method. */
    pub select: Selection,
    /** Point prefrence calculation method. */
    pub preference: Preference,
    /** Distance calculation metric. */
    pub metric: Metric,
    /** Possible dispersion behaviour. */
    pub dispersion: Option<Dispersion>,
    /** Possible dispersion coefficient. */
    pub factor: f64,
    /** Possible random number generator seed. */
    pub seed: Option<u64>
    }

impl Config {
    /** Show operation for the settings. */
    pub fn show(&self) {
        println!(
"o> -------- SETTINGS -------- <o
|            cycles: {}
|              ants: {}
|         pheromone: {}
|   decision points: {}
|   consumtion rate: {}
|           returns: {}
|         selection: {}
|       calculation: {}
|            metric: {}
|        dispersion: {}
| dispersion factor: {}
|              seed: {}
o> -------------------------- <o",
            self.cycles, self.ants, self.pheromone, self.decision,
            self.rate, self.returns,
            self.select, self.preference, self.metric,
            self.dispersion.show_or_none(), self.factor,
            self.seed.show_or_none()
            );
        }
    }

/** **Technical part** - structure for holding statistics of simulation's run, and operations needed for saving this data. */
#[derive(Default, Deserialize, Serialize)]
pub struct Stats {
    /** Whether all ants have finished their routes. */
    pub completed: bool,
    /** Final pheromone strengths for points in declaration order. */
    pub pheromone_strengths: Vec<f64>,
    /** Ant's average route length. */
    pub average_route_len: f64,
    /** Number of satiated ants for each cycle. */
    pub ants_per_phase: Vec<usize>,
    /** Average number of routes per ant. */
    pub average_returns: f64
    }

impl Stats {
    /** `pheromone_per_route` getter. */
    pub fn get_pheromone_per_route(&self) -> Vec<f64> {
        self.pheromone_strengths.iter()
            .map(|phero| phero / self.average_returns)
            .collect()
        }
    }