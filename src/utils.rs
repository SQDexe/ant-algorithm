/* Functions for calulating distance in metric */
pub mod distance {
    pub fn chebyshev(x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        (x0 - x1).abs().max((y0 - y1).abs()) as f64
        }
    pub fn euclidean(x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        let (dx, dy) = (x0 - x1, y0 - y1);
        ((dx * dx + dy * dy) as f64).sqrt()
        }
    pub const fn taxicab(x0: i16, y0: i16, x1: i16, y1: i16) -> f64 {
        ((x0 - x1).abs() + (y0 - y1).abs()) as f64
        }
    }

/* Functions for calculating pheromone dispersion */
pub mod disperse {
    use crate::utils::Point;

    pub const fn linear(point: &Point, factor: f64) -> f64
        { point.pheromone - factor }
    pub const fn exponential(point: &Point, factor: f64) -> f64
        { point.pheromone / factor }
    pub const fn relative(point: &Point, factor: f64) -> f64
        { point.pheromone * (1.0 - factor) }
    }

/* Functions for calculating point preference */
pub mod preference {
    use crate::{
        consts::bias,
        tech::DistanceFunction,
        utils::Point
        };

    pub fn distance(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { bias::NEUTRAL / dist_func(x, y, point.x, point.y) }
    pub const fn pheromone(point: &Point, _: i16, _: i16, _: DistanceFunction) -> f64
        { point.pheromone + bias::NEUTRAL }
    pub const fn food(point: &Point, _: i16, _: i16, _: DistanceFunction) -> f64
        { point.food as f64 + bias::NEUTRAL }
    pub fn phero_dist(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    pub fn food_dist(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { (point.food as f64 + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    pub const fn phero_food(point: &Point, _: i16, _: i16, _: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) }
    pub fn phero_food_dist(point: &Point, x: i16, y: i16, dist_func: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    }

/* Point structure, for holding basic point data */
pub struct Point {
    pub id: char,
    pub x: i16,
    pub y: i16,
    pub pheromone: f64,
    pub food: u32
    }

impl Point {
    pub const fn new(id: char, x: i16, y: i16, food: u32) -> Self {
        Point { id, x, y, food, pheromone: 0.0 }
        }
    }

/* Auxil structure, for calculation puropses */
pub struct Auxil {
    pub id: char,
    pub ratio: f64
    }

impl Auxil {
    pub const fn new(id: char, ratio: f64) -> Self {
        Auxil { id, ratio }
        }
    }