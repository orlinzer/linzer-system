// // Get the configuration from the environment
// // TODO: Get the configuration from the arguments
// // TODO: Get the configuration from the configuration file (JSON, TOML, YAML)
// // TODO: Get the configuration from the command line arguments

// #[cfg(not(feature = "production"))]
// use dotenv::dotenv;
// use std::env;

// /// Represents the configuration of the system
// #[allow(dead_code)]
// pub struct Config {
//     /// The address of the system
//     pub address: String,

//     /// The addresses of the connections of the system
//     pub connections: Vec<String>,
//     // ...
// }

// impl Config {
//     /// Creates a new configuration from the environment variables
//     #[allow(dead_code)]
//     pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
//         #[cfg(not(feature = "production"))]
//         dotenv().ok();

//         let address = env::var("ADDRESS")?;
//         let connections: Vec<String> = env::var("CONNECTIONS")?
//             .split(',')
//             .map(|s| s.to_string())
//             .collect();

//         Ok(Config {
//             address,
//             connections,
//         })
//     }
// }
