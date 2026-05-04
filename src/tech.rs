use {
    anyhow::{
        anyhow,
        Error
        },
    arrayvec::ArrayVec,
    clap::ValueEnum,
    derive_more::Display,
    serde::{
        Deserialize,
        Serialize
        },
    sqds_tools::ShowOption,
    core::str::FromStr,
    crate::{
        consts::limits::{
            DISPERSION_LINEAR_RANGE,
            DISPERSION_EXPONENTIAL_RANGE,
            DISPERSION_RELATIVE_RANGE
            },
        utils::{
            Auxil,
            Point,
            disperse::*,
            distance::*,
            preference::*,
            selection::*
            }
        }
    };



/* **Technical part** - type of next point selection enum. */
#[derive(Debug, Clone, Copy, Display, ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

impl Selection {
    /** Calculates the new index based on the number of decision points, and slice of helper structs. */
    pub fn calculate(&self, decision_points: usize, auxils: &[Auxil]) -> usize {
        match self {
            Self::Greedy => greedy(),
            Self::Random => randomly(decision_points),
            Self::Roulette => roulette(decision_points, auxils)
            }
        }
    }

/**
**Technical part** - ways of calculating preference for the points enum:
- P - Pheromone
- F - Food
- D - Distance
*/
#[derive(Debug, Clone, Copy, Display, ValueEnum)]
pub enum Preference {
    Distance,
    Pheromone,
    Food,
    PD,
    FD,
    PF,
    PFD
    }

impl Preference {
    /** Calculates the point preference based on the point's values, current coordinates, and the metric. */
    pub fn calculate(&self, point: &Point, x: i16, y: i16, metric: Metric) -> f64 {
        match self {
            Self::Distance => distance(point, x, y, metric),
            Self::Pheromone => pheromone(point),
            Self::Food => food(point),
            Self::PD => phero_dist(point, x, y, metric),
            Self::FD => food_dist(point, x, y, metric),
            Self::PF => phero_food(point),
            Self::PFD => phero_food_dist(point, x, y, metric),
            }
        }
    }

/** **Technical part** - types of metrics for distance calculation enum. */
#[derive(Debug, Clone, Copy, Display, ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

impl Metric {
    /** Calculates the distance based on points' coordinates. */
    pub fn calculate(&self, x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        match self {
            Self::Chebyshev => chebyshev(x0, y0, x1, y1),
            Self::Euclidean => euclidean(x0, y0, x1, y1),
            Self::Taxicab => taxicab(x0, y0, x1, y1)
            }
        }
    }

/** **Technical part** - types of pheromone dispersion enum. */
#[derive(Debug, Clone, Copy, Display, ValueEnum)]
pub enum Dispersion {
    Linear,
    Exponential,
    Relative
    }

impl Dispersion {
    /** `factor` coefficient checker. */
    pub fn is_factor_valid(&self, factor: &f64) -> bool {
        match self {
            Self::Linear => DISPERSION_LINEAR_RANGE.contains(factor),
            Self::Exponential => DISPERSION_EXPONENTIAL_RANGE.contains(factor),
            Self::Relative => DISPERSION_RELATIVE_RANGE.contains(factor)
            }
        }

    /** Calculates the dispersion based on the point's values, and the coefficient */
    pub fn calculate(&self, point: &Point, factor: f64) -> f64 {
        match self {
            Self::Linear => linear(point, factor),
            Self::Exponential => exponential(point, factor),
            Self::Relative => relative(point, factor)
            }
        }
    }

/** **Technical part** - cycle action. */
#[derive(Debug, Clone)]
pub struct Action (
    pub usize,
    pub char,
    pub u32
    );

/** **Technical part** - trait implementation for input parsing. */
impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /* Collect split text elements */
        let parts: ArrayVec<_, 3> = s.splitn(3, ',').collect();

        /* Try destructing, and parsing */
        let [cycle, id, food] = parts.as_slice() else {
            return Err(anyhow!("Incorrect Action format"));
            };

        /* Ouput */
        Ok(Self(
            cycle.parse()?,
            id.parse()?,
            food.parse()?
            ))
        }
    }

/** **Technical part** - structure for holding, and printing simulation's configuration. */
#[derive(Debug, Clone)]
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
|          cycles: {}
|            ants: {}
|       pheromone: {}
| decision points: {}
| consumtion rate: {}
|         returns: {}
|       selection: {}
|     calculation: {}
|          metric: {}
|      dispersion: {}
|          factor: {}
|            seed: {}
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
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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