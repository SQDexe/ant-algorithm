/*!
# Generic Ant Algorithm

A university project - an implementation of Ant Colony Optimisation Alogrithm, packaged with a small CLI tool for inspecting results, allows setting some parameters on the run, with the main route backed into the executable

## Acknowledgements
Based on `Klasyczny algorytm mrówkowy v.2.0` by Feliks Kurp
*/

/** **Logic module** - ants managment and logic. */
mod anthill;
/** **Logic module** - world space managment. */
mod world;
/** **Logic module** - grouping of other important structures. */
mod utils;

/** **Technical module** - CLI arguments declaration, and parsing. */
mod args;
/** **Technical module** - important constants. */
mod consts;
/** **Technical module** - application's errors. */
mod error;
/** **Technical module** - application's logging functionality. */
mod log;
/** **Technical module** - main simulation managment. */
mod simul;
/** **Technical module** - grouping of other important structures. */
mod tech;


use {
    clap::Parser,
    fastrand::seed,
    std::{
        time::{
            Instant,
            Duration
            },
        process::{
            ExitCode,
            Termination
            }
        },
    crate::{
        args::Args,
        error::RuntimeError,
        simul::Simulator,
        tech::ActionsConfig
        }
    };

#[cfg(not(debug_assertions))]
use std::panic::{
    set_hook,
    PanicHookInfo
    };



/** Fallible logic of the application. */
fn run() -> Result<(), RuntimeError> {
    /* Parse the CL arguments */
    let args = Args::parse();

    /* Break args into config structs */
    let (actions, config, disjoint) = args.into();
    let ActionsConfig { counts_time, output_path } = actions;

    /* If set, seed the random generator */
    if let Some(&value) = config.seed.as_ref() {
        seed(value);
        }

    /* Create a new simulation manager */
    let mut simulation = Simulator::new(config, disjoint)?;

    /* Get current time */
    let start = Instant::now();

    /* Simulate */
    simulation.simulate()?;

    /* Get full time */
    let stop = start.elapsed();

    /* Show informations */
    simulation.show();

    /* Show time information */
    if counts_time {
        show_time(stop);
        }

    /* Save statistics to a file */
    if let Some(path) = output_path.as_deref() {
        /* Try to save statistics */
        simulation.write_to_file(path)?;
        }
    
    Ok(())
    }

/** **Technical part** - prints simulation's duration data. */
fn show_time(duration: Duration) {
    println!(
"o> ---- TIME ---- <o
| seconds: {}
| microseconds: {}
o> -------------- <o",
        duration.as_secs_f64(),
        duration.as_micros()
        );
    }

/** **Technical part** - custom panic hook, a prettier panic handling for users. */
#[cfg(not(debug_assertions))]
pub fn panic_hook(panic_info: &PanicHookInfo<'_>) {
    /* Unpack panic payload */
    let msg = panic_info.payload_as_str()
        .unwrap_or("Unknown error");

    critical!("{msg}");
    }

/** Entry point of the program. */
fn main() -> ExitCode {
    /* Set custom panic hook, for better printing in "user" mode, doesn't work in debug mode */
    #[cfg(not(debug_assertions))]
    set_hook(Box::new(panic_hook));

    /* Run the program */
    let result = run();

    /* Catch any error back, and report them */
    if let Err(ref err) = result {
        error!("{err}");
        }

    result.map_or(ExitCode::FAILURE, |_| ExitCode::SUCCESS)
    }