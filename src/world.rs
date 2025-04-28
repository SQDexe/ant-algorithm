use {
    rand::{
        random,
        random_range
        },
    crate::{
        auxil::Auxil,
        enums::{
            Preference,
            Selection
            },
        consts::value,
        point::Point,
        utils::euclidean_distance
        }
    };

pub struct World {
    number_of_decision_points: usize,
    anthill_name: char,
    foodsource_name: char,
    points: Vec<Point>,
    auxils: Vec<Auxil>,
    get_index_operation: fn (&Self) -> usize,
    calculate_distance_operation: fn (&Point, i32, i32) -> f64
    }

impl World {
    pub fn new(point_list: &[(char, i32, i32)], number_of_decision_points: usize, anthill_name: char, foodsource_name: char, select_method: Selection, point_preference: Preference) -> Self {
        let (mut points, mut auxils) = {
            let nop = point_list.len();
            (Vec::with_capacity(nop), Vec::with_capacity(nop))
            };

        point_list.iter()
            .for_each(|&(name, x, y)| {
                points.push(Point::new(name, x, y, 0.0));
                auxils.push(Auxil::new('\0', 0.0));
                });

        World {
            number_of_decision_points,
            anthill_name,
            foodsource_name,
            points,
            auxils,
            get_index_operation: match select_method {
                Selection::Random => Self::randomly,
                Selection::Roulette => Self::roulette,
                Selection::Greedy => Self::greedy
                },
            calculate_distance_operation: match point_preference {
                Preference::Compound => Self::compound,
                Preference::Distance => Self::distance,
                Preference::Pheromone => Self::pheromone
                }
            }
        }

    /* moved from 'ant.rs' */
    fn randomly(&self) -> usize {
        random_range(0 .. self.number_of_decision_points)
        }

    /* moved from 'ant.rs' */
    fn roulette(&self) -> usize {
        let nodp = self.number_of_decision_points;
        
        let sum: f64 = self.auxils.iter()
            .take(nodp)
            .map(|auxil| auxil.ratio)
            .sum();

        let helper: Vec<f64> = self.auxils.iter()
            .take(nodp)
            .map(|auxil| auxil.ratio / sum)
            .collect();

        let mut index = 0;
        let mut rest = helper[index];
        let chance: f64 = random();
        while rest < chance {
            index += 1;
            rest += helper[index];
            }

        index
        }

    const fn greedy(&self) -> usize {
        0
        }

    fn compound(point: &Point, x: i32, y: i32) -> f64 {
        (point.pheromone + 1.0) / euclidean_distance(x - point.x, y - point.y)
        }
    
    fn distance(point: &Point, x: i32, y: i32) -> f64 {
        1.0 / euclidean_distance(x - point.x, y - point.y)
        }

    fn pheromone(point: &Point, _: i32, _: i32) -> f64 {
        point.pheromone + 1.0
        }

    // pub fn reset_pheromons(&mut self) {
    //     self.points.iter_mut()
    //         .for_each(|point|
    //             point.pheromone = 0.0
    //             );
    //     }

    /* moved from 'ant.rs' */
    fn reset_auxils(&mut self) {
        self.auxils.iter_mut().zip(self.points.iter())
            .for_each(|(auxil, point)|
                auxil.name = point.name
                );
        }

    /* moved from 'ant.rs' */
    fn sort_auxils(&mut self) {
        self.auxils.sort_unstable_by(|a, b|
            b.ratio.total_cmp(&a.ratio)
            );
        }

    /* moved from 'ant.rs' */
    pub fn calculate_distance(&mut self, position: char, route: &str) {
        self.reset_auxils();

        let Point { x, y, .. } = self.points.iter()
            .find(|&point| point.name == position)
            .unwrap();

        self.auxils.iter_mut().zip(self.points.iter())
            .for_each(|(auxil, point)| {
                let name = auxil.name;
                auxil.ratio = if name == position || route.contains(name) {
                    value::MINUTE
                } else {
                    (self.calculate_distance_operation)(&point, *x, *y)
                    }
                });

        self.sort_auxils();
        }

    /* moved from 'ant.rs' */
	pub fn cover_route(&mut self, pheromone: f64, visited: &str) {
		let cleared = visited.replace(self.anthill_name, "");

        self.points.iter_mut()
            .filter(|point| cleared.contains(point.name))
            .for_each(|point|
                point.pheromone += pheromone
                );
		}

    pub fn show(&self) {
        println!("| o>--- world ---<o");

        self.points.iter().enumerate()
            .for_each(|(i, point)|
                println!("| {:>3}. {} - {}", i, point.name, point.pheromone
                ));

        println!("| o>-------------<o");
        }

    pub fn get_anthill(&self) -> char {
        self.anthill_name
        }
    
    pub fn get_foodsource(&self) -> char {
        self.foodsource_name
        }

    pub fn get_auxils(&self) -> &[Auxil] {
        &self.auxils
        }

    pub fn get_pheromones_per_point(&self) -> Vec<f64> {
        self.points.iter()
            .map(|point| point.pheromone)
            .collect()
        }
    
    pub fn get_index(&self) -> usize {
        (self.get_index_operation)(self)
        }
    }