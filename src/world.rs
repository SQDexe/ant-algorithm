use {
    arrayvec::ArrayVec,
    rustc_hash::FxBuildHasher,
    sqds_tools::select,
    std::{
        collections::{
            HashSet,
            HashMap
            },
        process::exit
        },
    core::{
        fmt::Write,
        iter::{
            repeat_with,
            zip
            }
        },
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
    points: ArrayVec<Point, MAX_POINTS>,
    /** Auxils container. */
    auxils: ArrayVec<Auxil, MAX_POINTS>,
    /** Current points holding any food. */
    foodsource_ids: HashSet<char, FxBuildHasher>,
    /** Initial points holding any food. */
    initial_foodsources: HashMap<char, u32, FxBuildHasher>,
    /** Number of decision points. */
    number_of_decision_points: usize,
    /** Method of acquiring new index. */
    selection_method: Selection,
    /** Method of calculating point prefrence. */
    preference_method: Preference,
    /** Method of calculating distance. */
    distance_method: Metric,
    /** Possible method of calculating dispersion, with it's coefficient. */
    dispersion_method: Option<(Dispersion, f64)>
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
            selection_method: config.select,
            preference_method: config.preference,
            distance_method: config.metric,
            dispersion_method: config.dispersion.map(|dispersion| (dispersion, config.factor))
            }
        }

    /** Reset auxils in sync with points - the ratios are overwritten each time. */
    fn reset_auxils(&mut self) {
        for (auxil, point) in zip(&mut self.auxils, &self.points) {
            auxil.id = point.id
            }
        }

    /** Sort auxils from biggest to smallest. */
    fn sort_auxils(&mut self) {
        self.auxils.sort_unstable_by(|a, b|
            b.ratio.total_cmp(&a.ratio)
            );
        }

    /** Calculate new preference values for the points. */
    fn calculate_preference(&mut self, visited: &str) {
        /* Get current postion's id, and coordinates */
        let (current_id, current_x, current_y) = {
            /* SAFETY: unwrap is safe, because the route will never be empty */
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
        for (auxil, point) in zip(&mut self.auxils, &self.points) {
            let viable = ! visited.contains(auxil.id) ||
                self.foodsource_ids.contains(&current_id);

            /* If valiable, assign new ratio, otherwise, assign lowest value */
            auxil.ratio = select!(viable,
                self.preference_method.calculate(point, current_x, current_y, self.distance_method),
                bias::MINUTE
                )
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

        /* Preference calculation */
        self.calculate_preference(visited);

        /* Sort the helper array */
        self.sort_auxils();

        /* Select only auxils from the range */
        /* SAFETY: Number of decision points is always within the number of points, checked during setup */
        let auxils = unsafe {
            self.auxils.get_unchecked(.. self.number_of_decision_points)
            };

        /* New position */
        let choice = loop {
            /* Get new position index */
            let index = self.selection_method.calculate(self.number_of_decision_points, &auxils);

            /* SAFETY: Returned index will always be in range */
            let auxil = unsafe {
                self.auxils.get_unchecked(index)
                };

            /* Get new guesses, until unvisited is found */
            if ! visited.contains(auxil.id) {
                break auxil;
                }
            };
        
        /* Return id of new position */
        choice.id
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

    /** Reduce amount of pheromones according to the function, if applicable. */
    pub fn disperse_pheromons(&mut self) {
        if let Some((dispersion, factor)) = self.dispersion_method {
            for point in &mut self.points {
                point.pheromone = dispersion.calculate(point, factor).max(0.0)
                }
            }
        }

    fn find_point(&mut self, position_id: char) -> &mut Point {
        /* SAFETY: unwrap is safe, because the point ids are checked during the setup */
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
        /* Preallocate string, 47 bytes is a rough estimate of format string length */
        let mut tmp = String::with_capacity(self.points.len() * 47);

        /* Fill the string */
        for Point { id, food, pheromone, .. } in &self.points {
            _ = writeln!(tmp, "| # {id}: {food:>4} - {pheromone}");
            }

        /* Print the table */
        println!("| o>--- world ---<o");
        print!("{tmp}");
        println!("| o>-------------<o");
        }

    /** Show a table of coordinates of all points. */
    pub fn show_grid(&self) {
        /* Preallocate string, 26 bytes is a rough estimate of format string length */
        let mut tmp = String::with_capacity(self.points.len() * 26);

        /* Fill the string */
        for Point { id, x, y, .. } in &self.points {
            _ = writeln!(tmp, "| # {id}: ({x:>3},{y:>3})");
            }

        /* Print the table */
        println!("o> ---- GRID ---- <o");
        print!("{tmp}");
        println!("o> -------------- <o");
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