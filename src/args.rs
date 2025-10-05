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
        consts::default,
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
#[command(author, version, about)]
pub struct Args {
    /// Sets number of cycles
    #[clap(short, long, default_value_t = default::NUM_OF_CYCLES, long_help)]
    pub cycles: usize,
    /// Sets number of ants
    #[clap(short, long, default_value_t = default::NUM_OF_ANTS)]
    pub ants: usize,
    /// Sets the strength of pheromones
    #[clap(short, long, default_value_t = default::PHERO_STRENGTH)]
    pub pheromone: f64,
    /// Sets the number of decision points
    #[clap(short, long, default_value_t = default::NUM_OF_DECISION_POINTS)]
    pub decision: usize,

    /// Sets whether, and how much food is consumed
    #[clap(short, long, default_value_t = default::CONSUME_RATE)]
    pub rate: u32,
    /// Sets whether ants return to the anthill
    #[clap(short = 'R', long, action, default_value_t = default::RETURN_BEHAVIOUR)]
    pub returns: bool,

    /// Sets how points are selected
    #[clap(short = 'S', long, value_enum, default_value_t = default::SELECT_METHOD)]
    pub select: Selection,
    /// Sets how the point preference is calculated
    #[clap(short = 'P', long, value_enum, default_value_t = default::PREFERENCE_METHOD)]
    pub preference: Preference,
    /// Sets how the distance between points is calculated
    #[clap(short = 'M', long, value_enum, default_value_t = default::METRIC)]
    pub metric: Metric,

    /// Sets the dispersion mode
    #[clap(short = 'D', long, value_enum, requires = "factor", default_value_t = default::DISPERSION)]
    pub dispersion: Dispersion,
    /// Sets the coefficient of the dispersion,
    /// linear      - 0 <= factor,
    /// exponential - 1 <= factor,
    /// relative    - 0 <= factor <= 1
    #[clap(short = 'f', long, requires = "dispersion", verbatim_doc_comment)]
    pub factor: Option<f64>,
    
    /// Sets new world grid,
    /// must contain at least 2 points,
    /// the first point is automatically chosen as anthill,
    /// format 'id,x,y[,food]'
    /// 
    /// [default: a,6,1;b,13,1;c,4,3;d,4,5;e,8,5;f,6,8;g,10,8,15]
    #[clap(short = 'G', long, verbatim_doc_comment)]
    pub grid: Option<GridTable>,
    /// Sets food at existing points during runtime,
    /// format 'cycle,id,amount'
    #[clap(short = 'A', long, verbatim_doc_comment)]
    pub actions: Option<ActionTable>,

    /// Run program in quite mode
    #[clap(short, long, action, default_value_t = default::QUIET)]
    pub quiet: bool,
    /// Sets how many times to run the simulation
    #[clap(short, long, default_value_t = default::BATCH_SIZE)]
    pub batch: usize,
    /// A file to write statistics to in JSON format,
    /// will create, or append/truncate existing file,
    /// search path from current working directory
    #[clap(short, long, verbatim_doc_comment)]
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
    #[inline]
    pub fn build(self) -> Vec<PointInfo> {
        self.0
        }

    fn parse_line(line: &str) -> DynResult<PointInfo> {
        let parts: Box<[_]> = line.split(',').collect();
        match parts[..] {
            [id, x, y] => Ok(PointInfo::Empty(
                id.parse()?,
                x.parse()?,
                y.parse()?
                )),
            [id, x, y, food] => Ok(PointInfo::Food(
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
        let parts: Box<[_]> = line.split(',').collect();
        match parts[..] {
            [cycle, id, amount] => Ok((
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