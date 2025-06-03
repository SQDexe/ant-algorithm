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
            BoolSelect,
            Dispersion,
            DistanceFunction,
            Metric,
            PointInfo,
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
    ( into $points:expr, $id:ident ) => {
        $points.into_iter()
            .find(|point| point.id == $id)
        };
    }

pub struct World {
    points: Vec<Point>,
    auxils: Vec<Auxil>,
    anthill_id: char,
    foodsource_ids: HashSet<char>,
    number_of_decision_points: usize,
    pheromone: f64,
    consume_rate: u32,
    ants_return: bool,
    get_index_operation: fn (&Self) -> usize,
    preference_operation: fn (&Point, i32, i32, DistanceFunction) -> f64,
    distance_operation: DistanceFunction,
    disperse_operation: fn (&Point, f64) -> f64,
    factor: f64
    }

impl World {
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
        }

    /* Selection methods */
    const fn select_greedy(&self) -> usize
        { 0 }
    fn select_randomly(&self) -> usize
        { random_range(0 .. self.number_of_decision_points) }
    fn select_roulette(&self) -> usize {
        let nodp = self.number_of_decision_points;
        
        let sum: f64 = self.auxils.iter()
            .take(nodp)
            .map(|auxil| auxil.ratio)
            .sum();

        let helper: Vec<f64> = self.auxils.iter()
            .take(nodp)
            .map(|auxil| auxil.ratio / sum)
            .collect();

        let mut index = 0;
        let mut rest = helper[index];
        
        let chance = random();

        while rest < chance {
            index += 1;
            rest += helper[index];
            }

        index
        }

    /* Logic methods */
    fn reset_auxils(&mut self) {
        for (auxil, point) in zip!(mut self.auxils, self.points) {
            auxil.id = point.id
            }
        }

    fn sort_auxils(&mut self) {
        self.auxils.sort_unstable_by(|a, b|
            b.ratio.total_cmp(&a.ratio)
            );
        }

    fn set_pheromones(&mut self, func: fn (&Point, f64) -> f64) {
        for point in &mut self.points {
            point.pheromone = func(point, self.factor).max(0.0)
            };
        }

    pub fn get_new_position(&mut self, position_id: char, visited: &str) -> char {
        self.reset_auxils();
        
        if self.foodsource_ids.is_empty() {
            error_exit!(1, "!!! A problem occured while calculating postions - lack of foodsources !!!");
            }

        let Some(&Point { x, y, .. }) = find_point!(self.points, position_id)
        else {
            error_exit!(1, "!!! A problem occured while calculating postions - recived an invalid point id: {} !!!", position_id);
            };

        for (auxil, point) in zip!(mut self.auxils, self.points) {
            auxil.ratio = visited.contains(auxil.id).select(
                bias::MINUTE,
                (self.preference_operation)(point, x, y, self.distance_operation)
                )
            };

        self.sort_auxils();

        let mut choice = (self.get_index_operation)(self);

        while visited.contains(self.auxils[choice].id) {
            choice = (self.get_index_operation)(self);
            }
        
        self.auxils[choice].id
        }

    pub fn cover_route(&mut self, visited: &str) {
        let cleared = visited.replace(self.anthill_id, "");

        for point in self.points.iter_mut()
        .filter(|point| cleared.contains(point.id)) {
            point.pheromone += self.pheromone
            }
        }

    pub fn reset_pheromons(&mut self) {
        self.set_pheromones(|_, _| 0.0);
        }

    pub fn disperse_pheromons(&mut self) {
        self.set_pheromones(self.disperse_operation);
        }

    pub fn set_foodsource(&mut self, position_id: char, amount: u32) {
        let Some(point) = find_point!(mut self.points, position_id)
        else {
            error_exit!(1, "!!! A problem occured while updating points - recived an invalid point id: {} !!!", position_id);
            };

        point.food = amount;
        self.foodsource_ids.insert(position_id);
        }

    pub fn consume_foodsource(&mut self, position_id: char) {
        let Some(point) = find_point!(mut self.points, position_id)
        else {
            error_exit!(1, "!!! A problem occured while consuming food - recived an invalid point id: {} !!!", position_id);
            };

        if let Some(amount) = point.food.checked_sub(self.consume_rate) {
            point.food = amount;
            if amount == 0 {
                self.foodsource_ids.remove(&position_id);
                }
        } else {
            error_exit!(1, "!!! A problem occured consuming food - tried consuming from an empty foodsource !!!");
            }
        }

    pub fn is_foodsource(&self, position_id: &char) -> bool { 
        self.foodsource_ids.contains(position_id)
        }

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

    /* Getters */
    pub fn get_anthill(&self) -> char
        { self.anthill_id }
    pub fn do_ants_consume(&self) -> bool
        { self.consume_rate != 0 }
    pub fn do_ants_return(&self) -> bool
        { self.ants_return }
    pub fn get_pheromones_per_point(&self) -> Vec<f64> {
        self.points.iter()
            .map(|point| point.pheromone)
            .collect()
        }
    }


/* Technical stuff */
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

/* Technical stuff - world builder */
impl WorldBuilder {
    pub fn build(self) -> Option<World> {
        let point_list = self.point_list?;

        let anthill_id = point_list.iter()
            .next()?
            .get_id();

        let foodsource_ids = HashSet::from_iter(
            point_list.iter()
                .filter_map(|&point_info| match point_info {
                    PointInfo::Food(.., 0) => None,
                    PointInfo::Food(id, ..) => Some(id),
                    _ => None
                    })
            );

        let (points, auxils) = point_list.into_iter()
            .map(|point_info| (
                match point_info {
                    PointInfo::Empty(id, x, y) => Point::new(id, x, y, 0),
                    PointInfo::Food(id, x, y, food) => Point::new(id, x, y, food)    
                    },
                Auxil::new('\0', bias::UNKOWN)
                ))
            .unzip();

        Some(World {
            points,
            auxils,
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
            factor: self.factor.unwrap_or(bias::UNKOWN)
            })
        }

    /* Setters */
    pub fn point_list(mut self, point_list: Vec<PointInfo>) -> Self {
        self.point_list = Some(point_list);
        self
        }
    pub fn decision_points(mut self, number_of_decision_points: usize) -> Self {
        self.number_of_decision_points = Some(number_of_decision_points);
        self
        }
    pub fn pheromone(mut self, pheromone: f64) -> Self {
        self.pheromone = Some(pheromone);
        self
        }
    pub fn ants_return(mut self, ants_return: bool) -> Self {
        self.ants_return = Some(ants_return);
        self
        }
    pub fn consume_rate(mut self, consume_rate: u32) -> Self {
        self.consume_rate = Some(consume_rate);
        self
        }
    pub fn select_method(mut self, select_method: Selection) -> Self {
        self.select_method = Some(select_method);
        self
        }
    pub fn point_preference(mut self, point_preference: Preference) -> Self {
        self.point_preference = Some(point_preference);
        self
        }
    pub fn metric(mut self, metric: Metric) -> Self {
        self.metric = Some(metric);
        self
        }
    pub fn dispersion_factor(mut self, dispersion_method: Dispersion, factor: f64) -> Self {
        self.dispersion_method = Some(dispersion_method);
        self.factor = Some(factor);
        self
        }
    }