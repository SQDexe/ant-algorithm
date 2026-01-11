use {
    anyhow::{
        anyhow,
        Error
        },
    tinyvec::ArrayVec,
    core::str::FromStr
    };

/** `Ant` structure, basic logical unit. */
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
#[derive(Clone, Default)]
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
    pub const fn has_food(&self) -> bool {
        self.food != 0
        }
    }

/** **Technical part** - trait implementation for input parsing. */
impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: ArrayVec<[_; 4]> = s.splitn(4, ',').collect();
        match parts[..] {
            [id, x, y] => Ok(Self::new(
                id.parse()?,
                x.parse()?,
                y.parse()?,
                0
                )),
            [id, x, y, food] => Ok(Self::new(
                id.parse()?,
                x.parse()?,
                y.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow!("Point parsing failed"))
            }
        }
    }

/** Auxil structure, for calculation puropses. */
#[derive(Default)]
pub struct Auxil {
    /** ID of refrenced point. */
    pub id: char,
    /** Calculations output. */
    pub ratio: f64
    }

impl Auxil {
    /** Constructor. */
    #[inline]
    pub const fn new(id: char, ratio: f64) -> Self {
        Self { id, ratio }
        }
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
        tech::DistanceFunction,
        utils::Point
        };

    /** Point prefrence calculation for distance. */
    pub fn distance(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { bias::NEUTRAL / dist_func(x, y, point.x, point.y) }
    /** Point prefrence calculation for pheromones. */
    pub const fn pheromone(point: &Point, _: i16, _: i16, _: DistanceFunction) -> f64
        { point.pheromone + bias::NEUTRAL }
    /** Point prefrence calculation for food. */
    pub const fn food(point: &Point, _: i16, _: i16, _: DistanceFunction) -> f64
        { point.food as f64 + bias::NEUTRAL }
    /** Point prefrence calculation for pheromones, and distance. */
    pub fn phero_dist(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    /** Point prefrence calculation for food, and distance. */
    pub fn food_dist(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { (point.food as f64 + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    /** Point prefrence calculation for pheromones, and food. */
    pub const fn phero_food(point: &Point, _: i16, _: i16, _: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) }
    /** Point prefrence calculation for pheromones, food, and distance. */
    pub fn phero_food_dist(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    }