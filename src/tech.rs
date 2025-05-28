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

/* Handy assertion macro */
#[macro_export]
macro_rules! assertion {
    ( $($boolean:expr), *) => {$(
        if ! $boolean {
            eprintln!("Assertion failed on: {}", stringify!($boolean));
            exit(1);
            }
        )*};
    }

/* Shorthand for distance calculation function */
pub type DistanceFunction = fn (i32, i32, i32, i32) -> f64;

/* Ways of choosing next point */
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Selection {
    Greedy,
    Random,
    Roulette
    }

/* Ways of calculating preference for the points:
P - Pheromone
F - Food
D - Distance
*/
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Preference {
    Distance,
    Pheromone,
    Food,
    PD,
    FD,
    PF,
    PFD
    }

/* Types of metrics for distance calculation */
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Metric {
    Chebyshev,
    Euclidean,
    Taxicab
    }

/* Types of pheromone dispersion */
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Dispersion {
    Exponential,
    Linear,
    Relative
    }

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