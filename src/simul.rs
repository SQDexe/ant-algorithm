use {
    anyhow::Result as DynResult,
    serde_json::{
        from_str as from_json,
        to_string_pretty as to_json
        },
    sqds_tools::{
        select,
        batch_assert
        },
    std::{
        collections::{
            HashMap,
            HashSet
            },
        fs::File,
        io::{
            Read,
            Write
            },
        path::Path,
        rc::Rc
        },
    core::cell::RefCell,
    crate::{
        anthill::AntHill,
        args::Args,
        consts::{
            bias,
            GRID
            },
        tech::{
            Action,
            Config,
            ToDisplay,
            Dispersion,
            Stats
            },
        world::World
        }
    };

/* Simulation structure, for managing the instatiating, asserting correct configuration, simulation running, prinitng, and saving data */
pub struct Simulator {
    logs: bool,
    batch_size: usize,
    config: Config,
    actions: HashMap<usize, Vec<(char, u32)>>,
    ant_hill: AntHill,
    world_cell: Rc<RefCell<World>>,
    stats: Vec<Stats>,
    show_operation: fn (&Self)
    }

impl Simulator {
    /* Constructor */
    pub fn new(args: Args) -> Self {
        /* Unpack arguments */
        let Args {
            cycles, ants, pheromone, decision,
            rate, returns,
            select, preference, metric,
            dispersion,
            batch,
            factor, actions, grid,
            seed,
            .. } = args;

        /* Preproces arguments */
        let (factor, grid) = (
            factor.unwrap_or(bias::UNKOWN),
            grid.unwrap_or(Vec::from(GRID))
            );
        let actions: HashMap<_, Vec<_>> = actions.into_iter()
            .flatten()
            .fold(HashMap::new(), |mut map, Action(cycle, id, amount)| {
                map.entry(cycle)
                    .or_default()
                    .push((id, amount));

                map
                });

        /* Assert some conditions to avoid unnecessary errors */ {
            /* Prepare variables */
            let valid_grid_range = -99 ..= 99;
            let num_of_points = grid.len();
            let (point_ids, point_pos): (HashSet<_>, HashSet<_>) = grid.iter()
                .map(|point| (point.get_id(), point.get_position()))
                .unzip();
            let actions_ids = actions.values()
                .flatten()
                .map(|&(chr, _)| chr)
                .collect();

            /* Prepare assertion variables */
            let resonable_num_of_points = (2 .. 1000).contains(&num_of_points);
            let points_have_unique_ids = point_ids.len() == num_of_points;
            let points_have_unique_postions = point_pos.len() == num_of_points;
            let points_inside_gird = point_pos.iter()
                .all(|(x, y)| valid_grid_range.contains(x) && valid_grid_range.contains(y));
            let correct_num_of_decision_points = (1 ..= num_of_points).contains(&decision);
            let resonable_num_of_ants = (1 ..= 0xffffff).contains(&ants);
            let resonable_num_of_cycles = (1 .. 100).contains(&cycles);
            let positive_nonzero_pheromone_strength = 0.0 < pheromone;
            let unset_or_correct_dispersion_factor = match (dispersion, factor) {
                (Some(Dispersion::Linear), 0.0 ..) => true,
                (Some(Dispersion::Exponential), 1.0 ..) => true,
                (Some(Dispersion::Relative), 0.0 ..= 1.0) => true,
                (None, value) if value.is_nan() => true,
                _ => false
                };
            let actions_correct = point_ids.is_superset(&actions_ids);
            let anthill_has_no_food = {
                let anthill = &grid[0];
                ! (anthill.has_food() || actions_ids.contains(&anthill.get_id()))
                };
            let resonable_batch_size = (1 .. 1000).contains(&batch);

            /* Assert! */
            batch_assert!(
                resonable_num_of_points,
                points_have_unique_ids,
                points_have_unique_postions,
                points_inside_gird,
                correct_num_of_decision_points,
                resonable_num_of_ants,
                resonable_num_of_cycles,
                positive_nonzero_pheromone_strength,
                unset_or_correct_dispersion_factor,
                actions_correct,
                anthill_has_no_food,
                resonable_batch_size
                );
            }

        /* Check the batch size */
        let singleton = batch == 1;
        
        /* Set whether to log information */
        let logs = if ! args.quiet && (0xfff < args.ants || ! singleton) {
            eprintln!("[INFO]: Logging hidden");
            false
        } else {
            ! args.quiet
            };

        /* Create configuration container */
        let config = Config {
            cycles, ants, pheromone, decision,
            rate, returns,
            select, preference, metric,
            dispersion, factor,
            seed
            };
        
        /* Create World, and contain it inside smart pointer */
        let world_cell = {
            let world = World::new(grid, &config);
            Rc::new(RefCell::new(world))
            };

        /* Create Anthill object */
        let ant_hill = AntHill::new(&world_cell, ants);

        /* Create simulator */
        Self {
            logs,
            batch_size: batch,
            config,
            actions,
            stats: Vec::with_capacity(batch),         
            ant_hill,
            world_cell,
            show_operation: select!(singleton,
                Self::show_one,
                Self::show_avg
                )
            }
        }

    /* Run simulation */
    pub fn simulate(&mut self) {
        /* Simulate number of times */
        for _ in 0 .. self.batch_size {
            /* Container for statistics */
            let mut stats = Stats::default();

            /* Print information, if applicable */
            if self.logs {
                println!("o>====== BEGINNING ======<o");
                self.ant_hill.show();
                self.world_cell.borrow()
                    .show();
                println!("o>=======================<o\n");
                }

            /* Begin the simulation */
            for phase in 0 .. self.config.cycles {
                /* Make simulation step */
                self.ant_hill.action();

                let mut world = self.world_cell.borrow_mut();

                /* Disperse pheromones, if applicable */
                if self.config.dispersion.is_some() {
                    world.disperse_pheromons();
                    }

                /* Execute actions, if applicable */
                if let Some(acts) = self.actions.get(&phase) {
                    for &(id, amount) in acts {
                        world.set_foodsource(id, amount);
                        }
                    }

                /* Gather statistics */
                stats.ants_per_phase.push(self.ant_hill.get_satiated_ants_count());

                /* Print information, if applicable */
                if self.logs {
                    println!("o>======  PHASE {:>2} ======<o", phase + 1);
                    self.ant_hill.show();
                    world.show();
                    println!("o>=======================<o\n");
                    }
                }

            /* Gather final statistics */
            stats.completed = self.ant_hill.has_all_ants_satiated();
            stats.pheromone_strengths = self.world_cell.borrow()
                .get_pheromones_per_point();
            stats.average_route_len = self.ant_hill.get_average_route_length();
            stats.average_returns = self.ant_hill.get_average_routes_count();

            /* Add statistics */
            self.stats.push(stats);

            /* Reset */
            self.ant_hill.reset();
            self.world_cell.borrow_mut()
                .reset();
            }
        }
    
    /* Show once for singular simulation */
    fn show_one(&self) {
        let stats = &self.stats[0];

        println!(
"o> --------- STATISTICS --------- <o
|        all reached goal: {}
|    pheromones per point: [{}]
|    average route length: {}
| satiated ants per phase: [{}]
|  average routes per ant: {}
|    pheromones per route: [{}]
o> ------------------------------ <o",
            stats.completed,
            stats.pheromone_strengths.iter().to_display(", "),
            stats.average_route_len,
            stats.ants_per_phase.iter().to_display(", "),
            stats.average_returns,
            stats.get_pheromone_per_route().iter().to_display(", ")
            );
        }
    
    /* Show average for batch simulation */
    fn show_avg(&self) {
        let (
            times,
            pheromone_strengths,
            route_len,
            ants_per_phase,
            returns,
            pheromone_per_route
            ) = self.get_average_stats();

        println!(
"o> ------ AVG STATS OF {:>3} ------ <o
|  total completed routes: {}
|    pheromones per point: [{}]
|    average route length: {}
| satiated ants per phase: [{}]
|  average routes per ant: {}
|    pheromones per route: [{}]
o> ------------------------------ <o",
            self.batch_size,
            times,
            pheromone_strengths.iter().to_display(", "),
            route_len,
            ants_per_phase.iter().to_display(", "),
            returns,
            pheromone_per_route.iter().to_display(", ")
            );
        }

    /* Gets average statistics for whole batch */
    fn get_average_stats(&self) -> (usize, Vec<f64>, f64, Vec<f64>, f64, Vec<f64>) {
        let number_of_points = self.world_cell.borrow()
            .get_number_of_points();
        let batch = self.batch_size as f64;

        /* Set empty containers */
        let mut total = Stats::default();
        let mut total_complete_routes = 0;
        total.pheromone_strengths = vec![0.0; number_of_points];
        total.ants_per_phase = vec![0; self.config.cycles];
        let mut total_pheromone_per_route = vec![0.0; number_of_points];
  
        /* Get total statistics for whole batch */
        for stat in self.stats.iter() {
            total.average_route_len += stat.average_route_len;
            total.average_returns += stat.average_returns;
            total_complete_routes += stat.completed as usize;
  
            for (i, (strength, avg_strength)) in stat.pheromone_strengths.iter()
            .zip(stat.get_pheromone_per_route().iter())
            .enumerate() {
                total.pheromone_strengths[i] += strength;
                total_pheromone_per_route[i] += avg_strength;
                }
            for (i, ants) in stat.ants_per_phase.iter().enumerate() {
                total.ants_per_phase[i] += ants;
                }
            }
  
        /* Average out the totals */
        let avg_route_len = total.average_route_len / batch;
        let avg_returns = total.average_returns / batch;
        let (avg_pheromone_strengths, avg_pheromone_per_route) = total.pheromone_strengths.iter()
            .zip(total_pheromone_per_route.iter())
            .map(|(n, m)| (n / batch, m / batch))
            .unzip();
        let avg_ants_per_phase = total.ants_per_phase.iter()
            .map(|&n| n as f64 / batch)
            .collect();
        
        ( total_complete_routes, avg_pheromone_strengths, avg_route_len, avg_ants_per_phase, avg_returns, avg_pheromone_per_route )
        }

    /* Show the simulation's summary */
    pub fn show(&self) {
        /* Show world grid */
        self.world_cell.borrow()
            .show_grid();

        /* Show simulation's settings */
        self.config.show();

        /* Show simulation's statistics */
        (self.show_operation)(self);
        }

    /* Write statistics to file */
    pub fn write_to_file(&mut self, absolute_path: &Path) -> DynResult<()> {
        /* EMpty statistics container */
        let mut data = Vec::with_capacity(self.batch_size);
        
        /* If file exists, pull it's contents */
        if absolute_path.exists() {
            /* File reader handle */
            let mut file = File::open(&absolute_path)?;

            /* Fill the content buffer */
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            /* Push old statistics from the file */
            data.extend(from_json::<Box<[_]>>(&contents)?);
            }
        
        /* File writer handle */
        let mut file = File::create(&absolute_path)?;

        /* Push current statistics */
        data.append(&mut self.stats);

        /* Try writing statistics to the file */
        file.write_all(&to_json(&data)?.into_bytes())?;

        Ok(())
        }
    }