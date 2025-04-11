pub struct Point {
    pub name: char,
    pub x: i32,
    pub y: i32,
    pub pheromone: f64
    }

impl Point {
    pub const fn new(name: char, x: i32, y: i32, pheromone: f64) -> Self {
        Point { name, x, y, pheromone }
        }
    }