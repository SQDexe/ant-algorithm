use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Selection {
    Random,
    Roulette
    }

/* changed from 1e6_f64 */
pub const GREAT_DISTANCE: f64 = f64::MAX;
pub const NUM_OF_POINTS: usize = 7;
pub const TABLE: [(char, i32, i32); NUM_OF_POINTS] = [
    ('a', 6, 1),
    ('b', 13, 1),
    ('c', 4, 3),
    ('d', 4, 5),
    ('e', 8, 5),
    ('f', 6, 8),
    ('g', 10, 8)
    ];

/* default settings */
pub const DEFAULT_SELECT_METHOD: Selection = Selection::Roulette;
pub const DEFAULT_NUM_OF_ANTS: usize = 15;
pub const DEFAULT_PHERO_STRENGTH: f64 = 1.0;
pub const DEFAULT_NUM_OF_DECISION_POINTS: usize = 3;
pub const DEFAULT_NUM_OF_CYCLES: usize = 8;
/* --------- */