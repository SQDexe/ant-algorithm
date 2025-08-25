/* Technical stuff - print error, and exit macro */
#[macro_export]
macro_rules! error_exit {
    ( $code:literal, $fstr:expr $(, $arg:expr)* ) => {
        eprintln!($fstr, $( $arg )* );
        std::process::exit($code);
        };
    }

/* Technical stuff - handy assertion macro */
#[macro_export]
macro_rules! assertion {
    ( $($boolean:expr), *) => {$(
        if ! $boolean {
            error_exit!(1, "Assertion failed on: {}", stringify!($boolean));
            }
        )*};
    }

/* Technical stuff - iterator zipping shortcut */
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

/* Technical stuff - derive Display trait for enums */
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

/* Technical stuff - alias for distance calculation function */
pub type DistanceFunction = fn (i16, i16, i16, i16) -> f64;

/* Technical stuff - type of next point selection enum */
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

derive_enum_display!(Selection, Greedy, Random, Roulette);

/* Technical stuff - ways of calculating preference for the points enum:
P - Pheromone
F - Food
D - Distance
*/
#[derive(Clone, Copy, clap::ValueEnum)]
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

/* Technical stuff - types of metrics for distance calculation enum */
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

derive_enum_display!(Metric, Chebyshev, Euclidean, Taxicab);

/* Technical stuff - types of pheromone dispersion enum */
#[derive(Clone, Copy, clap::ValueEnum)]
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

/* Technical stuff - data holder needed for constructing world's grid */
#[derive(Clone, Copy)]
pub enum PointInfo {
    Food(char, i16, i16, u32),
    Empty(char, i16, i16)
    }

impl PointInfo {
    pub const fn get_id(&self) -> char {
        match self {
            &Self::Empty(id, ..) => id,
            &Self::Food(id, ..) => id
            }
        }

    pub const fn get_position(&self) -> (i16, i16) {
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

/* Technical stuff - alias for Rust's robust if else */
pub trait BoolSelect<T> {
    fn select(&self, truthy: T, falsy: T) -> T;
    }

impl<T> BoolSelect<T> for bool {
    fn select(&self, truthy: T, falsy: T) -> T {
        if *self { truthy } else { falsy }
        }
    }

/* Technical stuff - collect data for printing */
pub trait ToDisplay: Iterator {
    fn to_display(self, sep: &str) -> String;
    }

impl<T, I: Iterator<Item = T>> ToDisplay for I
where T: ToString {
    fn to_display(self, sep: &str) -> String {
        self.map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(sep)
        }
    }