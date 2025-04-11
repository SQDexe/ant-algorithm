use {
	std::{
		cell::RefCell,
		rc::Rc
		},
	crate::world::World
	};


pub struct Ant {
    pub world: Rc<RefCell<World>>,
    pub position: char,
    pub pheromone: f64,
    pub satiated: bool,
    pub route: String
    }

impl Ant {
	/* moved from 'anthill.rs' */
    pub fn new(world: Rc<RefCell<World>>, pheromone: f64) -> Self {
		let position = world.borrow().anthill_name;
        Ant {
            world,
            position,
            pheromone,
            satiated: false,
            route: position.to_string()
            }
        }

    fn check_route(&self, character: char) -> bool {
        self.route.contains(character)
        }
		
	fn select_point(&self) -> usize {
		let mut world = self.world.borrow_mut();
        	
		world.reset_auxils();		
		world.calculate_distance(self.position);
		world.sort_auxils();

		let mut choice = world.get_index();
		let mut is_wrong = self.check_route(world.auxils[choice].name);
		let mut loop_prevention: usize = 0;
		
		while is_wrong {
			choice = world.get_index();
			is_wrong = self.check_route(world.auxils[choice].name);

			loop_prevention += 1;
			if 1024 <= loop_prevention {
				panic!("Got stuck in while loop!");
				}
			}
		
		choice
		}
		
	pub fn action(&mut self) {
		let index = self.select_point();
		let mut world = self.world.borrow_mut();

		let (current_position, food) = (
			world.auxils[index].name,
			world.foodsource_name
			);
		
		if current_position != food {
			self.position = current_position;
			self.route.push(self.position);
			}
		else {
			self.satiated = true;
			self.position = food;
			self.route.push(self.position);
			world.cover_route(self.pheromone, &self.route);
			}
		}

	/* moved from 'anthill.rs' */
	pub fn reset(&mut self) {
		let home = self.world.borrow()
			.anthill_name;
		self.position = home;
		self.route.push(home);
		}

	pub fn show(&self) {
		println!("| {:>8} | {:>3} | {}",
			self.satiated,
			self.route.len(),
			self.route
			);
		}
	}