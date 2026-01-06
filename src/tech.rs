use {
    anyhow::{
        anyhow,
        Error
        },
    clap::ValueEnum,
    derive_more::Display,
    serde::{
        Deserialize,
        Serialize
        },
    sqds_tools::ShowOption,
    core::str::FromStr
    };

/* Technical stuff - alias for distance calculation function */
pub type DistanceFunction = fn (i16, i16, i16, i16) -> f64;

/* Technical stuff - type of next point selection enum */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

/* Technical stuff - ways of calculating preference for the points enum:
P - Pheromone
F - Food
D - Distance
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

/* Technical stuff - types of metrics for distance calculation enum */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

/* Technical stuff - types of pheromone dispersion enum */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Dispersion {
    Linear,
    Exponential,
    Relative
    }

/* Technical stuff - aliases of some types */
#[derive(Clone)]
pub struct Action (
    pub usize,
    pub char,
    pub u32
    );

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Box<[_]> = s.split(',').collect();
        match parts[..] {
            [cycle, id, food] => Ok(Action(
                cycle.parse()?,
                id.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow!("Action parsing failed"))
            }
        }
    }

/* Technical stuff - data holder needed for constructing world's grid */
#[derive(Clone, Copy)]
pub enum PointInfo {
    Food(char, i16, i16, u32),
    Empty(char, i16, i16)
    }

impl PointInfo {
    #[inline]
    pub const fn get_id(&self) -> char {
        match self {
            &Self::Empty(id, ..) => id,
            &Self::Food(id, ..) => id
            }
        }

    #[inline]
    pub const fn get_position(&self) -> (i16, i16) {
        match self {
            &Self::Empty(_, x, y) => (x, y),
            &Self::Food(_, x, y, _) => (x, y)
            }
        }

    #[inline]
    pub const fn has_food(&self) -> bool {
        match self {
            Self::Empty(..) => false,
            Self::Food(.. , 0) => false,
            _ => true
            }
        }

    #[inline]
    pub const fn get_food(&self) -> u32 {
        match self {
            &Self::Food(.. , amount) => amount,
            _ => 0   
            }
        }
    }

impl FromStr for PointInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Box<[_]> = s.split(',').collect();
        match parts[..] {
            [id, x, y] => Ok(PointInfo::Empty(
                id.parse()?,
                x.parse()?,
                y.parse()?
                )),
            [id, x, y, food] => Ok(PointInfo::Food(
                id.parse()?,
                x.parse()?,
                y.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow!("PointInfo parsing failed"))
            }
        }
    }

/* Technical stuff - structure for holding, and printing simulation's configuration */
pub struct Config {
    pub cycles: usize,
    pub ants: usize,
    pub pheromone: f64,
    pub decision: usize,
    pub rate: u32,
    pub returns: bool,
    pub select: Selection,
    pub preference: Preference,
    pub metric: Metric,
    pub dispersion: Option<Dispersion>,
    pub factor: f64,
    pub seed: Option<u64>
    }

impl Config {
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

/* Technical stuff - structure for holding statistics of simulation's run, and operations needed for saving this data */
#[derive(Default, Deserialize, Serialize)]
pub struct Stats {
    pub completed: bool,
    pub pheromone_strengths: Vec<f64>,
    pub average_route_len: f64,
    pub ants_per_phase: Vec<usize>,
    pub average_returns: f64
    }

impl Stats {
    pub fn get_pheromone_per_route(&self) -> Vec<f64> {
        self.pheromone_strengths.iter()
            .map(|phero| phero / self.average_returns)
            .collect()
        }
    }