// const DEFAULT_POSITION: char = 'a';

// pub const fn chars_sub(chr1: char, chr2: char) -> i32 {
//     (chr1 as i32).saturating_sub(chr2 as i32)
//     }

// pub const fn def_chars_sub(chr: char) -> usize {
//     chars_sub(chr, DEFAULT_POSITION) as usize
//     }

/* Functions for calulating distance in metric */
pub mod distance {
    pub fn chebyshev(x0: i32, y0: i32, x1: i32, y1: i32) -> f64 {
        (x0 - x1).abs().max((y0 - y1).abs()) as f64
        }
    pub fn euclidean(x0: i32, y0: i32, x1: i32, y1: i32) -> f64 {
        let (dx, dy) = (x0 - x1, y0 - y1);
        ((dx * dx + dy * dy) as f64).sqrt()
        }
    pub const fn taxicab(x0: i32, y0: i32, x1: i32, y1: i32) -> f64 {
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

pub mod preference {
    use crate::{
        consts::bias,
        tech::DistanceFunction,
        utils::Point
        };

    /* Point preference methods */
    pub fn distance(point: &Point, x: i32, y: i32, dist_func: DistanceFunction) -> f64
        { bias::NEUTRAL / dist_func(x, y, point.x, point.y) }
    pub const fn pheromone(point: &Point, _: i32, _: i32, _: DistanceFunction) -> f64
        { point.pheromone + bias::NEUTRAL }
    pub const fn food(point: &Point, _: i32, _: i32, _: DistanceFunction) -> f64
        { point.food as f64 + bias::NEUTRAL }
    pub fn phero_dist(point: &Point, x: i32, y: i32, dist_func: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    pub fn food_dist(point: &Point, x: i32, y: i32, dist_func: DistanceFunction) -> f64
        { (point.food as f64 + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    pub const fn phero_food(point: &Point, _: i32, _: i32, _: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) }
    pub fn phero_food_dist(point: &Point, x: i32, y: i32, dist_func: DistanceFunction) -> f64
        { (point.pheromone + bias::NEUTRAL) * (point.food as f64 + bias::NEUTRAL) / dist_func(x, y, point.x, point.y) }
    }

/* Basic structure to hold point data */
pub struct Point {
    pub id: char,
    pub x: i32,
    pub y: i32,
    pub pheromone: f64,
    pub food: u32
    }

impl Point {
    pub const fn new(id: char, x: i32, y: i32, food: u32) -> Self {
        Point { id, x, y, food, pheromone: 0.0 }
        }
    }

/* A structure to help in calculations */
pub struct Auxil {
    pub id: char,
    pub ratio: f64
    }

impl Auxil {
    pub const fn new(id: char, ratio: f64) -> Self {
        Auxil { id, ratio }
        }
    }