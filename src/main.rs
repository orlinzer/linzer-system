use clap::{App, Arg};
// use linzer_system::Controller;
use std::io::Result;

/// The main function
///
/// # Returns
///
/// An error if the main function fails
fn main() -> Result<()> {
    // Get the command line arguments
    let matches: clap::ArgMatches = App::new("Linzer System")
        .version("1.0")
        .author("Or Linzer <orlinzer@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                // .required(true)
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
        .get_matches();

    // Check the mode
    let mode = matches.value_of("mode").unwrap_or("");

    match mode {
        "client" => {
            println!("Client mode");
            // client()?
        }
        "service" => {
            println!("Service mode");
            // service()?.join().expect("Thread panicked")?
        }
        _ => {
            // eprintln!("Invalid mode: {}", mode);
            // std::process::exit(1);
        }
    }

    // Get the address
    let address = matches.value_of("address").unwrap_or("0.0.0.0:0");

    // // Create the system controller
    // let mut controller = Controller::new(&address.to_string());

    // // Start the system controller
    // controller.start()?;

    loop {
        // Sleep for a while
        std::thread::sleep(std::time::Duration::from_secs(1));

        // TODO: Start the system CLI

        // Read the command from the user
        let mut input = String::new();

        std::io::stdin().read_line(&mut input)?;

        // Parse the command
        let command: Vec<&str> = input.trim().split(' ').collect();

        // Execute the command
        match command[0] {
            // "add" => {
            //     controller.add_connection(&command[1].to_string())?;
            // }
            // "remove" => {
            //     controller.remove_connection(&command[1].to_string())?;
            // }
            "exit" => {
                break;
            }
            _ => {
                eprintln!("Invalid command: {}", command[0]);
            }
        }
    }

    // Stop the system controller
    // controller.stop()?;

    return Ok(());
}
