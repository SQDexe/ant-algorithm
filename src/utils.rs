use crate::tech::PointInfo;

/** `Point` structure, for holding basic point data. */
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
    pub food: u32,
    /** Point's initial amount of pheromones. */
    initial_food: u32
    }

impl Point {
    /** Constructor. */
    #[inline]
    pub const fn new(id: char, x: i16, y: i16, food: u32) -> Self {
        Self {
            id, x, y, food,
            pheromone: 0.0,
            initial_food: food
            }
        }

    /** Reset point's values. */
    pub const fn reset(&mut self) {
        self.food = self.initial_food;
        self.pheromone = 0.0;
        }
    }

/** **Technical part** - trait implementation for converting from `PointInfo`. */
impl From<PointInfo> for Point {
    fn from(value: PointInfo) -> Self {
        match value {
            PointInfo::Empty(id, x, y) =>
                Self::new(id, x, y, 0),
            PointInfo::Food(id, x, y, food) =>
                Self::new(id, x, y, food)    
            }
        }
    }

/** Auxil structure, for calculation puropses. */
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