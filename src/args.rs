use {
    anyhow::{
        anyhow,
        Result as DynResult,
        Error
        },
    clap::Parser,
    std::{
        collections::HashMap,
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

/* Technical stuff - parsing, and storing of the CL arguments */
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
    #[clap(short, long, default_value_t = default::BATCH_SIZE, help = tips::BATCH)]
    pub batch: usize,
    #[clap(short, long, help = tips::OUTPUT)]
    pub output: Option<String>
    }

/* Technical stuff - aliases of some types */
type Action = (usize, char, u32);
type Pair = (char, u32);

/* Technical stuff - grid argument parser */
#[derive(Clone)]
pub struct GridTable (
    Vec<PointInfo>
    );

impl GridTable {
    pub fn build(self) -> Vec<PointInfo> {
        self.0
        }

    fn parse_line(line: &str) -> DynResult<PointInfo> {
        let parts: Vec<_> = line.split(',').collect();
        match parts.as_slice() {
            &[id, x, y] => Ok(PointInfo::Empty(
                id.parse()?,
                x.parse()?,
                y.parse()?
                )),
            &[id, x, y, food] => Ok(PointInfo::Food(
                id.parse()?,
                x.parse()?,
                y.parse()?,
                food.parse()?
                )),
            _ => Err(anyhow!("Parsing failed"))
            }

        }
    }

impl FromStr for GridTable {
    type Err = Error;

    fn from_str(s: &str) -> DynResult<Self> {
        Ok(Self (
            s.split(';')
                .filter_map(|e| Self::parse_line(e).ok())
                .collect()
            ))
        }
    }

/* Technical stuff - actions arguments parser */
#[derive(Clone)]
pub struct ActionTable (
    Vec<Action>
    );

impl ActionTable {
    pub fn build(self) -> HashMap<usize, Vec<Pair>> {
        let mut rest: HashMap<usize, Vec<Pair>> = HashMap::new();

        for (cycle, id, amount) in self.0.into_iter() {
            rest.entry(cycle)
                .or_default()
                .push((id, amount));
            }

        rest
        }

    fn parse_line(line: &str) -> DynResult<Action> {
        let parts: Vec<_> = line.split(',').collect();
        match parts.as_slice() {
            &[cycle, id, amount] => Ok((
                cycle.parse()?,
                id.parse()?,
                amount.parse()?
                )),
            _ => Err(anyhow!("Parsing failed"))
            }
        }
    }

impl FromStr for ActionTable {
    type Err = Error;

    fn from_str(s: &str) -> DynResult<Self> {
        Ok(Self (
            s.split(';')
                .filter_map(|e| Self::parse_line(e).ok())
                .collect()
            ))
        }
    }