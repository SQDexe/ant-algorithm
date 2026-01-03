use crate::tech::PointInfo;

/* Default world setup */
const NUM_OF_POINTS: usize = 7;
pub const GRID: [PointInfo; NUM_OF_POINTS] = [
    PointInfo::Empty('a', 6, 1),
    PointInfo::Empty('b', 13, 1),
    PointInfo::Empty('c', 4, 3),
    PointInfo::Empty('d', 4, 5),
    PointInfo::Empty('e', 8, 5),
    PointInfo::Empty('f', 6, 8),
    PointInfo::Food('g', 10, 8, 15)
    ];

/* Default simulation settings */
pub mod default {
    use crate::tech::{
        Preference,
        Selection,
        Metric
        };

    pub const NUM_OF_CYCLES: usize = 8;
    pub const NUM_OF_ANTS: usize = 15;
    pub const NUM_OF_DECISION_POINTS: usize = 3;
    pub const PHERO_STRENGTH: f64 = 1.0;
    pub const RETURN_BEHAVIOUR: bool = false;
    pub const CONSUME_RATE: u32 = 0;
    pub const SELECT_METHOD: Selection = Selection::Roulette;
    pub const PREFERENCE_METHOD: Preference = Preference::PD;
    pub const METRIC: Metric = Metric::Euclidean;
    pub const QUIET: bool = false;
    pub const TIMING: bool = false;
    pub const BATCH_SIZE: usize = 1;
    }

/* Values for different kinds of calculations */
pub mod bias {
    pub const NEUTRAL: f64 = 1.0;
    pub const UNKOWN: f64 = f64::NAN;
    /* changed from 1e6_f64 */
    pub const GREAT: f64 = f64::MAX;
    /* changed from 1e-6_f64 */
    pub const MINUTE: f64 = f64::MIN_POSITIVE;
    }

/* Limitations for arguments */
pub mod limits {
    use core::ops::{
        Range,
        RangeFrom,
        RangeInclusive
        };

    pub const GRID_RANGE: RangeInclusive<i16> = -99 ..= 99;
    pub const POINTS_RNAGE: Range<usize> = 2 .. 1000;
    pub const ANTS_RANGE: RangeInclusive<usize> = 1 ..= 0xffffff;
    pub const CYCLES_RANGE: Range<usize> = 1 .. 100;
    pub const PHERO_RANGE: RangeFrom<f64> = 0.0 ..;
    pub const BATCH_RANGE: Range<usize> = 1 .. 1000;
    pub const DISP_LINEAR_RANGE: RangeFrom<f64> = 0.0 ..;
    pub const DISP_EXPONENTIAL_RANGE: RangeFrom<f64> = 1.0 ..;
    pub const DISP_RELATIVE_RANGE: RangeInclusive<f64> = 0.0 ..= 1.0;
    }