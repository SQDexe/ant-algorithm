use {
    rand::{
        random,
        random_range
        },
    crate::{
        auxil::Auxil, consts::{
            value::{
                MINUTE,
                UNKOWN
                },
            PHERO_CALC_BIAS
            },
        enums::{
            Dispersion,
            Preference,
            Selection
            },
        point::Point,
        utils::euclidean_distance
        }
    };

pub struct World {
    number_of_decision_points: usize,
    ants_return: bool,
    pheromone: f64,
    factor: f64,
    anthill_name: char,
    foodsource_name: char,
    points: Vec<Point>,
    auxils: Vec<Auxil>,
    disperse_operation: fn (&Point, f64) -> f64,
    get_index_operation: fn (&Self) -> usize,
    preference_operation: fn (&Point, i32, i32) -> f64
    }

impl World {
    pub fn new(
    point_list: &[(char, i32, i32)],
    number_of_decision_points: usize,
    ants_return: bool,
    pheromone: f64,
    factor: Option<f64>,
    dispersion_method: Option<Dispersion>,
    select_method: Selection,
    point_preference: Preference
    ) -> Self {

        let [(anthill_name, ..), .., (foodsource_name, ..)] = *point_list else {
            unreachable!()
            };
        
        let (points, auxils) = point_list.iter()
            .map(|&(name, x, y)| (
                Point::new(name, x, y, 0.0),
                Auxil::new('\0', UNKOWN)
                ))
            .unzip();

        World {
            number_of_decision_points,
            ants_return,
            /* moved from 'ant.rs' */
            pheromone,
            factor: match factor {
                Some(0.0) =>
                    panic!("Unsupported FACTOR value"),
                Some(value) => value,
                _ => UNKOWN
                },
            anthill_name,
            foodsource_name,
            points,
            auxils,
            disperse_operation: match dispersion_method {
                Some(Dispersion::Linear) => |point, factor|
                    point.pheromone - factor,
                Some(Dispersion::Exponential) => |point, factor|
                    point.pheromone / factor,
                Some(Dispersion::Relative) => |point, factor|
                    point.pheromone * (1.0 - factor),
                _ => |_, _| UNKOWN
                },
            get_index_operation: match select_method {
                Selection::Random => Self::randomly,
                Selection::Roulette => Self::roulette,
                Selection::Greedy => Self::greedy
                },
            preference_operation: match point_preference {
                Preference::Compound => |point, x, y|
                    (point.pheromone + PHERO_CALC_BIAS) / euclidean_distance(x - point.x, y - point.y),
                Preference::Distance => |point, x, y|
                    PHERO_CALC_BIAS / euclidean_distance(x - point.x, y - point.y),
                Preference::Pheromone => |point, _, _|
                    point.pheromone + PHERO_CALC_BIAS
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

    fn set_pheromones(&mut self, func: fn(&Point, f64) -> f64) {
        self.points.iter_mut()
            .for_each(|point|
                point.pheromone = func(point, self.factor).max(0.0)
                );
        }

    /* moved from 'ant.rs' */
    pub fn calculate_preference(&mut self, position: char, visited: &str) {
        self.reset_auxils();

        let Point { x, y, .. } = *self.points.iter()
            .find(|point| point.name == position)
            .unwrap();

        self.auxils.iter_mut().zip(self.points.iter())
            .for_each(|(auxil, point)| {
                let name = auxil.name;
                auxil.ratio = if name == position || visited.contains(name) {
                    MINUTE
                } else {
                    (self.preference_operation)(&point, x, y)
                    }
                });

        self.sort_auxils();
        }

    /* moved from 'ant.rs' */
    pub fn cover_route(&mut self, visited: &str) {
        let cleared = visited.replace(self.anthill_name, "");

        self.points.iter_mut()
            .filter(|point| cleared.contains(point.name))
            .for_each(|point|
                point.pheromone += self.pheromone
                );
        }

    // pub fn reset_pheromons(&mut self) {
    //     self.set_pheromones(|_, _| 0.0);
    //     }

    pub fn disperse_pheromons(&mut self) {
        self.set_pheromones(self.disperse_operation);
        }

    pub fn show(&self) {
        println!("| o>--- world ---<o");

        print!("{}",
            self.points.iter().enumerate()
                .map(|(i, point)|
                    format!("| {:>3}. {} - {}\n",
                        i, point.name, point.pheromone
                        )
                    )
                .collect::<String>()
            );

        println!("| o>-------------<o");
        }

    pub fn get_anthill(&self) -> char {
        self.anthill_name
        }
    
    pub fn get_foodsource(&self) -> char {
        self.foodsource_name
        }

    pub fn get_ants_return(&self) -> bool {
        self.ants_return
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