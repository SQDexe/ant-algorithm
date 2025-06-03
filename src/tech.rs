use {
    clap::ValueEnum,
    std::{
        cell::{
            Ref,
            RefCell,
            RefMut
            },
        rc::Rc
        }
    };

/* Print error, and exit macro */
#[macro_export]
macro_rules! error_exit {
    ( $code:literal, $fstr:expr $(, $arg:expr)* ) => {
        eprintln!($fstr, $( $arg )* );
        std::process::exit($code);
        };
    }

/* Handy assertion macro */
#[macro_export]
macro_rules! assertion {
    ( $($boolean:expr), *) => {$(
        if ! $boolean {
            error_exit!(1, "Assertion failed on: {}", stringify!($boolean));
            }
        )*};
    }

/* Iterator zipping shortcut */
#[macro_export]
macro_rules! zip {
    ( mut $iter1:expr, $iter2:expr ) => {
        $iter1.iter_mut()
            .zip($iter2.iter())
        };
    ( $iter1:expr, $iter2:expr ) => {
        $iter1.iter()
            .zip($iter2.iter())
        };
    ( mut $iter1:expr, mut $iter2:expr ) => {
        $iter1.iter_mut()
            .zip($iter2.iter_mut())
        };
    }

/* Derive Display trait for enums */
macro_rules! derive_enum_display {
    ( $enum:ident, $( $item:ident ),+ ) => {
        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $( Self::$item => stringify!($item), )+
                    })
                }
            }
        };
    ( $enum:ident, $( $key:ident = $value:literal ),+ ) => {
        impl std::fmt::Display for $enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    $( Self::$key => $value, )+
                    })
                }
            }
        };
    }

/* Shorthand for distance calculation function */
pub type DistanceFunction = fn (i32, i32, i32, i32) -> f64;

/* Ways of choosing next point */
#[derive(Clone, Copy, ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

derive_enum_display!(Selection, Greedy, Random, Roulette);

/* Ways of calculating preference for the points:
P - Pheromone
F - Food
D - Distance
*/
#[derive(Clone, Copy, ValueEnum)]
pub enum Preference {
    Distance,
    Pheromone,
    Food,
    PD,
    FD,
    PF,
    PFD
    }

derive_enum_display!(Preference, Distance, Pheromone, Food, PD, FD, PF, PFD);

/* Types of metrics for distance calculation */
#[derive(Clone, Copy, ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

derive_enum_display!(Metric, Chebyshev, Euclidean, Taxicab);

/* Types of pheromone dispersion */
#[derive(Clone, Copy, ValueEnum)]
pub enum Dispersion {
    None,
    Linear,
    Exponential,
    Relative
    }

impl Dispersion {
    pub const fn is_set(&self) -> bool {
        match self {
            Dispersion::None => false,
            _ => true
            }
        }
    }

derive_enum_display!(Dispersion, None, Linear, Exponential, Relative);

/* Holds data needed for constructing world's grid */
#[derive(Clone, Copy)]
pub enum PointInfo {
    Food(char, i32, i32, u32),
    Empty(char, i32, i32)
    }

impl PointInfo {
    pub const fn get_id(&self) -> char {
        match self {
            &Self::Empty(id, ..) => id,
            &Self::Food(id, ..) => id
            }
        }

    pub const fn get_position(&self) -> (i32, i32) {
        match self {
            &Self::Empty(_, x, y) => (x, y),
            &Self::Food(_, x, y, _) => (x, y)
            }
        }

    pub const fn has_food(&self) -> bool {
        match self {
            &Self::Empty(..) => false,
            &Self::Food(.. , 0) => false,
            _ => true
            }
        }

    pub const fn get_food(&self) -> u32 {
        match self {
            &Self::Food(.. , amount) => amount,
            _ => 0   
            }
        }
    }

/* Shortcut for Rust's robust if else */
pub trait BoolSelect<T> {
    fn select(&self, truthy: T, falsy: T) -> T;
    }

impl<T> BoolSelect<T> for bool {
    fn select(&self, truthy: T, falsy: T) -> T {
        if *self { truthy } else { falsy }
        }
    }

/* Collect data for printing */
pub trait ToDisplay {
    fn to_display(&self, sep: &str) -> String;
    }

impl<T, U> ToDisplay for T
where
T: IntoIterator<Item = U> + Clone,
U: ToString {
    fn to_display(&self, sep: &str) -> String {
        self.clone()
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(sep)
        }
    }

/* name ideas:
SmartPointer<T>
RcRef<T>
SmartCell<T>
RefCountedCell<T>
CountedMut<T>
ChillCell<T>
Cello<T>
*/

/* Technical stuff */
pub struct SmartCell<T> (
    Rc<RefCell<T>>
    );

impl<T> SmartCell<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
        }

    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
        }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
        }

    pub fn clone(&self) -> Self {
        SmartCell(Rc::clone(&self.0))
        }
    }