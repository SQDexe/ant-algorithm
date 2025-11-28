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
    std::time::Instant,
    crate::{
        args::Args,
        simul::Simulator
        }
    };

fn main() {    
    /* Parse the CL arguments */
    let mut args = Args::parse();
    let (output, timing) = (
        args.output.take(),
        args.timing
        );

    /* Create a new simulation manager */
    let mut simulation = Simulator::new(args);

    /* Get current time */
    let start = Instant::now();

    /* Simulate */
    simulation.simulate();

    /* Get full time */
    let stop = start.elapsed();

    /* Show informations */
    simulation.show();

    /* Show time information */
    if timing {
        println!(
"o> ---- TIME ---- <o
| seconds: {}
| microseconds: {}
o> -------------- <o",
            stop.as_secs_f64(),
            stop.as_micros()
            );
        }

    /* Save statistics to a file */
    if let Some(path) = output {        
        /* Try to save statistics */
        match simulation.write_to_file(&path) {
            Ok(_) => eprintln!("[INFO]: Statistics saved in '{}'", path.display()),
            _ => eprintln!("[ERROR]: A problem occured while trying to save the statistics")
            }
        }
    }