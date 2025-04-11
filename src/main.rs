pub mod ant;
pub mod args;
pub mod anthill;
pub mod auxil;
pub mod point;
pub mod utils;
pub mod world;
pub mod consts;

use {
    clap::Parser,
    std::{
        cell::RefCell,
        rc::Rc
        },
    args::Args,
    anthill::AntHill,
    consts::{
        NUM_OF_POINTS,
        TABLE
        },
    world::World
    };

fn main() {
    let (num_of_ants, num_of_cycles, num_of_decision_points, select_method, phero_strength, show) = {
        let args = Args::parse();
        (args.ants, args.cycles, args.decision, args.method, args.phero, ! args.quiet)
        };

    assert!((2 .. 1000).contains(&NUM_OF_POINTS));
	assert!(1 <= num_of_decision_points);
    assert!(num_of_decision_points <= NUM_OF_POINTS);
	assert!((1 .. 100).contains(&num_of_cycles));
	
    let world_cell = {
        let [(first, ..), .., (last, ..)] = TABLE;
        Rc::new(RefCell::new(World::new(select_method.clone(), NUM_OF_POINTS, num_of_decision_points, first, last)))
        };

    world_cell.borrow_mut()
        .init(&TABLE);

    let mut ant_hill = AntHill::new(&world_cell, num_of_ants, phero_strength);

    let mut ants_per_phase = vec![0_usize; num_of_cycles];

    if show {
        println!("o>====== BEGINNING ======<o");
        ant_hill.show();
        world_cell.borrow()
            .show();
        println!("o>=======================<o\n");
        }

    for i in 0 .. num_of_cycles {
        ant_hill.action();

        ants_per_phase[i] = ant_hill.get_satiated_ants();

        if show {
            println!("o>======  PHASE {:>2} ======<o", i + 1);
            ant_hill.show();
            world_cell.borrow()
                .show();
            println!("o>=======================<o\n");
            }
        }

    let average_route_len = ant_hill.get_avrage_route_length();
    let pheromone_strengths = world_cell.borrow()
        .get_pheromones_per_point();

    println!(
"o> ---- SETTINGS ---- <o
|            ants: {}
|          cycles: {}
|       pheromone: {}
| decision points: {}
|       selection: {:?}
o> --- STATISTICS --- <o
|    average route length: {}
| satiated ants per phase: {:?}
|    pheromones per point: {:?}
o> ------------------ <o",
        num_of_ants, num_of_cycles, phero_strength, num_of_decision_points, select_method,
        average_route_len, ants_per_phase, pheromone_strengths
        );
    }