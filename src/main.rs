mod ant;
mod args;
mod anthill;
mod auxil;
mod consts;
mod enums;
mod point;
mod utils;
mod world;

use {
    clap::Parser,
    std::{
        cell::RefCell,
        rc::Rc
        },
    crate::{
        args::Args,
        anthill::AntHill,
        consts::{
            NUM_OF_POINTS,
            TABLE
            },
        world::World
        }
    };

fn main() {
    let (
        num_of_cycles,
        num_of_ants, num_of_decision_points,
        phero_strength,
        select_method, preference,
        returns, show
        ) = {
        let args = Args::parse();
        (
            args.cycles,
            args.ants, args.decision,
            args.pheromone,
            args.select, args.preference,
            args.returns, ! args.quiet
            )
        };

    assert!((2 .. 1000).contains(&NUM_OF_POINTS));
	assert!(1 <= num_of_decision_points);
    assert!(num_of_decision_points <= NUM_OF_POINTS);
	assert!((1 .. 100).contains(&num_of_cycles));
    assert!(0.0 < phero_strength);
	
    let world_cell = {
        let [(first, ..), .., (last, ..)] = TABLE;
        Rc::new(RefCell::new(World::new(
            &TABLE,
            num_of_decision_points,
            first,
            last,
            select_method.clone(),
            preference.clone()
            )))
        };

    let mut ant_hill = AntHill::new(
        &world_cell,
        num_of_ants,
        phero_strength,
        returns
        );

    let mut ants_per_phase = Vec::with_capacity(num_of_cycles);

    if show {
        println!("o>====== BEGINNING ======<o");
        ant_hill.show();
        world_cell.borrow()
            .show();
        println!("o>=======================<o\n");
        }

    for i in 0 .. num_of_cycles {
        ant_hill.action();

        ants_per_phase.push(ant_hill.get_satiated_ants_count());

        if show {
            println!("o>======  PHASE {:>2} ======<o", i + 1);
            ant_hill.show();
            world_cell.borrow()
                .show();
            println!("o>=======================<o\n");
            }
        }

    let average_route_len = ant_hill.get_average_route_length();
    let pheromone_strengths = world_cell.borrow()
        .get_pheromones_per_point();
    let average_returns = ant_hill.get_average_routes_count();
    let average_pheromone_strengths = if average_returns != 0.0 {
        pheromone_strengths.iter()
            .map(|phero| phero / average_returns)
            .collect()
    } else {
        vec![]
        };

    println!(
"o> ---- SETTINGS ---- <o
|            ants: {}
|          cycles: {}
|       pheromone: {}
| decision points: {}
|       selection: {:?}
|     calculation: {:?}
|         returns: {}
o> --- STATISTICS --- <o
|    pheromones per point: {:?}
|    average route length: {}
| satiated ants per phase: {:?}
| average returns per ant: {}
|   pheromones by returns: {:?}
o> ------------------ <o",
        num_of_ants, num_of_cycles, phero_strength, num_of_decision_points, select_method, preference, returns,
        pheromone_strengths,
        average_route_len, ants_per_phase,
        average_returns, average_pheromone_strengths
        );
    }