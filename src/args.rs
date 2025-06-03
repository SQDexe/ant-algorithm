use {
    clap::Parser,
    std::{
        collections::HashMap,
        error::Error,
        str::FromStr
        },
    crate::{
        consts::{
            default,
            tips
            },
        tech::{
            Dispersion,
            Metric,
            Preference,
            Selection,
            PointInfo
            }
        }
    };

/* Technical stuff */
#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(short, long, default_value_t = default::NUM_OF_CYCLES, help = tips::CYCLES)]
    pub cycles: usize,
    #[clap(short, long, default_value_t = default::NUM_OF_ANTS, help = tips::ANTS)]
    pub ants: usize,
    #[clap(short, long, default_value_t = default::PHERO_STRENGTH, help = tips::PHEROMONE)]
    pub pheromone: f64,
    #[clap(short, long, default_value_t = default::NUM_OF_DECISION_POINTS, help = tips::DECISION)]
    pub decision: usize,

    #[clap(short, long, default_value_t = default::CONSUME_RATE, help = tips::RATE)]
    pub rate: u32,
    #[clap(short = 'R', long, action, default_value_t = default::RETURN_BEHAVIOUR, help = tips::RETURNS)]
    pub returns: bool,

    #[clap(short = 'S', long, value_enum, default_value_t = default::SELECT_METHOD, help = tips::SELECT)]
    pub select: Selection,
    #[clap(short = 'P', long, value_enum, default_value_t = default::PREFERENCE_METHOD, help = tips::PREFERENCE)]
    pub preference: Preference,
    #[clap(short = 'M', long, value_enum, default_value_t = default::METRIC, help = tips::METRIC)]
    pub metric: Metric,

    #[clap(short = 'D', long, value_enum, requires = "factor", default_value_t = default::DISPERSION, help = tips::DISPERSION)]
    pub dispersion: Dispersion,
    #[clap(short = 'f', long, help = tips::FACTOR)]
    pub factor: Option<f64>,
    
    #[clap(short = 'G', long, help = tips::GRID)]
    pub grid: Option<GridTable>,
    #[clap(short = 'A', long, help = tips::ACTIONS)]
    pub actions: Option<ActionTable>,

    #[clap(short, long, action, default_value_t = default::QUIET, help = tips::QUIET)]
    pub quiet: bool,
    #[clap(short, long, help = tips::OUTPUT)]
    pub output: Option<String>
    }

/* Technical stuff */
type GenericError = Box<dyn Error + Send + Sync>;
type Action = (usize, char, u32);
type Pair = (char, u32);

#[derive(Clone)]
pub struct GridTable (
    Vec<PointInfo>
    );

impl GridTable {
    pub fn build(self) -> Vec<PointInfo> {
        self.0
        }
    }

impl FromStr for GridTable {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(GridTable (
            s.split(';')
                .filter_map(|e| match e.split(',').collect::<Vec<&str>>()[..] {
                    [id, x, y] => Some(PointInfo::Empty(
                        id.parse().ok()?,
                        x.parse().ok()?,
                        y.parse().ok()?
                        )),
                    [id, x, y, food] => Some(PointInfo::Food(
                        id.parse().ok()?,
                        x.parse().ok()?,
                        y.parse().ok()?,
                        food.parse().ok()?
                        )),
                    _ => None
                    })
                .collect()
            ))
        }
    }

#[derive(Clone)]
pub struct ActionTable (
    Vec<Action>
    );

impl ActionTable {
    pub fn build(self) -> HashMap<usize, Vec<Pair>> {
        let mut rest: HashMap<usize, Vec<Pair>> = HashMap::new();

        self.0.into_iter()
            .for_each(|(cycle, id, amount)| {
            let pair = (id, amount);
            if let Some(points) = rest.get_mut(&cycle) {
                points.push(pair);
            } else {
                rest.insert(cycle, Vec::from([pair]));
                }
            });

        rest
        }
    }

impl FromStr for ActionTable {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ActionTable (
            s.split(';')
                .filter_map(|e| match e.split(',').collect::<Vec<&str>>()[..] {
                    [cycle, id, amount] => Some((
                        cycle.parse().ok()?,
                        id.parse().ok()?,
                        amount.parse().ok()?
                        )),
                    _ => None
                    })
                .collect()
            ))
        }
    }