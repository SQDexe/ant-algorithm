use {
    std::iter::repeat_with,
    crate::{
        ant::Ant,
        tech::SmartCell,
        world::World
        }
    };

pub struct AntHill {
    number_of_ants: usize,
    ants: Vec<Ant>
    }

impl AntHill {
    pub fn new(world_cell: &SmartCell<World>, number_of_ants: usize) -> Self {
        let anthill = world_cell.borrow()
            .get_anthill();

        AntHill {
            number_of_ants,
            ants: repeat_with(|| Ant::new(anthill, world_cell.clone()))
                .take(number_of_ants)
                .collect()
            }
        }

    /* Logic methods */
    pub fn action(&mut self) {
        self.ants.iter_mut()
            .filter(|ant| ! ant.is_satiated())
            .for_each(Ant::action);
        }

    pub fn reset(&mut self) {
        self.ants.iter_mut()
            .for_each(Ant::reset);
        }

    pub fn show(&self) {
        println!(
"| o>------- ants -------<o
| satiated | len | route
| ---------|-----|-------
{}| o>--------------------<o",
            self.ants.iter()
                .map(|ant|
                    format!("| {:>8} | {:>3} | {}\n",
                        ant.is_satiated(), ant.get_route_length(), ant.get_route()
                        )
                    )
                .collect::<String>()
            );
        }

    /* Getters */
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
    pub fn has_all_ants_satiated(&self) -> bool {
        self.ants.iter()
            .all(Ant::is_satiated)
        }
    }