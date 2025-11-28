use {
    anyhow::{
        anyhow,
        Error
        },
    clap::ValueEnum,
    derive_more::Display,
    core::str::FromStr
    };

/* Technical stuff - alias for distance calculation function */
pub type DistanceFunction = fn (i16, i16, i16, i16) -> f64;

/* Technical stuff - type of next point selection enum */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

/* Technical stuff - ways of calculating preference for the points enum:
P - Pheromone
F - Food
D - Distance
*/
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Preference {
    Distance,
    Pheromone,
    Food,
    PD,
    FD,
    PF,
    PFD
    }

/* Technical stuff - types of metrics for distance calculation enum */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

/* Technical stuff - types of pheromone dispersion enum */
#[derive(Clone, Copy, Display, ValueEnum)]
pub enum Dispersion {
    Linear,
    Exponential,
    Relative
    }

/* Technical stuff - aliases of some types */
#[derive(Clone)]
pub struct Action ( usize, char, u32 );

impl Action {
    #[inline]
    pub const fn get_values(&self) -> (usize, char, u32) {
        let &Action(cycle, id, amount) = self;
        (cycle, id, amount)
        }
    }

impl FromStr for Action {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Box<[_]> = s.split(',').collect();
        match parts[..] {
            [cycle, id, food] => Ok(Action(
                cycle.parse()?,
                id.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow!("Action parsing failed"))
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

impl FromStr for PointInfo {
    type Err = Error;

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
            _ => Err(anyhow!("PointInfo parsing failed"))
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