use {
    std::{
        cell::RefCell,
        rc::Rc
        },
    crate::world::World
    };

/* Ant structure, basic logical unit */
pub struct Ant {
    world_cell: Rc<RefCell<World>>,
    satiated: bool,
    route: String,
    routes_counter: usize
    }

impl Ant {
    /* Constructor */
    pub fn new(anthill_id: char, world_cell: Rc<RefCell<World>>) -> Self {
        Self {
            world_cell,
            satiated: false,
            route: anthill_id.to_string(),
            routes_counter: 0
            }
        }
    
    /* Logic methods */
    pub fn action(&mut self) {
        /* Abort if satiated */
        if self.satiated {
            return;
            }

        let ( new_position, food_reached, consume, returns ) = {
            let mut world = self.world_cell.borrow_mut();

            /* Get new position, and check if it's a foodsource */
            let new_position = world.get_new_position(&self.route);
            let food_reached = world.is_foodsource(&new_position);

            ( new_position, food_reached,
            world.do_ants_consume(), world.do_ants_return() )
            };
        
        /* Update current position, and path */
        self.route.push(new_position);
        
        /* Actions taken upon reaching a foodsource */
        if food_reached {
            /* Mark ant as satiated, and cover the route */
            let mut world_cell = self.world_cell.borrow_mut();
            self.satiated = true;
            world_cell.cover_route(&self.route);

            /* If ants consume, consume the foodsource */
            if consume {
                world_cell.consume_foodsource(new_position);
                }

            /* Early drop to avoid conflicts */
            drop(world_cell);

            /* If ants return, reset position, and increment the counter */
            if returns {
                self.reset_position();
                self.routes_counter += 1;
                }
            }
        }

    /* Reset the postion, route, and unmark the ant */
    fn reset_position(&mut self) {
        let anthill = self.world_cell.borrow()
            .get_anthill();

        self.satiated = false;
        self.route = anthill.to_string();
        }

    /* Reset the position, and the counter */
    pub fn reset(&mut self) {
        self.reset_position();
        self.routes_counter = 0;
        }

    /* Getters */
    #[inline]
    pub const fn is_satiated(&self) -> bool
        { self.satiated }
    #[inline]
    pub fn get_route(&self) -> &str
        { &self.route }
    #[inline]
    pub const fn get_route_length(&self) -> usize
        { self.route.len() }
    #[inline]
    pub const fn get_routes_count(&self) -> usize
        { self.routes_counter }
    }