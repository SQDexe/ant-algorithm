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

/* Default settings */
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
    }

/*  */
pub mod bias {
    pub const NEUTRAL: f64 = 1.0;
    /* changed from 1e6_f64 */
    pub const GREAT: f64 = f64::MAX;
    /* changed from 1e-6_f64 */
    pub const MINUTE: f64 = f64::MIN_POSITIVE;
    }

/* Technical stuff */
pub mod tips {
    pub const CYCLES: &str = "Sets number of cycles\n";
    pub const ANTS: &str = "Sets number of ants\n";
    pub const PHEROMONE: &str = "Sets the strength of pheromones\n";
    pub const DECISION: &str = "Sets the number of decision points\n";
    pub const RATE: &str = "Sets whether, and how much food is consumed\n";
    pub const RETURNS: &str = "Sets whether ants return to the anthill";
    pub const SELECT: &str = "Sets how points are selected\n";
    pub const PREFERENCE: &str = "Sets how the point preference is calculated\n";
    pub const METRIC: &str = "Sets how the distance between points is calculated\n";
    pub const DISPERSION: &str = "Sets the dispersion mode\n";
    pub const FACTOR: &str = "Sets the coefficient of the dispersion";
    pub const ACTIONS: &str =
"Sets food at existing points during runtime,
format 'cycle,id,amount'";
    pub const GRID: &str =
"Sets new world grid,
must contain at least 2 points,
the first point is automatically chosen as anthill,
format 'id,x,y[,food]'";
    pub const QUIET: &str = "Run program in quite mode";
    pub const OUTPUT: &str =
"A file to write statistics to in JSON format,
will create, or append/truncate existing file,
searches from current working directory\n";
    }