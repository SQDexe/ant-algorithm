use {
    std::{
        cell::RefCell,
        rc::Rc
        },
    crate::{
        ant::Ant,
        world::World
        }
    };

pub struct AntHill {
    number_of_ants: usize,
    ants: Vec<Ant>
    }

impl AntHill {
    pub fn new(world: &Rc<RefCell<World>>, number_of_ants: usize, pheromone: f64, returns: bool) -> Self {
        AntHill {
            number_of_ants,
            ants: (0 .. number_of_ants)
                .map(|_| Ant::new(Rc::clone(world), pheromone, returns))
                .collect()
            }
        }

    pub fn reset(&mut self) {
        self.ants.iter_mut()
            .for_each(Ant::reset);
        }

    pub fn action(&mut self) {
        self.ants.iter_mut()
            .filter(|ant| ! ant.is_satiated())
            .for_each(Ant::action);
        }

    pub fn show(&self) {
        println!(
"| o>------- ants -------<o
| satiated | len | route
| ---------|-----|-------"
                );

        self.ants.iter()
            .for_each(|ant|
                println!("| {:>8} | {:>3} | {}",
                    ant.is_satiated(),
                    ant.get_route_length(),
                    ant.get_route()
                    )
                );

        println!("| o>--------------------<o");
        }

    pub fn get_average_route_length(&self) -> f64 {
        self.ants.iter()
            .map(|ant| ant.get_route_length() as f64)
            .sum::<f64>() / self.number_of_ants as f64
        }

    pub fn get_average_routes_count(&self) -> f64 {
        self.ants.iter()
            .map(|ant| ant.get_routes_count() as f64)
            .sum::<f64>() / self.number_of_ants as f64
        }

    pub fn get_satiated_ants_count(&self) -> usize {
        self.ants.iter()
            .filter(|&ant| ant.is_satiated())
            .count()
        }
    }