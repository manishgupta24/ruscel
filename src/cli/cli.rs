use clap::{App, Arg};
use log::error;
use std::path::Path;

// CommandLineInterface Struct to parse cli args
// and load them into a struct for processing
pub struct CommandLineInterface {
    pub config: String,
    pub verbosity: usize,
    pub worker: bool,
    pub worker_type: String,
}

impl CommandLineInterface {
    pub fn new(config: String, verbosity: usize, worker: bool, worker_type: String) -> Self {
        CommandLineInterface {
            config: config,
            verbosity: verbosity,
            worker: worker,
            worker_type: worker_type,
        }
    }

    pub fn validate(&self) {
        if !Path::new(&self.config).exists() {
            error!("config file not found");
        }
    }
}

pub fn cli_builder() -> CommandLineInterface {
    let matches = App::new("Ruscel")
        .version("0.1.0")
        .author("Manish Gupta <manishgupta.ait@gmail.com>")
        .about("A Distributed task queue")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
                .default_value("ruscel.conf"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the verbosity level"),
        )
        .arg(
            Arg::with_name("worker")
                .short("w")
                .help("Sets the worker mode"),
        )
        .arg(
            Arg::with_name("worker_type")
                .short("k")
                .possible_values(&["thread"])
                .default_value("thread")
                .required_if("worker", "")
                .help("Sets the worker type"),
        )
        .get_matches();

    let config = matches.value_of("config").unwrap().to_owned();
    let verbosity = matches.occurrences_of("verbose") as usize;
    let worker = matches.is_present("worker");
    let worker_type = matches.value_of("worker_type").unwrap().to_owned();

    let cli_app = CommandLineInterface::new(config, verbosity, worker, worker_type);
    cli_app.validate();
    cli_app
}
