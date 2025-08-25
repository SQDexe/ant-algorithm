use {
    rand::{
        random,
        random_range
        },
    std::collections::HashSet,
    crate::{
        error_exit,
        zip,
        consts::bias,
        tech::{
            DistanceFunction,
            PointInfo,
            BoolSelect,
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

/* Technical stuff - macro for finding point */
macro_rules! find_point {
    ( $points:expr, $id:ident ) => {
        $points.iter()
            .find(|point| point.id == $id)
        };
    ( mut $points:expr, $id:ident ) => {
        $points.iter_mut()
            .find(|point| point.id == $id)
        };
    }

/* World structure, for handling most of logic operations, and managing the grid */
pub struct World {
    points: Vec<Point>,
    auxils: Vec<Auxil>,
    foods: Vec<u32>,
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
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
        }

    /* Selection methods */
    const fn select_greedy(&self) -> usize
        { 0 }
    fn select_randomly(&self) -> usize
        { random_range(0 .. self.number_of_decision_points) }
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
        let chance = random();

        /* Spin the wheel until it stops */
        while rest < chance {
            index += 1;
            rest += wheel[index];
            }

        index
        }

    /* Reset auxils in sync with points - the ratios are overwritten each time */
    fn reset_auxils(&mut self) {
        for (auxil, point) in zip!(mut self.auxils, self.points) {
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

    /* Reset all pheromones to zero */
    fn reset_pheromons(&mut self) {
        self.set_pheromones(|_, _| 0.0);
        }

    /* Reset all foodsources to original state */
    fn reset_points(&mut self) {
        for (point, &food) in zip!(mut self.points, self.foods) {
            point.food = food;
            }
        }

    /* Create new position according to passed arguments */
    pub fn get_new_position(&mut self, visited: &str) -> char {
        /* Clear the helper array */
        self.reset_auxils();

        /* Get current postion */
        let Some(position_id) = visited.chars().last()
        else {
            error_exit!(1, "!!! A problem occured while calculating postions - got empty route !!!");
            };
        
        if self.foodsource_ids.is_empty() {
            error_exit!(1, "!!! A problem occured while calculating postions - lack of foodsources !!!");
            }

        /* Find current position's coordinates */
        let Some(&Point { x, y, .. }) = find_point!(self.points, position_id)
        else {
            error_exit!(1, "!!! A problem occured while calculating postions - recived an invalid point id: {} !!!", position_id);
            };

        /* Calculate preference scores for all the points, visited points get smallest score to avoid getting stuck */
        for (auxil, point) in zip!(mut self.auxils, self.points) {
            let viable = ! visited.contains(auxil.id) || self.foodsource_ids.contains(&position_id);
            auxil.ratio = viable.select(
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
        let Some(point) = find_point!(mut self.points, position_id)
        else {
            error_exit!(1, "!!! A problem occured while updating points - recived an invalid point id: {} !!!", position_id);
            };

        /* Assign the amount, and add to foodsource list */
        point.food = amount;
        self.foodsource_ids.insert(position_id);
        }

    /* Decrease of food at given point */
    pub fn consume_foodsource(&mut self, position_id: char) {
        let Some(point) = find_point!(mut self.points, position_id)
        else {
            error_exit!(1, "!!! A problem occured while consuming food - recived an invalid point id: {} !!!", position_id);
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

    /* Reset points to original state */
    pub fn reset(&mut self) {
        self.reset_pheromons();
        self.reset_points();
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
    pub const fn get_anthill(&self) -> char
        { self.anthill_id }
    pub const fn get_number_of_points(&self) -> usize
        { self.points.len() }
    pub const fn do_ants_consume(&self) -> bool
        { self.consume_rate != 0 }
    pub const fn do_ants_return(&self) -> bool
        { self.ants_return }
    pub fn get_pheromones_per_point(&self) -> Vec<f64> {
        self.points.iter()
            .map(|point| point.pheromone)
            .collect()
        }
    }


/* Technical stuff - World builder */
#[derive(Default)]
pub struct WorldBuilder {
    point_list: Option<Vec<PointInfo>>,
    number_of_decision_points: Option<usize>,
    pheromone: Option<f64>,
    consume_rate: Option<u32>,
    ants_return: Option<bool>,
    select_method: Option<Selection>,
    point_preference: Option<Preference>,
    metric: Option<Metric>,
    dispersion_method: Option<Dispersion>,
    factor: Option<f64>,
    }

/* Technical stuff - World builder */
impl WorldBuilder {
    /* Technical stuff - World builder */
    pub fn build(self) -> Option<World> {
        let point_list = self.point_list?;

        let anthill_id = point_list.iter()
            .next()?
            .get_id();

        let foodsource_ids = HashSet::from_iter(
            point_list.iter()
                .filter_map(|point_info| match point_info {
                    &PointInfo::Food(.., 0) => None,
                    &PointInfo::Food(id, ..) => Some(id),
                    _ => None
                    })
            );

        let (points, auxils): (Vec<_>, Vec<_>) = point_list.into_iter()
            .map(|point_info| (
                match point_info {
                    PointInfo::Empty(id, x, y) => Point::new(id, x, y, 0),
                    PointInfo::Food(id, x, y, food) => Point::new(id, x, y, food)    
                    },
                Auxil::new('\0', bias::UNKOWN)
                ))
            .unzip();

        let foods = points.iter()
            .map(|point| point.food)
            .collect();

        let world = World {
            points,
            auxils,
            foods,
            anthill_id,
            foodsource_ids,
            number_of_decision_points: self.number_of_decision_points?,
            pheromone: self.pheromone?,
            ants_return: self.ants_return?,
            consume_rate: self.consume_rate?,
            get_index_operation: match self.select_method? {
                Selection::Random => World::select_randomly,
                Selection::Roulette => World::select_roulette,
                Selection::Greedy => World::select_greedy
                },
            preference_operation: match self.point_preference? {
                Preference::Distance => preference::distance,
                Preference::Pheromone => preference::pheromone,
                Preference::Food => preference::food,
                Preference::PD => preference::phero_dist,
                Preference::FD => preference::food_dist,
                Preference::PF => preference::phero_food,
                Preference::PFD => preference::phero_food_dist
                },
            distance_operation: match self.metric? {
                Metric::Chebyshev => distance::chebyshev,
                Metric::Euclidean => distance::euclidean,
                Metric::Taxicab => distance::taxicab
                },
            disperse_operation: match self.dispersion_method? {
                Dispersion::Linear => disperse::linear,
                Dispersion::Exponential => disperse::exponential,
                Dispersion::Relative => disperse::relative,
                _ => |_, _| bias::UNKOWN
                },
            factor: self.factor?
            };

        Some(world)
        }

    /* Setters */
    pub fn point_list(mut self, point_list: Vec<PointInfo>) -> Self {
        self.point_list = Some(point_list);
        self
        }
    pub const fn decision_points(mut self, number_of_decision_points: usize) -> Self {
        self.number_of_decision_points = Some(number_of_decision_points);
        self
        }
    pub const fn pheromone(mut self, pheromone: f64) -> Self {
        self.pheromone = Some(pheromone);
        self
        }
    pub const fn ants_return(mut self, ants_return: bool) -> Self {
        self.ants_return = Some(ants_return);
        self
        }
    pub const fn consume_rate(mut self, consume_rate: u32) -> Self {
        self.consume_rate = Some(consume_rate);
        self
        }
    pub const fn select_method(mut self, select_method: Selection) -> Self {
        self.select_method = Some(select_method);
        self
        }
    pub const fn point_preference(mut self, point_preference: Preference) -> Self {
        self.point_preference = Some(point_preference);
        self
        }
    pub const fn metric(mut self, metric: Metric) -> Self {
        self.metric = Some(metric);
        self
        }
    pub const fn dispersion_factor(mut self, dispersion_method: Dispersion, factor: f64) -> Self {
        self.dispersion_method = Some(dispersion_method);
        self.factor = Some(factor);
        self
        }
    }