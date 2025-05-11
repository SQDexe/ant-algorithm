use {
    std::{
        cell::RefCell,
        rc::Rc
        },
    crate::world::World
    };


pub struct Ant {
    world: Rc<RefCell<World>>,
    position: char,
    satiated: bool,
    route: String,
    routes_counter: usize
    }

impl Ant {
    /* moved from 'anthill.rs' */
    pub fn new(world: Rc<RefCell<World>>) -> Self {
        let position = world.borrow()
            .get_anthill();
    
        Ant {
            world,
            position,
            satiated: false,
            route: position.to_string(),
            routes_counter: 0
            }
        }

    fn check_route(&self, character: char) -> bool {
        self.route.contains(character)
        }
        
    fn select_point(&self) -> usize {
        let mut world = self.world.borrow_mut();
    
        world.calculate_preference(self.position, &self.route);

        let mut choice = world.get_index();
        // let mut loop_prevention: usize = 0;

        while self.check_route(world.get_auxils()[choice].name) {
            choice = world.get_index();

            // loop_prevention += 1;
            // if 1024 <= loop_prevention {
            //     panic!("Got stuck in while loop!");
            //     }
            }
        
        choice
        }
        
    pub fn action(&mut self) {
        let index = self.select_point();

        let ( current_position, food, returns ) = {
            let world = self.world.borrow();
            ( world.get_auxils()[index].name, world.get_foodsource(), world.get_ants_return() )
            };
        
        self.position = current_position;
        self.route.push(self.position);
        
        if current_position == food {
            self.satiated = true;
            self.world.borrow_mut()
                .cover_route(&self.route);

            if returns {
                self.reset();
                self.routes_counter += 1;
                }
            }
        }

    /* moved from 'anthill.rs' */
    pub fn reset(&mut self) {
        let anthill = self.world.borrow()
            .get_anthill();

        self.satiated = false;
        self.position = anthill;
        self.route = anthill.to_string();
        }

    pub fn is_satiated(&self) -> bool {
        self.satiated
        }

    pub fn get_route(&self) -> &str {
        &self.route
        }

    pub fn get_route_length(&self) -> usize {
        self.route.len()
        }

    pub fn get_routes_count(&self) -> usize {
        self.routes_counter
        }
    }