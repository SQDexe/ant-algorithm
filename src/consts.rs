/** Default simulation settings. */
pub mod default {
    use crate::{
        tech::*,
        utils::Point
        };

    /** Default world grid. */
    pub const GRID: [Point; 7] = [
        Point::new('a',  6, 1,  0),
        Point::new('b', 13, 1,  0),
        Point::new('c',  4, 3,  0),
        Point::new('d',  4, 5,  0),
        Point::new('e',  8, 5,  0),
        Point::new('f',  6, 8,  0),
        Point::new('g', 10, 8, 15)
        ];
    /** Default number of cycles. */
    pub const NUM_OF_CYCLES: u64 = 8;
    /** Default number of ants. */
    pub const NUM_OF_ANTS: u64 = 15;
    /** Default number of decisin points. */
    pub const NUM_OF_DECISION_POINTS: u64 = 3;
    /** Default pheromone strength. */
    pub const PHERO_STRENGTH: f64 = 1.0;
    /** Default return behaviour. */
    pub const RETURN_BEHAVIOUR: bool = false;
    /** Default food consuming rate. */
    pub const CONSUME_RATE: u32 = 0;
    /** Default point selection method. */
    pub const SELECT_METHOD: Selection = Selection::Roulette;
    /** Default point preference method. */
    pub const PREFERENCE_METHOD: Preference = Preference::PD;
    /** Default distance calculation metric. */
    pub const METRIC: Metric = Metric::Euclidean;
    /** Default logging behaviour. */
    pub const QUIET: bool = false;
    /** Default computation duration logging behaviour. */
    pub const TIMING: bool = false;
    /** Default number of simulation repetitions. */
    pub const BATCH_SIZE: u64 = 1;
    }

/** Values for different kinds of calculations. */
pub mod bias {
    /** Neutral bias */
    pub const NEUTRAL: f64 = 1.0;
    /** Unknown value */
    pub const UNKOWN: f64 = f64::NAN;
    /** Maximal bias, changed from `1e6_f64` */
    pub const GREAT: f64 = f64::MAX;
    /** Minimal bias, changed from `1e-6_f64` */
    pub const MINUTE: f64 = f64::MIN_POSITIVE;
    }

/** Limitations for arguments. */
pub mod limits {
    use core::ops::{
        Range,
        RangeFrom,
        RangeInclusive
        };

    /** Allowed range for point coordinates. */
    pub const GRID_RANGE: RangeInclusive<i16> = -99 ..= 99;
    /** Allowed range for number of points. */
    pub const POINTS_RANGE: Range<usize> = 2 .. 100;
    /** Allowed range for number of decision points. */
    pub const DECSISION_POINTS_RANGE: Range<u64> = 
        POINTS_RANGE.start as u64 .. POINTS_RANGE.end as u64;
    /** Allowed range for number of ants. */
    pub const ANTS_RANGE: RangeInclusive<u64> = 1 ..= 0xffffff;
    /** Allowed range for number of cycles. */
    pub const CYCLES_RANGE: Range<u64> = 1 .. 100;
    /** Allowed range for pheromone strength. */
    pub const PHERO_RANGE: RangeFrom<f64> = 0.0 ..;
    /** Allowed range for simulation repetitions. */
    pub const BATCH_RANGE: Range<u64> = 1 .. 1000;
    /** Allowed range for linear dispersion coefficient. */
    pub const DISP_LINEAR_RANGE: RangeFrom<f64> = 0.0 ..;
    /** Allowed range for exponential dispersion coefficient. */
    pub const DISP_EXPONENTIAL_RANGE: RangeFrom<f64> = 1.0 ..;
    /** Allowed range for relative dispersion coefficient. */
    pub const DISP_RELATIVE_RANGE: RangeInclusive<f64> = 0.0 ..= 1.0;

    /** Shorthand for maximal number of points. */
    pub const MAX_POINTS: usize = POINTS_RANGE.end;
    }