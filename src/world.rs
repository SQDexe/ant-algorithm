use {
    fastrand::{
        f64 as random_f64,
        usize as random_usize
        },
    rustc_hash::FxBuildHasher,
    tinyvec::ArrayVec,
    sqds_tools::select,
    std::{
        collections::{ HashSet, HashMap },
        process::exit
        },
    core::iter::repeat_with,
    crate::{
        consts::{
            bias,
            limits::MAX_POINTS
            },
        tech::*,
        utils::*
        }
    };



/** `World` structure, for handling most of logic operations, and managing the grid. */
pub struct World {
    /** Number of points of the grid. */
    num_of_points: usize,
    /** Points container. */
    points: ArrayVec<[Point; MAX_POINTS]>,
    /** Auxils container. */
    auxils: ArrayVec<[Auxil; MAX_POINTS]>,
    /** Current points holding any food. */
    foodsource_ids: HashSet<char, FxBuildHasher>,
    /** Initial points holding any food. */
    initial_foodsources: HashMap<char, u32, FxBuildHasher>,
    /** Number of decision points. */
    number_of_decision_points: usize,
    /** Function for acquiring new index. */
    get_index_operation: fn (&Self) -> usize,
    /** Function for calculating point prefrence. */
    preference_operation: fn (&Point, i16, i16, DistanceFunction) -> f64,
    /** Function for calculating distance. */
    distance_operation: DistanceFunction,
    /** Function for calculating dispersion. */
    disperse_operation: fn (&Point, f64) -> f64,
    /** Possible dispersion coefficient. */
    factor: f64
    }

impl World {
    /** Constructor. */
    pub fn new(point_list: Vec<Point>, config: &Config) -> Self {
        let (initial_foodsources, foodsource_ids): (HashMap<_, _, _>, HashSet<_, _>) = point_list.iter()
            .filter_map(|point|
                (! point.is_empty())
                    .then_some(((point.id, point.food), point.id))
                )
            .unzip();

        /* Convert the points list, and get length. */
        let points = ArrayVec::from_iter(point_list);
        let num_of_points = points.len();

        /* Crate auxils list. */
        let auxils = repeat_with(Auxil::default)
            .take(num_of_points)
            .collect();

        /* Crate world */
        Self {
            num_of_points,
            points,
            auxils,
            foodsource_ids,
            initial_foodsources,
            number_of_decision_points: config.decision,
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
            }
        }

    /** Greedy selection method. */
    #[inline]
    const fn select_greedy(&self) -> usize
        { 0 }
    /** Random selection method. */
    fn select_randomly(&self) -> usize {
        random_usize(0 .. self.number_of_decision_points)
        }
    /** Roulette selection method. */
    fn select_roulette(&self) -> usize {
        /* Get helper array */
        let wheel: ArrayVec<[f64; MAX_POINTS]> = {
            let iter = self.auxils.iter()
                .take(self.number_of_decision_points)
                .map(|auxil| auxil.ratio);

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

    /** Reset auxils in sync with points - the ratios are overwritten each time. */
    fn reset_auxils(&mut self) {
        for (auxil, point) in self.auxils.iter_mut().zip(&self.points) {
            auxil.id = point.id
            }
        }

    /** Sort auxils from biggest to smallest. */
    fn sort_auxils(&mut self) {
        self.auxils.sort_unstable_by(|a, b|
            b.ratio.total_cmp(&a.ratio)
            );
        }

    /** Set pheromones according to passed function. */
    fn set_pheromones(&mut self, func: fn (&Point, f64) -> f64) {
        for point in &mut self.points {
            point.pheromone = func(point, self.factor).max(0.0)
            };
        }

    /** Create new position according to passed arguments. */
    pub fn get_new_position(&mut self, visited: &str) -> char {
        /* Clear the helper array */
        self.reset_auxils();
        
        /* Safety check - stop the simulation if true */
        if self.foodsource_ids.is_empty() {
            eprintln!("[ERROR]: A problem occured while trying to find foodsources - simulation stopped");
            exit(1);
            }

        /* Get current postion's id, and coordinates */
        let (current_id, current_x, current_y) = {
            /* Unsafe note - unwrap is safe, because the route will never be empty */
            let id = unsafe {
                visited.chars()
                    .last()
                    .unwrap_unchecked()
                };
            
            /* Retrive point data */
            let point = self.find_point(id);
            (point.id, point.x, point.y)
            };

        /* Calculate preference scores for all the points, visited points get smallest score to avoid getting stuck */
        let iter = self.auxils.iter_mut()
            .zip(&self.points);
        for (auxil, point) in iter {
            let viable = ! visited.contains(auxil.id) ||
                self.foodsource_ids.contains(&current_id);
            auxil.ratio = select!(viable,
                (self.preference_operation)(point, current_x, current_y, self.distance_operation),
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

    /** Cover the route with pheromones. */
    pub fn cover_route(&mut self, visited: &str, exclude: &[char], pheromone: f64) {
        let iter = self.points.iter_mut()
            .filter(|point|
                visited.contains(point.id) &&
                ! exclude.contains(&point.id)
                );
        for point in iter {
            point.pheromone += pheromone
            }
        }

    /** Reduce amount of pheromones according to the function. */
    pub fn disperse_pheromons(&mut self) {
        self.set_pheromones(self.disperse_operation);
        }

    fn find_point(&mut self, position_id: char) -> &mut Point {
        /* Unsafe note - unwrap is safe, because the point ids are checked during the setup */
        unsafe {
            /* Try finding the point */
            self.points.iter_mut()
                .find(|point| point.id == position_id)
                .unwrap_unchecked()
            }
        }

    /** Set amount of food at given point. */
    pub fn set_foodsource(&mut self, position_id: char, amount: u32) {
        let point = self.find_point(position_id);
            
        /* Assign the amount, and add to foodsource list */
        point.food = amount;
        self.foodsource_ids.insert(position_id);
        }

    /** Decrease food at given point. */
    pub fn consume_foodsource(&mut self, position_id: char, amount: u32) {
        let point = self.find_point(position_id);

        /* Subtract amount from the point, if value goes to zero, remove from foodsource list */
        point.food = point.food.saturating_sub(amount);
        if point.is_empty() {
            self.foodsource_ids.remove(&position_id);
            }
        }

    /** Check whether the point is a foodsource. */
    pub fn is_foodsource(&self, position_id: &char) -> bool { 
        self.foodsource_ids.contains(position_id)
        }

    /** Reset points to original state - food, and pheromones. */
    pub fn reset(&mut self) {
        /* Clear available foodsources */
        self.foodsource_ids.clear();

        /* Reset points */
        for point in &mut self.points {
            point.pheromone = 0.0;

            /* Additional reset if point had food initally */
            if let Some(&initial_value) = self.initial_foodsources.get(&point.id) {
                point.food = initial_value;
                self.foodsource_ids.insert(point.id);
                } 
            }
        }

    /** Show a table of states of all points. */
    pub fn show(&self) {
        let tmp: String = self.points.iter()
            .map(|Point { id, food, pheromone, .. }|
                format!("| # {id}: {food:>4} - {pheromone}\n")
                )
            .collect();

        println!(
"| o>--- world ---<o
{tmp}| o>-------------<o"
            );
        }

    /** Show a table of coordinates of all points. */
    pub fn show_grid(&self) {
        let tmp: String = self.points.iter()
            .map(|Point { id, x, y, .. }|
                format!("| # {id}: ({x:>3},{y:>3})\n")
                )
            .collect();

        println!(
"o> ---- GRID ---- <o
{tmp}o> -------------- <o"
            );
        }
    /** `points`' length getter.` */
    #[inline]
    pub const fn get_number_of_points(&self) -> usize
        { self.num_of_points }
    /** `pheromones_per_point` getter. */
    pub fn get_pheromones_per_point(&self) -> Vec<f64> {
        self.points.iter()
            .map(|point| point.pheromone)
            .collect()
        }
    }