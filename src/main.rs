/* Logical modules */
mod ant;
mod anthill;
mod world;
mod utils;

/* Technical modules */
mod args;
mod consts;
mod stats;
mod tech;

use {
    clap::Parser,
    std::{
        collections::HashSet,
        env::current_dir,
        process::exit
        },
    crate::{
        anthill::AntHill,
        args::Args,
        consts::GRID,
        stats::Stats,
        tech::SmartCell,
        world::World
        }
    };

fn main() {
    /* Parsing CL arguments */
    let (
        cycles, ants, decision_points, phero,
        rate, returns,
        select, preference, metric,
        dispersion, factor,
        actions, grid,
        mut show, output
    ) = {
        let args = Args::parse();

        ( args.cycles, args.ants, args.decision, args.pheromone,
        args.rate, args.returns,
        args.select, args.preference, args.metric,
        args.dispersion, args.factor,
        args.actions
            .and_then(|e| Some(e.build()))
            .unwrap_or_default(),
        args.grid
            .and_then(|e| Some(e.build()))
            .unwrap_or(Vec::from(GRID)),
        ! args.quiet, args.output )
        };

    /* Assert some conditions to avoid unnecessary errors */ {
        /* Prepare variables */
        let num_of_points = grid.len();
        let (point_ids, point_pos): (HashSet<char>, HashSet<(i32, i32)>) = grid.iter()
            .map(|e| (e.get_id(), e.get_position()))
            .unzip();
        let actions_ids = HashSet::from_iter(
            actions.values()
                .cloned()
                .flatten()
                .map(|(c, _)| c)
            );

        /* Prepare assertion variables */
        let resonable_num_of_points = (2 .. 1000).contains(&num_of_points);
        let points_have_unique_ids = point_ids.len() == num_of_points;
        let points_have_unique_postions = point_pos.len() == num_of_points;
        let correct_num_of_decision_points = (1 ..= num_of_points).contains(&decision_points);
        let resonable_num_of_ants = (1 .. 16777216).contains(&ants);
        let resonable_num_of_cycles = (1 .. 100).contains(&cycles);
        let positive_nonzero_pheromone_strength = 0.0 < phero;
        let unset_or_positive_dispersion_factor = factor.is_none_or(|n| 0.0 < n);
        let actions_correct = point_ids.is_superset(&actions_ids);
        let anthill_has_no_food = ! grid[0].has_food();

        /* Assert! */
        assertion!(
            resonable_num_of_points,
            points_have_unique_ids,
            points_have_unique_postions,
            correct_num_of_decision_points,
            resonable_num_of_ants,
            resonable_num_of_cycles,
            positive_nonzero_pheromone_strength,
            unset_or_positive_dispersion_factor,
            actions_correct,
            anthill_has_no_food
            );
        }

    
    /* Build World, and contain it inside smart pointer */
    let world_cell = {
        let Some(world) = World::builder()
            .point_list(grid)
            .decision_points(decision_points)
            .pheromone(phero)
            .ants_return(returns)
            .consume_rate(rate)
            .select_method(select)
            .point_preference(preference)
            .metric(metric)
            .dispersion_factor(dispersion, factor)
            .build()
        else {
            eprintln!("A problem occured while trying to build the world object - simulation stopped");
            exit(1);
            };
        SmartCell::new(world)
        };

    /* Create Ant Hill object */
    let mut ant_hill = AntHill::new(
        &world_cell,
        ants
        );


    /* Time saver */
    show = if show && 1023 < ants {
        println!("Printing hidden due to big number of ants");
        false
    } else { show };
        
    /* Container for statistics */
    let mut stats = Stats::default();
 
    /* Print information, if applicable */
    if show {
        println!("o>====== BEGINNING ======<o");
        ant_hill.show();
        world_cell.borrow()
            .show();
        println!("o>=======================<o\n");
        }


    /* Begin the simulation */
    for i in 0 .. cycles {
        ant_hill.action();

        /* Disperse pheromones, if applicable */
        if dispersion.is_some() {
            world_cell.borrow_mut()
                .disperse_pheromons();
            }

        /* Execute actions, if applicable */
        if let Some(acts) = actions.get(&i) {
            let mut world = world_cell.borrow_mut();
            for &(id, amount) in acts {
                world.set_foodsource(id, amount);
                }
            }

        /* Gather statistics */
        stats.ants_per_phase.push(ant_hill.get_satiated_ants_count());

        /* Print information, if applicable */
        if show {
            println!("o>======  PHASE {:>2} ======<o", i + 1);
            ant_hill.show();
            world_cell.borrow()
                .show();
            println!("o>=======================<o\n");
            }
        }


    /* Gather final statistics */
    stats.completed = ant_hill.has_all_ants_satiated();
    stats.pheromone_strengths = world_cell.borrow()
        .get_pheromones_per_point();
    stats.average_route_len = ant_hill.get_average_route_length();
    stats.average_returns = ant_hill.get_average_routes_count();

    /* Show simulation's settings */
    println!(
"o> ---- SETTINGS ---- <o
|            cycles: {}
|              ants: {}
|         pheromone: {}
|   decision points: {}
|   consumtion rate: {}
|           returns: {}
|         selection: {:?}
|       calculation: {:?}
|            metric: {:?}
|        dispersion: {:?}
| dispersion factor: {}",
    cycles, ants, phero, decision_points,
    rate, returns,
    select, preference, metric,
    dispersion, factor.unwrap_or(0.0),
    );

    /* Show simulation's statistics */
    stats.show();

    /* Save statistics to a file */
    if let Some(filename) = output {
        if let Ok(cwd) = current_dir() {
            if stats.write_to_file(cwd.join(&filename).as_path()).is_ok() {
                println!("Statistics saved in '{filename}'");
                }
            else {
                eprintln!("A problem occured while trying to save the statistics");
                }
            }
        else {
            eprintln!("A problem occured while trying to get the current working directory");
            }
        }
    }