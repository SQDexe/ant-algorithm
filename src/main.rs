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
    std::env::current_dir,
    crate::{
        args::Args,
        simul::Simulator,
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
    if let Some(filename) = output {
        /* Get current working directory */
        if let Ok(cwd) = current_dir() {
            /* Try to save statistics */
            if simulation.write_to_file(cwd.join(&filename).as_path()).is_ok() {
                println!("### Statistics saved in '{filename}' ###");
            } else {
                eprintln!("!!! A problem occured while trying to save the statistics !!!");
                }
        } else {
            eprintln!("!!! A problem occured while trying to get the current working directory !!!");
            }
        }
    }