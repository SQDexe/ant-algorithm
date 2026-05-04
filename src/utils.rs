use {
    anyhow::{
        anyhow,
        Error
        },
    arrayvec::ArrayVec,
    core::str::FromStr,
    crate::consts::limits::GRID_RANGE
    };



/** `Ant` structure, basic logical unit. */
#[derive(Debug, Clone)]
pub struct Ant {
    /** Check, whether the ant found food. */
    pub satiated: bool,
    /** Ant's current route. */
    pub route: String,
    /** Number of routes the ant went through. */
    pub routes_counter: u8
    }

impl Ant {
    /** Constructor. */
    pub fn new(anthill_id: char, max_length: usize) -> Self {
        let mut route = String::with_capacity(max_length);
        route.push(anthill_id);

        /* Create ant */
        Self {
            satiated: false,
            route,
            routes_counter: 0
            }
        }

    /** Reset the postion, route, and unmark the ant. */
    pub fn return_to(&mut self, destination: char) {
        self.satiated = false;
        self.route.clear();
        self.route.push(destination);
        }

    /** Reset the position, and the counter. */
    pub fn reset(&mut self, destination: char) {
        self.return_to(destination);
        self.routes_counter = 0;
        }
    }

/** `Point` structure, for holding basic point data. */
#[derive(Debug, Clone, Default)]
pub struct Point {
    /** Unique point ID. */
    pub id: char,
    /** Point's x coordinate. */
    pub x: i16,
    /** Point's y coordinate. */
    pub y: i16,
    /** Point's current amount of pheromones. */
    pub pheromone: f64,
    /** Point's current amount of food. */
    pub food: u32
    }

impl Point {
    /** Constructor. */
    #[inline]
    pub const fn new(id: char, x: i16, y: i16, food: u32) -> Self {
        Self { id, x, y, food, pheromone: 0.0 }
        }

    /** `food` checker. */
    pub const fn is_empty(&self) -> bool {
        self.food == 0
        }
    }

/** **Technical part** - trait implementation for input parsing. */
impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /* Collect split text elements */
        let parts: ArrayVec<_, 4> = s.splitn(4, ',').collect();

        /* Try destructing, otherwise throw error */
        let (str_id, str_x, str_y, food) = match parts.as_slice() {
            [id, x, y, food] => (id, x, y, food.parse()?),
            [id, x, y] => (id, x, y, 0),
            _ => return Err(anyhow!("Incorrect Point data format"))
            };

        let (id, x, y) = (
            str_id.parse()?,
            str_x.parse()?,
            str_y.parse()?
            );

        if ! GRID_RANGE.contains(&x) {
            return Err(anyhow!("Point's x coordiante outside of range"));
            }

        if ! GRID_RANGE.contains(&y) {
            return Err(anyhow!("Point's y coordiante outside of range"));
            }

        /* Try parsing, and ouptut */
        Ok(Self::new(id, x, y, food))
        }
    }

/** Auxil structure, for calculation puropses. */
#[derive(Debug, Clone, Default)]
pub struct Auxil {
    /** ID of refrenced point. */
    pub id: char,
    /** Calculations output. */
    pub ratio: f64
    }

/** Functions for calulating distance metric. */
pub mod distance {
    /** **Technical part** - helper function for calculating delta of values. */
    #[inline]
    const fn delta(x0: i16, x1: i16) -> i16 {
        x0 - x1
        }
    /** **Technical part** - helper function for calculating absolute value of a delta. */
    #[inline]
    const fn delta_abs(x0: i16, x1: i16) -> i16 {
        delta(x0, x1).abs()
        }

    /** Chebyshev metric distance calculation. */
    pub const fn chebyshev(x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        f64::max(
            delta_abs(x0, x1) as f64,
            delta_abs(y0, y1) as f64
            )
        }
    /** Euclidean metric distance calculation. */
    pub fn euclidean(x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        f64::hypot(
            delta(x0, x1) as f64,
            delta(y0, y1) as f64
            )
        }
    /** Taxicab metric distance calculation. */
    pub const fn taxicab(x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        (delta_abs(x0, x1) + delta_abs(y0, y1)) as f64
        }
    }

/** Functions for calculating pheromone dispersion. */
pub mod disperse {
    use crate::utils::Point;

    /** Linear dispersion calculation. */
    #[inline]
    pub const fn linear(point: &Point, factor: f64) -> f64
        { point.pheromone - factor }
    /** Exponential dispersion calculation. */
    #[inline]
    pub const fn exponential(point: &Point, factor: f64) -> f64
        { point.pheromone / factor }
    /** Relative dispersion calculation. */
    #[inline]
    pub const fn relative(point: &Point, factor: f64) -> f64
        { point.pheromone * (1.0 - factor) }
    }

/** Functions for calculating point preference. */
pub mod preference {
    use crate::{
        consts::bias,
        tech::Metric,
        utils::Point
        };

    /** Point prefrence calculation for distance. */
    pub fn distance(point: &Point, x: i16, y: i16, metric: Metric) -> f64
        { bias::NEUTRAL / metric.calculate(x, y, point.x, point.y) }
    /** Point prefrence calculation for pheromones. */
    pub const fn pheromone(point: &Point) -> f64
        { point.pheromone + bias::NEUTRAL }
    /** Point prefrence calculation for food. */
    pub const fn food(point: &Point) -> f64
        { point.food as f64 + bias::NEUTRAL }
    /** Point prefrence calculation for pheromones, and distance. */
    pub fn phero_dist(point: &Point, x: i16, y: i16, metric: Metric) -> f64
        { (point.pheromone + bias::NEUTRAL) / metric.calculate(x, y, point.x, point.y) }
    /** Point prefrence calculation for food, and distance. */
    pub fn food_dist(point: &Point, x: i16, y: i16, metric: Metric) -> f64
        { (point.food as f64 + bias::NEUTRAL) / metric.calculate(x, y, point.x, point.y) }
    /** Point prefrence calculation for pheromones, and food. */
    pub const fn phero_food(point: &Point) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) }
    /** Point prefrence calculation for pheromones, food, and distance. */
    pub fn phero_food_dist(point: &Point, x: i16, y: i16, metric: Metric) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) / metric.calculate(x, y, point.x, point.y) }
    }

/** Functions for calculating new indices. */
pub mod selection {
    use {
        fastrand::{
            f64 as random_f64,
            usize as random_usize
            },
        crate::utils::Auxil
        };

    /** Greedy selection method. */
    #[inline]
    pub const fn greedy() -> usize
        { 0 }
    /** Random selection method. */
    pub fn randomly(decision_points: usize) -> usize {
        random_usize(.. decision_points)
        }
    /** Roulette selection method. */
    pub fn roulette(decision_points: usize, axuils: &[Auxil]) -> usize {
        /* Sum the wheel */
        let sum: f64 = axuils.iter()
            .map(|auxil| auxil.ratio)
            .sum();
        
        /* Select random chance, and scale it with the sum */
        let chance = random_f64() * sum;

        /* Spin the wheel until it stops */
        let mut accumulator = 0.0;
        for (index, auxil) in axuils.into_iter().enumerate() {
            accumulator += auxil.ratio;
            if chance < accumulator {
                return index;
                }
            }

        /* Fallback for rare float precision errors */
        decision_points.saturating_sub(1)
        }
    }