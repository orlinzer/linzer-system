use crate::broker::broker_controller::BrokerController;
use clap::{App, Arg};
use env_logger;
use log::LevelFilter;
use std::io::Result;

pub fn run_cli_setup() -> Result<BrokerController> {
    let matches: clap::ArgMatches = App::new("Linzer System")
        .version("1.0")
        .author("Or Linzer <orlinzer@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .required(false)
                .takes_value(true)
                .help("Sets the mode of the system (client/service)"),
        )
        .arg(
            Arg::with_name("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS")
                .required(false)
                .takes_value(true)
                .help("Sets the address of the system"),
        )
        .arg(
            Arg::with_name("log-level")
                .short('l')
                .long("log-level")
                .value_name("LOG_LEVEL")
                .required(false)
                .takes_value(true)
                .help("Sets the log level (error, warn, info, debug, trace)"),
        )
        .get_matches();

    let log_level = matches.value_of("log-level").unwrap_or("info");
    let log_level_filter = match log_level {
        "off" => LevelFilter::Off,
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "info" => LevelFilter::Info,
        "debug" => LevelFilter::Debug,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::new()
        .filter_level(log_level_filter)
        .init();

    let mode = matches.value_of("mode").unwrap_or("");

    match mode {
        "client" => {
            println!("Client mode");
        }
        "service" => {
            println!("Service mode");
        }
        _ => {}
    }

    let address = matches
        .value_of("address")
        .unwrap_or("0.0.0.0:0")
        .to_string();

    log::info!(
        "System CLI initialized with mode: {}, address: {}, log level: {}",
        mode,
        address,
        log_level
    );

    Ok(BrokerController::new(&address))
}

pub fn run_cli_loop(controller: &mut BrokerController) -> Result<()> {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let command: Vec<&str> = input.trim().split(' ').collect();

        match command[0] {
            "start" => {
                controller.start()?;
            }
            "stop" => {
                controller.stop()?;
            }
            "connection-add" => {
                if command.len() < 3 {
                    log::warn!("Usage: connection-add <address> <port>");
                    continue;
                }
                let address = command[1].to_string();
                controller.add_connection(&address)?;
            }
            "connection-remove" => {
                if command.len() < 2 {
                    log::warn!("Usage: connection-remove <address>");
                    continue;
                }
                let address = command[1].to_string();
                controller.remove_connection(&address)?;
            }
            "exit" => {
                log::info!("Exiting CLI");
                break;
            }
            _ => {
                log::warn!("Unknown command: {}", command[0]);
                log::info!(
                    "Available commands: start, stop, connection-add, connection-remove, exit"
                );
            }
        }
    }

    Ok(())
}
