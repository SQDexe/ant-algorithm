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

/* Technical stuff - shorthand for robust if-else, or match bool blocks */
#[macro_export]
macro_rules! select {
    ($bool:expr, $truthy:expr, $falsy:expr) => {
        match $bool {
            true => $truthy,
            false => $falsy
            }
        };
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
        impl core::fmt::Display for $enum {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", match self {
                    $( Self::$item => stringify!($item), )+
                    })
                }
            }
        };
    ( $enum:ident, $( $key:ident = $value:literal ),+ ) => {
        impl core::fmt::Display for $enum {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    Linear,
    Exponential,
    Relative
    }

derive_enum_display!(Dispersion, Linear, Exponential, Relative);

/* Technical stuff - aliases of some types */
#[derive(Clone)]
pub struct Action (
    usize,
    char,
    u32
    );

impl Action {
    #[inline]
    pub const fn get_values(&self) -> (usize, char, u32) {
        let &Action(cycle, id, amount) = self;
        (cycle, id, amount)
        }
    }

impl core::str::FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Box<[_]> = s.split(',').collect();
        match parts[..] {
            [cycle, id, food] => Ok(Action(
                cycle.parse()?,
                id.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow::anyhow!("Action parsing failed"))
            }
        }
    }

/* Technical stuff - data holder needed for constructing world's grid */
#[derive(Clone, Copy)]
pub enum PointInfo {
    Food(char, i16, i16, u32),
    Empty(char, i16, i16)
    }

impl PointInfo {
    #[inline]
    pub const fn get_id(&self) -> char {
        match self {
            &Self::Empty(id, ..) => id,
            &Self::Food(id, ..) => id
            }
        }

    #[inline]
    pub const fn get_position(&self) -> (i16, i16) {
        match self {
            &Self::Empty(_, x, y) => (x, y),
            &Self::Food(_, x, y, _) => (x, y)
            }
        }

    #[inline]
    pub const fn has_food(&self) -> bool {
        match self {
            &Self::Empty(..) => false,
            &Self::Food(.. , 0) => false,
            _ => true
            }
        }

    #[inline]
    pub const fn get_food(&self) -> u32 {
        match self {
            &Self::Food(.. , amount) => amount,
            _ => 0   
            }
        }
    }

impl core::str::FromStr for PointInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Box<[_]> = s.split(',').collect();
        match parts[..] {
            [id, x, y] => Ok(PointInfo::Empty(
                id.parse()?,
                x.parse()?,
                y.parse()?
                )),
            [id, x, y, food] => Ok(PointInfo::Food(
                id.parse()?,
                x.parse()?,
                y.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow::anyhow!("PointInfo parsing failed"))
            }
        }
    }

/* Technical stuff - collect data for printing */
pub trait ToDisplay: Iterator {
    fn to_display(self, sep: &str) -> String;
    }

impl<T, I> ToDisplay for I
where T: ToString, I: Iterator<Item = T> {
    fn to_display(self, sep: &str) -> String {
        self.map(|e| e.to_string())
            .collect::<Box<[_]>>()
            .join(sep)
        }
    }

/* Technical stuff - pretty prinitng an Option */
pub trait DisplayOption {
    fn display_option(&self) -> String;
    }

impl<T> DisplayOption for Option<T>
where T: core::fmt::Display {

    fn display_option(&self) -> String {
        match self {
            Some(value) => format!("{value}"),
            None => String::from("None")
            }
        }
    }