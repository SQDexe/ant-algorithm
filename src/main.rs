/* Logical modules */
mod ant;
mod anthill;
mod world;
mod utils;

/* Technical modules */
mod args;
mod consts;
mod simul;
mod tech;

use {
    clap::Parser,
    crate::{
        args::Args,
        simul::Simulator
        }
    };

fn main() {
    /* Parse the CL arguments */
    let args = Args::parse();
    let output = args.output.clone();

    /* Create a new simulation manager */
    let mut simulation = Simulator::new(args);

    /* Simulate */
    simulation.simulate();    

    /* Show informations */
    simulation.show();

    /* Save statistics to a file */
    if let Some(path) = output {        
        /* Try to save statistics */
        match simulation.write_to_file(&path) {
            Ok(_) => println!("Info: Statistics saved in '{}'", path.display()),
            _ => eprintln!("Error: A problem occured while trying to save the statistics")
            }
        }
    }