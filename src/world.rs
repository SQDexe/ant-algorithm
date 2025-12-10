use {
    fastrand::{
        f64 as random_f64,
        usize as random_usize
        },
    sqds_tools::select,
    std::{
        collections::HashSet,
        process::exit
        },
    crate::{
        consts::bias,
        tech::{
            Config,
            DistanceFunction,
            PointInfo,
            Dispersion,
            Metric,
            Preference,
            Selection
            },
        utils::{
            disperse,
            distance,
            preference,
            Auxil,
            Point
            }
        }
    };

/* World structure, for handling most of logic operations, and managing the grid */
pub struct World {
    points: Box<[Point]>,
    auxils: Box<[Auxil]>,
    anthill_id: char,
    foodsource_ids: HashSet<char>,
    number_of_decision_points: usize,
    pheromone: f64,
    consume_rate: u32,
    ants_return: bool,
    get_index_operation: fn (&Self) -> usize,
    preference_operation: fn (&Point, i16, i16, DistanceFunction) -> f64,
    distance_operation: DistanceFunction,
    disperse_operation: fn (&Point, f64) -> f64,
    factor: f64
    }

impl World {
    /* Get builder object */
    pub fn new(point_list: Vec<PointInfo>, config: &Config) -> Option<Self> {
        let anthill_id = point_list.first()?
            .get_id();

        let foodsource_ids = point_list.iter()
            .filter_map(|point_info|
                point_info.has_food()
                    .then(|| point_info.get_id())
                )
            .collect();

        let (points, auxils): (Vec<_>, Vec<_>) = point_list.into_iter()
            .map(|point_info| (
                Point::from(point_info),
                Auxil::new('\0', bias::UNKOWN)
                ))
            .unzip();

        let world = World {
            points: points.into_boxed_slice(),
            auxils: auxils.into_boxed_slice(),
            anthill_id,
            foodsource_ids,
            number_of_decision_points: config.decision,
            pheromone: config.pheromone,
            ants_return: config.returns,
            consume_rate: config.rate,
            get_index_operation: match config.select {
                Selection::Random => World::select_randomly,
                Selection::Roulette => World::select_roulette,
                Selection::Greedy => World::select_greedy
                },
            preference_operation: match config.preference {
                Preference::Distance => preference::distance,
                Preference::Pheromone => preference::pheromone,
                Preference::Food => preference::food,
                Preference::PD => preference::phero_dist,
                Preference::FD => preference::food_dist,
                Preference::PF => preference::phero_food,
                Preference::PFD => preference::phero_food_dist
                },
            distance_operation: match config.metric {
                Metric::Chebyshev => distance::chebyshev,
                Metric::Euclidean => distance::euclidean,
                Metric::Taxicab => distance::taxicab
                },
            disperse_operation: match config.dispersion {
                Some(Dispersion::Linear) => disperse::linear,
                Some(Dispersion::Exponential) => disperse::exponential,
                Some(Dispersion::Relative) => disperse::relative,
                _ => |_, _| bias::UNKOWN
                },
            factor: config.factor
            };

        Some(world)
        }

    /* Selection methods */
    #[inline]
    const fn select_greedy(&self) -> usize
        { 0 }
    fn select_randomly(&self) -> usize {
        random_usize(0 .. self.number_of_decision_points)
        }
    fn select_roulette(&self) -> usize {
        /* Get helper array */
        let wheel: Vec<f64> = {
            let iter = self.auxils.iter()
                .take(self.number_of_decision_points)
                .map(|Auxil { ratio, ..}| ratio);

            /* Sum ratios within range */
            let sum: f64 = iter.clone()
                .sum();

            /* Collect helper array into roulette wheel */
            iter.map(|ratio| ratio / sum)
                .collect()
            };

        /* First choice from the wheel */
        let mut index = 0;
        let mut rest = wheel[index];
        
        /* Select random chance */
        let chance = random_f64();

        /* Spin the wheel until it stops */
        while rest < chance {
            index += 1;
            rest += wheel[index];
            }

        index
        }

    /* Reset auxils in sync with points - the ratios are overwritten each time */
    fn reset_auxils(&mut self) {
        for (auxil, point) in self.auxils.iter_mut()
        .zip(self.points.iter()) {
            auxil.id = point.id
            }
        }

    /* Sort auxils from biggest to smallest */
    fn sort_auxils(&mut self) {
        self.auxils.sort_unstable_by(|a, b|
            b.ratio.total_cmp(&a.ratio)
            );
        }

    /* Set pheromones according to passed function */
    fn set_pheromones(&mut self, func: fn (&Point, f64) -> f64) {
        for point in &mut self.points {
            point.pheromone = func(point, self.factor).max(0.0)
            };
        }

    /* Create new position according to passed arguments */
    pub fn get_new_position(&mut self, visited: &str) -> char {
        /* Clear the helper array */
        self.reset_auxils();
        
        if self.foodsource_ids.is_empty() {
            eprintln!("[ERROR]: A problem occured while trying to find foodsources - simulation stopped");
            exit(1);
            }

        /* Get current postion's id, and coordinates */
        let data_tuple = visited.chars()
            .last()
            .and_then(|position_id|
                self.points.iter()
                    .find(|point| point.id == position_id)
                    .map(|&Point { x, y, .. }| (position_id, x, y))
                );

        /* Unsafe note - unwrap is safe, because the route will never be empty, and the current position will always be contained */
        let (position_id, x, y) = unsafe {
            data_tuple.unwrap_unchecked()
            };

        /* Calculate preference scores for all the points, visited points get smallest score to avoid getting stuck */
        for (auxil, point) in self.auxils.iter_mut()
        .zip(self.points.iter()) {
            let viable = ! visited.contains(auxil.id) || self.foodsource_ids.contains(&position_id);
            auxil.ratio = select!(viable,
                (self.preference_operation)(point, x, y, self.distance_operation),
                bias::MINUTE
                )
            };

        /* Sort the helper array */
        self.sort_auxils();

        /* Get first new guess */
        let mut choice = (self.get_index_operation)(self);

        /* Double check, if not visited already */
        while visited.contains(self.auxils[choice].id) {
            choice = (self.get_index_operation)(self);
            }
        
        /* Return id of new position */
        self.auxils[choice].id
        }

    /* Cover the route with pheromones */
    pub fn cover_route(&mut self, visited: &str) {
        let cleared = visited.replace(self.anthill_id, "");

        for point in self.points.iter_mut()
        .filter(|point| cleared.contains(point.id)) {
            point.pheromone += self.pheromone
            }
        }

    /* Reduce amount of pheromones according to the function */
    pub fn disperse_pheromons(&mut self) {
        self.set_pheromones(self.disperse_operation);
        }

    /* Set amount of food at given point */
    pub fn set_foodsource(&mut self, position_id: char, amount: u32) {
        /* Try finding the point */
        let wrapped_point = self.points.iter_mut()
            .find(|point| point.id == position_id);

        /* Unsafe note - unwrap is safe, because the point ids are checked during the setup */
        let point = unsafe {
            wrapped_point.unwrap_unchecked()
            };
            
        /* Assign the amount, and add to foodsource list */
        point.food = amount;
        self.foodsource_ids.insert(position_id);
        }

    /* Decrease of food at given point */
    pub fn consume_foodsource(&mut self, position_id: char) {
        /* Try finding the point */
        let wrapped_point = self.points.iter_mut()
            .find(|point| point.id == position_id);

        /* Unsafe note - unwrap is safe, because the point ids are checked during the setup */
        let point = unsafe {
            wrapped_point.unwrap_unchecked()
            };

        /* Subtract amount from the point, if value goes to zero, remove from foodsource list */
        point.food = point.food.saturating_sub(self.consume_rate);
        if point.food == 0 {
            self.foodsource_ids.remove(&position_id);
            }
        }

    /* Check whether the point is a foodsource */
    pub fn is_foodsource(&self, position_id: &char) -> bool { 
        self.foodsource_ids.contains(position_id)
        }

    /* Reset points to original state - food, and pheromones */
    pub fn reset(&mut self) {
        self.points.iter_mut()
            .for_each(Point::reset);
        }

    /* Show a table of states of all points */
    pub fn show(&self) {
        println!(
"| o>--- world ---<o
{}| o>-------------<o",
            self.points.iter()
                .map(|Point { id, food, pheromone, .. }|
                    format!("| # {id}: {food:>4} - {pheromone}\n")
                    )
                .collect::<String>()
            );
        }

    /* Show a table of coordinates of all points */
    pub fn show_grid(&self) {
        println!(
"o> ---- GRID ---- <o
{}o> -------------- <o",
            self.points.iter()
                .map(|Point { id, x, y, .. }|
                    format!("| # {id}: ({x:>3},{y:>3})\n")
                    )
                .collect::<String>()
            );
        }

    /* Getters */
    #[inline]
    pub const fn get_anthill(&self) -> char
        { self.anthill_id }
    #[inline]
    pub const fn get_number_of_points(&self) -> usize
        { self.points.len() }
    #[inline]
    pub const fn do_ants_consume(&self) -> bool
        { self.consume_rate != 0 }
    #[inline]
    pub const fn do_ants_return(&self) -> bool
        { self.ants_return }
    pub fn get_pheromones_per_point(&self) -> Vec<f64> {
        self.points.iter()
            .map(|point| point.pheromone)
            .collect()
        }
    }