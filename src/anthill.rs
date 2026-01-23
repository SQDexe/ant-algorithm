use {
    core::iter::repeat_with,
    crate::{
        tech::Config,
        utils::Ant,
        world::World
        }
    };



/** `Anthill` strucutre, for handling `Ant`'s operations. */
pub struct AntHill {
    /** Number of ants in the colony. */
    num_of_ants: usize,
    /** Container for the ants. */
    ants: Box<[Ant]>,
    /** Anthill's ID. */
    anthill_id: char,
    /** Amount of pheromones laid out by ants. */
    pheromone: f64,
    /** Amount of food ants cosume. */
    consume_rate: u32,
    /** Whether ants return to the colony after finishing their routes. */
    do_return: bool
    }

impl AntHill {
    /** Constructor. */
    pub fn new(anthill_id: char, config: &Config, num_of_points: usize) -> Self {
        let ants = repeat_with(|| Ant::new(anthill_id, num_of_points))
            .take(config.ants)
            .collect();

        /* Create anthill */
        Self {
            num_of_ants: config.ants,
            ants,
            anthill_id,
            pheromone: config.pheromone,
            consume_rate: config.rate,
            do_return: config.returns
            }
        }

    /** Make all unsatiated ants take action. */
    pub fn action(&mut self, world: &mut World) {
        /* Precalculate condition */
        let do_ants_cosume = self.consume_rate != 0;

        /* Iter over unsatiated ants */
        let iter = self.ants.iter_mut()
            .filter(|ant| ! ant.satiated);
        for ant in iter {
            /* Get new position, and check if it's a foodsource */
            let new_position = world.get_new_position(&ant.route);
            let food_reached = world.is_foodsource(&new_position);
        
            /* Update current position, and path */
            ant.route.push(new_position);
        
            /* Actions taken upon reaching a foodsource */
            if food_reached {
                /* Mark ant as satiated, and cover the route */
                ant.satiated = true;
                world.cover_route(&ant.route, &[self.anthill_id], self.pheromone);

                /* If ants consume, consume the foodsource */
                if do_ants_cosume {
                    world.consume_foodsource(new_position, self.consume_rate);
                    }

                /* If ants return, reset position, and increment the counter */
                if self.do_return {
                    ant.return_to(self.anthill_id);
                    ant.routes_counter += 1;
                    }
                }
            }
        }

    /** Reset all ants. */
    pub fn reset(&mut self) {
        for ant in self.ants.iter_mut() {
            ant.reset(self.anthill_id);
            }
        }

    /** Show a table of states of all ants. */
    pub fn show(&self) {
        let tmp: String = self.ants.iter()
            .map(|Ant { satiated, route, .. }|
                format!("| {:>8} | {:>3} | {}\n",
                    satiated, route.len(), route
                    )
                )
            .collect();

        println!(
"| o>------- ants -------<o
| satiated | len | route
| ---------|-----|-------
{tmp}| o>--------------------<o"
            );
        }

    /** `average_route_length` getter. */
    pub fn get_average_route_length(&self) -> f64 {
        self.ants.iter()
            .map(|ant| ant.route.len() as f64)
            .sum::<f64>() /
            self.num_of_ants as f64
        }
    /** `average_routes_count` getter. */
    pub fn get_average_routes_count(&self) -> f64 {
        self.ants.iter()
            .map(|ant| ant.routes_counter as f64)
            .sum::<f64>() /
            self.num_of_ants as f64
        }
    /** `satiated_ants_count` getter. */
    pub fn get_satiated_ants_count(&self) -> usize {
        self.ants.iter()
            .filter(|ant| ant.satiated)
            .count()
        }
    /** Ant's `satiated` checker. */
    pub fn has_all_ants_satiated(&self) -> bool {
        self.ants.iter()
            .all(|ant| ant.satiated)
        }
    }