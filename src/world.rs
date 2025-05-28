use {
    rand::{
        random,
        random_range
        },
    std::{
        collections::HashSet,
        process::exit
        },
    crate::{
        consts::bias,
        tech::{
            DistanceFunction,
            Preference,
            Selection,
            Metric,
            Dispersion,
            PointInfo
            },
        utils::{
            distance,
            disperse,
            preference,
            Auxil,
            Point
            }
        }
    };

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
        let chance: f64 = random();

        while rest < chance {
            index += 1;
            rest += helper[index];
            }

        index
        }

    /* Logic methods */
    fn reset_auxils(&mut self) {
        self.auxils.iter_mut()
            .zip(self.points.iter())
            .for_each(|(auxil, point)|
                auxil.id = point.id
                );
        }

    fn sort_auxils(&mut self) {
        self.auxils.sort_unstable_by(|a, b|
            b.ratio.total_cmp(&a.ratio)
            );
        }

    fn set_pheromones(&mut self, func: fn (&Point, f64) -> f64) {
        self.points.iter_mut()
            .for_each(|point|
                point.pheromone = func(point, self.factor).max(0.0)
                );
        }

    pub fn get_new_position(&mut self, position_id: char, visited: &str) -> char {
        self.reset_auxils();
        
        if self.foodsource_ids.is_empty() {
            eprintln!("A problem occured while calculating postions - lack of foodsources");
            exit(1);
            }

        let Some(&Point { x, y, .. }) = self.points.iter()
            .find(|point| point.id == position_id)
        else {
            eprintln!("A problem occured while calculating postions - recived an invalid point id: {position_id}");
            exit(1);
            };

        self.auxils.iter_mut()
            .zip(self.points.iter())
            .for_each(|(auxil, point)| {
                auxil.ratio = if visited.contains(auxil.id) {
                    bias::MINUTE
                } else {
                    (self.preference_operation)(point, x, y, self.distance_operation)
                    }
                });

        self.sort_auxils();

        let mut choice = (self.get_index_operation)(self);
        let mut antiblock: u8 = 0;

        while visited.contains(self.auxils[choice].id) {
            choice = (self.get_index_operation)(self);

            antiblock += 1;
            if antiblock == u8::MAX {
                eprintln!("A problem occured while picking point - got stuck in loop");
                exit(1);
                }
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
        let Some(point) = self.points.iter_mut()
            .find(|point| point.id == position_id)
        else {
            eprintln!("A problem occured while updating points - recived an invalid point id: {position_id}");
            exit(1);
            };

        point.food = amount;
        self.foodsource_ids.insert(position_id);
        }

    pub fn consume_foodsource(&mut self, position_id: char) {
        let Some(point) = self.points.iter_mut()
            .find(|point| point.id == position_id) 
        else {
            eprintln!("A problem occured while consuming food - recived an invalid point id: {position_id}");
            exit(1);
            };

        if let Some(new_val) = point.food.checked_sub(self.consume_rate) {
            if new_val == 0 {
                self.foodsource_ids.remove(&position_id);
                }
            point.food = new_val;
        } else {
            eprintln!("A problem occured consuming food - tried consuming from an empty foodsource");
            exit(1);
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

/* Technical stuff */
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
                Auxil::new('\0', f64::NAN)
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
            disperse_operation: match self.dispersion_method {
                Some(Dispersion::Linear) => disperse::linear,
                Some(Dispersion::Exponential) => disperse::exponential,
                Some(Dispersion::Relative) => disperse::relative,
                _ => |_, _| f64::NAN
                },
            factor: self.factor.unwrap_or(f64::NAN)
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
    pub fn dispersion_factor(mut self, dispersion_method: Option<Dispersion>, factor: Option<f64>) -> Self {
        self.dispersion_method = dispersion_method;
        self.factor = factor;
        self
        }
    }