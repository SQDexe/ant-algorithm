use {
    rand::random,
    crate::{
        auxil::Auxil,
        consts::{
            Selection,
            GREAT_DISTANCE
            },
        point::Point,
        utils::distance
        }
    };

pub struct World {
    pub number_of_points: usize,
    pub number_of_decision_points: usize,
    pub anthill_name: char,
    pub foodsource_name: char,
    pub points: Vec<Point>,
    pub auxils: Vec<Auxil>,
    get_index_operation: fn (&Self) -> usize
    }

impl World {
    pub fn new(method: Selection, number_of_points: usize, number_of_decision_points: usize, anthill_name: char, foodsource_name: char) -> Self {
        World {
            number_of_points,
            number_of_decision_points,
            anthill_name,
            foodsource_name,
            points: Vec::with_capacity(number_of_points),
            auxils: Vec::with_capacity(number_of_points),
            get_index_operation: match method {
                Selection::Random => Self::choose_randomly,
                Selection::Roulette => Self::roulette
                }
            }
        }
    
    pub fn init(&mut self, point_list: &[(char, i32, i32)]) {
        for (name, x, y) in point_list {
            self.points.push(Point::new(*name, *x, *y, 0.0));
            self.auxils.push(Auxil::new('\0', 0.0));
            }
        }

    /* moved from 'ant.rs' */
    fn choose_randomly(&self) -> usize {
        let nodp = self.number_of_decision_points as f64;
        (random::<f64>() * nodp).trunc() as usize
        }

    /* moved from 'ant.rs' */
    fn roulette(&self) -> usize {
        let nodp = self.number_of_decision_points;
        
        let sum: f64 = self.auxils.iter()
            .take(nodp).map(|x| x.ratio)
            .sum();

        let helper: Vec<f64> = (0 .. nodp)
            .map(|i| self.auxils[i].ratio / sum)
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
    
    pub fn get_index(&self) -> usize {
        (self.get_index_operation)(self)
        }

    pub fn reset_pheromons(&mut self) {
        for point in self.points.iter_mut() {
            point.pheromone = 0.0;
            }
        }

    /* moved from 'ant.rs' */
    pub fn reset_auxils(&mut self) {
        for (auxil, point) in self.auxils.iter_mut().zip(self.points.iter()) {
            auxil.name = point.name;
            }
        }

    /* moved from 'ant.rs' */
    pub fn sort_auxils(&mut self) {
        for (auxil, point) in self.auxils.iter_mut().zip(self.points.iter()) {
            auxil.ratio = (point.pheromone + 1.0) / auxil.ratio;
            }

        self.auxils.sort_unstable_by(|a, b| b.ratio.total_cmp(&a.ratio));
        }

    /* moved from 'ant.rs' */
    pub fn calculate_distance(&mut self, position: char) {
        let anthill = self.anthill_name;
        let Point { x, y, .. } = self.points.iter()
            .find(|&e| e.name == position)
            .expect("Unexpected position");

        for (auxil, point) in self.auxils.iter_mut().zip(self.points.iter()) {
            let name = auxil.name;
            auxil.ratio = if name == position || name == anthill {
                GREAT_DISTANCE
                } else {
				let (dx, dy) = (
                    x - point.x,
                    y - point.y
                    );
                distance(dx, dy)
                }
            }
        }

    /* moved from 'ant.rs' */
	pub fn cover_route(&mut self, pheromone: f64, visited: &str) {
		let cleared = visited.replace(self.anthill_name, "");
		
        for point in self.points.iter_mut().filter(|e| cleared.contains(e.name)) {
            point.pheromone += pheromone
            }
		}

    pub fn get_pheromones_per_point(&self) -> Vec<f64> {
        self.points.iter().map(|e| e.pheromone).collect()
        }

    pub fn show(&self) {
        println!("| o>--- world ---<o");
        for (i, point) in self.points.iter().enumerate() {
            println!("| {:>3}. {} - {}", i, point.name, point.pheromone);
            }
        println!("| o>-------------<o");
        }
    }