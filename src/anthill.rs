use {
    std::rc::Rc,
    core::{
        iter::repeat_with,
        cell::RefCell
        },
    crate::{
        ant::Ant,
        world::World
        }
    };

/** `Anthill` strucutre, for handling `Ant`'s operations. */
pub struct AntHill {
    /** Number of ants in the collony. */
    number_of_ants: usize,
    /** Container for the ants. */
    ants: Box<[Ant]>
    }

impl AntHill {
    /** Constructor. */
    pub fn new(world_cell: &Rc<RefCell<World>>, number_of_ants: usize) -> Self {
        let anthill = world_cell.borrow()
            .get_anthill();

        Self {
            number_of_ants,
            ants: repeat_with(|| Ant::new(anthill, Rc::clone(&world_cell)))
                .take(number_of_ants)
                .collect()
            }
        }

    /** Make all unsatiated ants take action. */
    pub fn action(&mut self) {
        self.ants.iter_mut()
            .filter(|ant| ! ant.is_satiated())
            .for_each(Ant::action);
        }

    /** Reset all ants. */
    pub fn reset(&mut self) {
        self.ants.iter_mut()
            .for_each(Ant::reset);
        }

    /** Show a table of states of all ants. */
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

    /** `average_route_length` getter. */
    pub fn get_average_route_length(&self) -> f64 {
        self.ants.iter()
            .map(|ant| ant.get_route_length() as f64)
            .sum::<f64>() /
            self.number_of_ants as f64
        }
    /** `average_routes_count` getter. */
    pub fn get_average_routes_count(&self) -> f64 {
        self.ants.iter()
            .map(|ant| ant.get_routes_count() as f64)
            .sum::<f64>() /
            self.number_of_ants as f64
        }
    /** `satiated_ants_count` getter. */
    pub fn get_satiated_ants_count(&self) -> usize {
        self.ants.iter()
            .filter(|ant| ant.is_satiated())
            .count()
        }
    /** Ant's `satiated` checker. */
    pub fn has_all_ants_satiated(&self) -> bool {
        self.ants.iter()
            .all(Ant::is_satiated)
        }
    }