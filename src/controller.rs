// use crate::Manager;
// use std::io::Result;

// /// Represents the controller of the system
// pub struct Controller {
//     /// The manager of the system
//     manager: Manager,
// }

// /// Implements the controller of the system
// impl Controller {
//     /// Creates a new controller
//     ///
//     /// # Arguments
//     ///
//     /// * `address` - The address of this node
//     ///
//     /// # Returns
//     ///
//     /// A new controller
//     pub fn new(address: &String) -> Self {
//         Self {
//             manager: Manager::new(address),
//         }
//     }

//     /// Starts the system
//     ///
//     /// # Returns
//     ///
//     /// An error if the controller fails to start
//     pub fn start(&mut self) -> Result<()> {
//         self.manager.start()
//     }

//     /// Stops the system
//     ///
//     /// # Returns
//     ///
//     /// An error if the controller fails to stop
//     pub fn stop(&mut self) -> Result<()> {
//         self.manager.stop()
//     }

//     /// Adds a connection to the system
//     ///
//     /// # Arguments
//     ///
//     /// * `address` - The address of a remote node
//     ///
//     /// # Returns
//     ///
//     /// An error if the connection fails to be added
//     pub fn add_connection(&mut self, address: &String) -> Result<()> {
//         self.manager.add_connection(address)
//     }

//     /// Removes a connection from the system
//     pub fn remove_connection(&mut self, address: &String) -> Result<()> {
//         self.manager.remove_connection(address)
//     }
// }
