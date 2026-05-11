#![allow(unused)]

use {
    clap::ValueEnum,
    derive_more::Display,
    serde::{
        Deserialize,
        Serialize
        },
    sqds_tools::ShowOption,
    std::path::PathBuf,
    core::{
        char::ParseCharError,
        str::FromStr,
        fmt::{
            Write,
            Formatter,
            Result as FmtResult,
            Display
            },
        iter::zip,
        },
    crate::{
        consts::limits::{
            DISPERSION_LINEAR_RANGE,
            DISPERSION_EXPONENTIAL_RANGE,
            DISPERSION_RELATIVE_RANGE
            },
        error::ParseActionError,
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



/** **Technical part** - type to represent a point ID. */
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id ( char );

impl Id {
    /** Constructor. */
    #[inline]
    pub const fn new(value: char) -> Self {
        Self ( value )
        }

    /** Inner char getter. */
    #[inline]
    pub const fn get(&self) -> char {
        self.0
        }
    }

/* **Technical part** - trait implementation for ID printing. */
impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_char(self.0)
        }
    }

/** **Technical part** - trait implementation for ID parsing. */
impl FromStr for Id {
    type Err = ParseCharError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s.parse()?;

        Ok(Self( inner ))
        }
    }

/** **Technical part** - type to represent a route. */
#[derive(Debug, Clone, Default)]
#[repr(transparent)]
pub struct Route {
    inner: String
    }

impl Route {
    /** Constructor. */
    pub const fn new() -> Self {
        Self {
            inner: String::new()
            }
        }

    /** Constructor with capacity. */
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: String::with_capacity(capacity)
            }
        }

    /** `len` getter. */
    #[inline]
    pub const fn len(&self) -> usize {
        self.inner.len()
        }

    /** Checks whether the route contains the ID. */
    pub fn contains(&self, id: &Id) -> bool {
        self.inner.contains(id.get())
        }

    /** Get the first ID in the route. */
    pub fn first(&self) -> Option<Id> {
        self.inner.chars()
            .next()
            .map(Id::new)
        }

    /** Get the last ID in the route. */
    pub fn last(&self) -> Option<Id> {
        self.inner.chars()
            .last()
            .map(Id::new)
        }

    /** Push new ID to the route. */
    pub fn push(&mut self, id: Id) {
        self.inner.push(id.get());
        }

    /** Clear the route. */
    pub fn clear(&mut self) {
        self.inner.clear();
        }
    }

/* **Technical part** - trait implementation for route printing. */
impl Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.inner)
        }
    }

/* **Technical part** - type of next point selection enum. */
#[derive(Debug, Clone, Copy, Display, ValueEnum)]
pub enum Selection {
    /** Greedy selection method. */
    Greedy,
    /** Random selection method. */
    Random,
    /** Roulette selection method. */
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
    /** Point prefrence calculation for distance. */
    Distance,
    /** Point prefrence calculation for pheromones. */
    Pheromone,
    /** Point prefrence calculation for food. */
    Food,
    /** Point prefrence calculation for pheromones, and distance. */
    PD,
    /** Point prefrence calculation for food, and distance. */
    FD,
    /** Point prefrence calculation for pheromones, and food. */
    PF,
    /** Point prefrence calculation for pheromones, food, and distance. */
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
    /** Chebyshev metric distance calculation. */
    Chebyshev,
    /** Euclidean metric distance calculation. */
    Euclidean,
    /** Taxicab metric distance calculation. */
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
    /** Linear dispersion calculation. */
    Linear,
    /** Exponential dispersion calculation. */
    Exponential,
    /** Relative dispersion calculation. */
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
pub struct Action {
    /** Cycle at which to make the action. */
    pub cycle: usize,
    /** Point ID at which to add the food. */
    pub id: Id,
    /** Amount of food to add. */
    pub food_amount: u32
    }

/** **Technical part** - trait implementation for input parsing. */
impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /* Collect split text elements */
        let mut parts = s.splitn(3, ',');

        /* Try retriving elements from the iterator */
        let str_elements = [parts.next(), parts.next(), parts.next()];
        let [Some(str_cycle), Some(str_id), Some(str_food)] = str_elements else {
            return Err(ParseActionError::InvalidFormat);
            };

        /* Try parsing main elements */
        let (cycle, id, food_amount) = (
            str_cycle.parse()
                .map_err(|_| ParseActionError::FailedParseCycle)?,
            str_id.parse()
                .map_err(|_| ParseActionError::FailedParseId)?,
            str_food.parse()
                .map_err(|_| ParseActionError::FailedParseFoodAmount)?
            );

        /* Create action */
        Ok(Self { cycle, id, food_amount })
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

/** **Technical part** - structure for grouping of simulation's configuration of outer actions. */
#[derive(Debug, Clone)]
pub struct ActionsConfig {
    /** Whether simulation time should be displayed. */
    pub counts_time: bool,
    /** Possible path for the statistics' output file. */
    pub output_path: Option<PathBuf>
    }

/** **Technical part** - structure for grouping of disjoint simulation's configuration. */
#[derive(Debug, Clone)]
pub struct DisjointConfig {
    /** Whether simulation time should be displayed. */
    pub no_logging: bool,
    /** Possible path for the statistics' output file. */
    pub batch_size: usize,
    pub grid: Vec<Point>,
    pub actions: Vec<Action>
    }

/** **Technical part** - structure for holding statistics of simulation's run, and operations needed for saving this data. */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stats {
    /** Whether all ants have finished their routes. */
    pub completed: bool,
    /** Final pheromone strengths for points in declaration order. */
    pub pheromone_strengths: Box<[f64]>,
    /** Ant's average route length. */
    pub average_route_len: f64,
    /** Number of satiated ants for each cycle. */
    pub ants_per_phase: Box<[usize]>,
    /** Average number of routes per ant. */
    pub completed_routes: f64
    }

impl Stats {
    /** `pheromone_per_route` getter. */
    pub fn pheromone_per_route(&self) -> Box<[f64]> {
        self.pheromone_strengths.iter()
            .map(|phero| phero / self.completed_routes)
            .collect()
        }
    }

/** **Technical part** - structure for holding averaged statistics of a batch simulation. */
#[derive(Debug, Clone)]
pub struct AveragedStats {
    /** Number of times simulation was run. */
    pub batch_size: usize,
    /** Number of times all ants reached the food source. */
    pub total_complete_routes: usize,
    /** Average amount of pheromones on each point. */
    pub avg_pheromone_strengths: Box<[f64]>,
    /** Average ants' route length. */
    pub avg_route_len: f64,
    /** Average amount of satiated ants for each cycle. */
    pub avg_ants_per_phase: Box<[f64]>,
    /** Average number of routes per ant. */
    pub avg_completed_routes: f64,
    /** Average amount of pheromones on each point per average routes. */
    pub avg_pheromone_per_route: Box<[f64]>,
    }

impl AveragedStats {
    /** Constructor. */
    pub fn new(stats: &[Stats], cycles: usize, number_of_points: usize, batch_size: usize) -> Self {
        let batch = batch_size as f64;

        /* Set empty containers */
        let mut total_route_len = 0.0;
        let mut total_returns = 0.0;
        let mut total_complete_routes = 0;

        let mut total_pheromone_strengths = vec![0.0; number_of_points].into_boxed_slice();
        let mut total_ants_per_phase = vec![0; cycles].into_boxed_slice();
        let mut total_pheromone_per_route = vec![0.0; number_of_points].into_boxed_slice();

        /* Get total statistics for whole batch */
        for stat in stats {
            total_route_len += stat.average_route_len;
            total_returns += stat.completed_routes;
            total_complete_routes += stat.completed as usize;

            let values = zip(&stat.pheromone_strengths, stat.pheromone_per_route());
            let totals = zip(&mut total_pheromone_strengths, &mut total_pheromone_per_route);
            for ((total_strength, total_avg_strength), (strength, avg_strength)) in zip(totals, values) {
                *total_strength += strength;
                *total_avg_strength += avg_strength;
                }

            for (total_ants, &ants) in zip(&mut total_ants_per_phase, &stat.ants_per_phase) {
                *total_ants += ants;
                }
            }

        /* Average out the totals */
        for strength in &mut total_pheromone_strengths {
            *strength /= batch;
            }
        for avg_strength in &mut total_pheromone_per_route {
            *avg_strength /= batch;
            }
        let avg_ants_per_phase = total_ants_per_phase.into_iter()
            .map(|ants| (ants as f64) / batch)
            .collect();

        /* Create averaged stats */
        Self {
            batch_size,
            total_complete_routes,
            avg_route_len: total_route_len / batch,
            avg_completed_routes: total_returns / batch,
            avg_pheromone_strengths: total_pheromone_strengths,
            avg_pheromone_per_route: total_pheromone_per_route,
            avg_ants_per_phase
            }
        }
    }