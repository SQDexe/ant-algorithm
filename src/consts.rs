/* changed from 1e6_f64 */
pub mod value {
    pub const GREAT: f64 = f64::MAX;
    pub const MINUTE: f64 = f64::MIN_POSITIVE;
    }
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
pub mod default {
    use crate::enums::{
        Preference,
        Selection
        };

    pub const NUM_OF_CYCLES: usize = 8;
    pub const NUM_OF_ANTS: usize = 15;
    pub const NUM_OF_DECISION_POINTS: usize = 3;
    pub const PHERO_STRENGTH: f64 = 1.0;
    pub const SELECT_METHOD: Selection = Selection::Roulette;
    pub const PREFERENCE_METHOD: Preference = Preference::Compound;
    pub const RETURN_BEHAVIOUR: bool = false;
    pub const PRINT_BEHAVIOUR: bool = false;
    }

/* --------- */

/* most performant
DEFAULT_NUM_OF_DECISION_POINTS = 7
DEFAULT_PHERO_STRENGTH = 5.0
DEFAULT_NUM_OF_DECISION_POINTS = 3
DEFAULT_SELECT_METHOD = Selection::Roulette
*/