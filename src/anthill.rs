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
    pub fn new(world: &Rc<RefCell<World>>, number_of_ants: usize, pheromone: f64) -> Self {
        let mut ants = Vec::with_capacity(number_of_ants);

        for _ in 0 .. number_of_ants {
            ants.push(Ant::new(Rc::clone(world), pheromone));
            }

        AntHill { number_of_ants, ants }
        }

    pub fn get_avrage_route_length(&self) -> f64 {
        self.ants.iter()
            .map(|e| e.route.len() as f64)
            .sum::<f64>() / self.number_of_ants as f64
        }

    pub fn get_satiated_ants(&self) -> usize {
        self.ants.iter().filter(|&e| e.satiated).count()
        }

    pub fn reset(&mut self) {
        for ant in self.ants.iter_mut() {
            ant.reset();
            }
        }

    pub fn action(&mut self) {
        for ant in self.ants.iter_mut().filter(|e| ! e.satiated) {
            ant.action();
            }
        }

    pub fn show(&self) {
        println!(
"| o>------- ants -------<o
| satiated | len | route
| ---------|-----|-------"
                );
        for ant in self.ants.iter() {
            ant.show();
            }
        println!("| o>--------------------<o");
        }
    }