use {
    std::{
        cell::RefCell,
        iter::repeat_with,
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
    pub fn new(world: &Rc<RefCell<World>>, number_of_ants: usize) -> Self {
        AntHill {
            number_of_ants,
            ants: repeat_with(|| Ant::new(Rc::clone(world)))
                .take(number_of_ants)
                .collect()
            }
        }

    // pub fn reset(&mut self) {
    //     self.ants.iter_mut()
    //         .for_each(Ant::reset);
    //     }

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

        print!("{}",
            self.ants.iter()
                .map(|ant|
                    format!("| {:>8} | {:>3} | {}\n",
                        ant.is_satiated(), ant.get_route_length(), ant.get_route()
                        )
                    )
                .collect::<String>()
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
            .filter(|ant| ant.is_satiated())
            .count()
        }

    pub fn get_all_ants_satiated(&self) -> bool {
        self.ants.iter()
            .all(Ant::is_satiated)
        }
    }