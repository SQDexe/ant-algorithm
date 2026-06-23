/*! **Technical module** - main simulation managment. */

use {
    rustc_hash::FxBuildHasher,
    serde_json::{
        from_reader,
        to_writer_pretty
        },
    std::{
        collections::{
            HashMap,
            HashSet,
            hash_map::Values
            },
        fs::File,
        path::Path
        },
    crate::{
        info,
        anthill::AntHill,
        consts::limits::*,
        error::*,
        tech::*,
        utils::Point,
        world::World
        }
    };



/** `Simulation` structure, for managing the instatiating, asserting correct configuration, simulation running, prinitng, and saving data. */
#[derive(Debug, Clone)]
pub struct Simulator {
    /** Whether logging should happen. */
    logs: bool,
    /** Number or repetitions. */
    batch_size: usize,
    /** Simulation's configuration. */
    config: Config,
    /** Amount of food to add on corresponding cycle, and point. */
    actions: HashMap<usize, Box<[(Id, u32)]>, FxBuildHasher>,
    /** The colony of ants. */
    ant_hill: AntHill,
    /** World space object. */
    world: World,
    /** Statistics for each simulation run. */
    stats: Vec<Stats>
    }

impl Simulator {
    /** Constructor. */
    pub fn new(config: Config, disjoint_config: DisjointConfig) -> Result<Self, AssertionError> {
        /* Unpack config */
        let Config { cycles, ref ants, .. } = config;
        let DisjointConfig { no_logging, batch_size, grid, actions } = disjoint_config;

        /* Preproces arguments */
        let num_of_points = grid.len();
        let anthill = grid.first()
            .expect("The grid should always have first point");

        /* Build actions map */
        let actions = Self::actions_into_hashmap(actions, cycles);        

        /* Assert some conditions to avoid unnecessary errors */
        let grid_values = (grid.as_slice(), anthill, num_of_points);
        Self::assert(&config, grid_values, actions.values())?;

        /* Check whether the simulation runs once */
        let singleton = batch_size == 1;
        
        /* Set whether to log information */
        let logs = if no_logging || (PRINTABLE_ANTS_RANGE.contains(ants) && singleton) {
            ! no_logging
        } else {
            info!("Logging hidden");
            false
            };

        /* Create Anthill object */
        let ant_hill = AntHill::new(anthill.id, &config, num_of_points);
        
        /* Create World object */
        let world = World::new(grid, &config);

        /* Create Simulator object */
        Ok(Self {
            logs,
            batch_size,
            config,
            actions,
            stats: Vec::with_capacity(batch_size),         
            ant_hill,
            world
            })
        }

    /** Static, helper function for asserting simulation's conditions. */
    fn assert(config: &Config, grid_values: (&[Point], &Point, usize), actions: Values<'_, usize, Box<[(Id, u32)]>>) -> Result<(), AssertionError> {
        /* Unpack config */
        let Config { pheromone, decision, dispersion, factor, .. } = config;
        let (grid, anthill, num_of_points) = grid_values;

        /* Prepare variables */
        let (point_ids, point_pos): (HashSet<_, FxBuildHasher>, HashSet<_, FxBuildHasher>) = grid.into_iter()
            .map(|Point { id, x, y, .. }| (id, (x, y)))
            .unzip();
        let actions_ids = actions.flatten()
            .map(|&(id, _)| id)
            .collect();

        /* Assert! */
        if point_ids.len() != num_of_points
            { return Err(AssertionError::NonUniquePointIds); }
        if point_pos.len() != num_of_points
            { return Err(AssertionError::NonUniquePointPositions); }
        if ! (1 ..= num_of_points).contains(decision)
            { return Err(AssertionError::InvalidDecisionPoints); }
        if ! PHERO_RANGE.contains(pheromone)
            { return Err(AssertionError::PheromoneOutsideOfRange); }
        if dispersion.is_some_and(|mode| ! mode.is_factor_valid(factor))
            { return Err(AssertionError::InvalidDispersionCoefficient); }
        if ! point_ids.is_superset(&actions_ids)
            { return Err(AssertionError::NonOverlappingActionIds); }
        if ! anthill.is_empty() && actions_ids.contains(&anthill.id) 
            { return Err(AssertionError::NonEmptyAnthill); }

        Ok(())
        }

    /** Takes an iterator of actions, and crates a hash map from it */
    fn actions_into_hashmap<I>(actions: I, cycles: usize) -> HashMap<usize, Box<[(Id, u32)]>, FxBuildHasher>
    where I: IntoIterator<Item = Action> {
        let mut tmp: HashMap<_, Vec<_>, _> = HashMap::with_capacity_and_hasher(cycles, FxBuildHasher);

        /* Fill the temporary map */
        for Action { cycle, id, food_amount} in actions {
            tmp.entry(cycle)
                .or_default()
                .push((id, food_amount))
            }

        /* Rebuild into final file */
        tmp.into_iter()
            .map(|(cycle, action)| (cycle, action.into_boxed_slice()))
            .collect()
        }

    /** Run the simulation. */
    pub fn simulate(&mut self) -> Result<(), NoFoodSourceError> {
        /* Simulate number of times */
        for _ in 0 .. self.batch_size {
            /* Container for statistics */
            let mut ants_per_phase = Vec::with_capacity(self.config.cycles);

            /* Print information, if applicable */
            self.show_phase(None);

            /* Begin the simulation */
            for phase in 0 .. self.config.cycles {
                /* Make simulation step */
                self.ant_hill.action(&mut self.world)?;

                /* Disperse pheromones */
                self.world.disperse_pheromons();

                /* Execute actions, if applicable */
                if let Some(acts) = self.actions.get(&phase) {
                    for &(id, amount) in acts {
                        self.world.set_food_source(id, amount);
                        }
                    }

                /* Gather statistics */
                ants_per_phase.push(self.ant_hill.satiated_ants_count());

                /* Print information, if applicable */
                self.show_phase(Some(phase));
                }

            /* Gather final statistics */
            let stats = Stats::new(&self.ant_hill, &self.world, ants_per_phase);

            /* Add statistics */
            self.stats.push(stats);

            /* Reset */
            self.ant_hill.reset();
            self.world.reset();
            }

        Ok(())
        }

    /** Show the simulation's statistics based on their number. */
    pub fn show_stats(&self) {
        match self.stats.as_slice() {
            [single] =>
                single.show(),
            many => 
                AveragedStats::new(
                    self.config.cycles,
                    self.world.number_of_points(),
                    many
                    ).show()
            }
        }

    /** Show the simulation's phase statistics, based on phase number. */
    fn show_phase(&self, phase: Option<usize>) {
        if ! self.logs {
            return;
            }

        /* Show correct banner */
        if let Some(phase) = phase.and_then(|phase_num| phase_num.checked_add(1)) {
            println!("o>======  PHASE {:>2} ======<o", phase);
        } else {
            println!("o>====== BEGINNING ======<o");
            }
        
        /* Show phase stats */
        self.ant_hill.show();
        self.world.show();
        println!("o>=======================<o\n");
        }

    /** Show the simulation's summary. */
    pub fn show(&self) {
        /* Show world grid */
        self.world.show_grid();

        /* Show simulation's settings */
        self.config.show();

        /* Show simulation's statistics */
        self.show_stats();
        }

    /** Write statistics to file. */
    pub fn write_to_file(&mut self, path: &Path) -> Result<(), SaveError> {
        /* Empty statistics container */
        let mut data = Vec::with_capacity(self.batch_size);
        
        /* If file exists, pull it's contents */
        if path.exists() {
            /* File reader handle */
            let file = File::open(path)?;

            /* Extract current contents */
            let contents: Box<[_]> = from_reader(file)?;

            /* Push old statistics from the file */
            data.extend(contents);
            }
        
        /* File writer handle */
        let file = File::create(path)?;

        /* Push current statistics */
        data.extend_from_slice(&self.stats);

        /* Try writing statistics to the file */
        to_writer_pretty(file, &data)?;

        /* Write information */
        info!("Statistics saved in '{}'", path.display());

        Ok(())
        }
    }