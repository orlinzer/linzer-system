// #[allow(dead_code)]
// use crate::{threaded::Threaded, Connection, Listener};
// use std::{
//     collections::HashMap,
//     io::Result,
//     net::SocketAddr,
//     sync::{Arc, Mutex},
// };

// /// Represents the manager of the system
// pub struct Manager {
//     threaded: Option<Threaded>,

//     // /// The thread of the manager
//     // thread: Option<thread::JoinHandle<Result<()>>>,

//     // /// The channel used to communicate with the manager thread
//     // channel: (Option<mpsc::Sender<String>>, Option<mpsc::Receiver<String>>),
//     /// The address of this node
//     address: Arc<Mutex<String>>,

//     /// The listener used to receive messages
//     listener: Arc<Mutex<Option<Listener>>>,

//     /// The connections to the remote nodes
//     connections: Arc<Mutex<HashMap<String, Connection>>>,
// }

// /// Implements the manager of the system
// impl Manager {
//     /// Creates a new manager
//     ///
//     /// # Arguments
//     ///
//     /// * `address` - The address of this node
//     ///
//     /// # Returns
//     ///
//     /// A new manager
//     pub fn new(address: &String) -> Self {
//         Self {
//             threaded: Some(Threaded::new()),
//             // thread: None,
//             // channel: (None, None),
//             address: Arc::new(Mutex::new(address.clone())),
//             listener: Arc::new(Mutex::new(Some(Listener::new(address)))),
//             connections: Arc::new(Mutex::new(HashMap::new())),
//         }
//     }

//     /// Starts the manager
//     ///
//     /// # Returns
//     ///
//     /// An error if the manager fails to start
//     pub fn start(&mut self) -> Result<()> {
//         let _ = self.threaded.as_mut().expect("Threaded is None").start();

//         // // Check if the thread is already running
//         // if self.thread.is_some() {
//         //     return Err(std::io::Error::new(
//         //         std::io::ErrorKind::AlreadyExists,
//         //         "Thread already exists",
//         //     ));
//         // }

//         // // Initialize the manager
//         // self.initialize()?;

//         // // Clone the manager for the thread
//         // let mut self_clone = self.clone();

//         // // Create a channel to communicate with the manager thread
//         // let (sx, tx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

//         // // Set the channel
//         // self.channel = (Some(sx), None);
//         // self_clone.channel = (None, Some(tx));

//         // // Start the manager thread
//         // self.thread = Some(thread::spawn(move || {
//         //     // Start the main loop
//         //     loop {
//         //         match self_clone.main_loop() {
//         //             Ok(result) => {
//         //                 if result == -1 {
//         //                     break Ok(());
//         //                 }
//         //             }
//         //             Err(e) => {
//         //                 break Err(e);
//         //             }
//         //         }
//         //     }
//         // }));

//         Ok(())
//     }

//     /// Stops the manager
//     ///
//     /// # Returns
//     ///
//     /// An error if the manager fails to stop
//     pub fn stop(&mut self) -> Result<()> {
//         // Stop the connections

//         // Get guarded access to the connections
//         let mut connections_guard = self
//             .connections
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

//         for connection in connections_guard.values_mut() {
//             connection.stop()?;
//         }

//         // Stop the listener
//         self.listener
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?
//             .as_mut()
//             .ok_or_else(|| {
//                 std::io::Error::new(std::io::ErrorKind::Other, "Listener not initialized")
//             })?
//             .stop()?;

//         // Stop the manager thread
//         // if let Some(thread) = self.thread.take() {
//         // Send a message to the manager thread
//         // if let Some(sender) = &self.channel.0 {
//         //     sender.send("stop".to_string()).map_err(|e| {
//         //         std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))
//         //     })?;
//         // }

//         //     thread.join().map_err(|e| {
//         //         std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))
//         //     })??;
//         // }

//         Ok(())
//     }

//     /// Adds a connection to the manager
//     pub fn add_connection(&mut self, address: &String) -> Result<()> {
//         // Parse the address
//         let address = address
//             .parse::<SocketAddr>()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

//         // Get guarded access to the connections
//         let mut connections_guard = self
//             .connections
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

//         // Insert the new connection
//         connections_guard.insert(
//             address.to_string(),
//             Connection::new(&address.to_string())
//                 .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?,
//         );

//         Ok(())
//     }

//     /// Removes a connection from the manager
//     pub fn remove_connection(&mut self, address: &String) -> Result<()> {
//         // Parse the address
//         let address = address
//             .parse::<SocketAddr>()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

//         // Get guarded access to the connections
//         let mut connections_guard = self
//             .connections
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

//         // Remove the connection
//         connections_guard.remove(&address.to_string());

//         Ok(())
//     }

//     fn clone(&self) -> Self {
//         Manager {
//             threaded: None,
//             // channel: (None, None),
//             address: self.address.clone(),
//             listener: self.listener.clone(),
//             connections: self.connections.clone(),
//         }
//     }

//     fn initialize(&mut self) -> Result<()> {
//         // Start the listener
//         self.listener
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?
//             .as_mut()
//             .ok_or_else(|| {
//                 std::io::Error::new(std::io::ErrorKind::Other, "Listener not initialized")
//             })?
//             .start()?;

//         // Start the connections

//         // Get guarded access to the connections
//         let mut connections_guard = self
//             .connections
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

//         for connection in connections_guard.values_mut() {
//             connection.start()?;
//         }

//         Ok(())
//     }

//     // fn main_loop(&mut self) -> Result<i32> {
//     //     // Receive messages from the channel
//     //     match self.channel.1.as_ref().unwrap().try_recv() {
//     //         Ok(message) => {
//     //             if message == "stop" {
//     //                 println!("Manager main loop stop");
//     //                 return Ok(-1);
//     //             }
//     //         }
//     //         Err(_) => {}
//     //     }

//     //     // Sleep for a while
//     //     thread::sleep(std::time::Duration::from_secs(1));

//     //     Ok(0)
//     // }

//     fn finalize(&mut self) -> Result<()> {
//         // Stop the connections

//         // Get guarded access to the connections
//         let mut connections_guard = self
//             .connections
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

//         for connection in connections_guard.values_mut() {
//             connection.stop()?;
//         }

//         // Stop the listener
//         self.listener
//             .lock()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?
//             .as_mut()
//             .ok_or_else(|| {
//                 std::io::Error::new(std::io::ErrorKind::Other, "Listener not initialized")
//             })?
//             .stop()?;

//         Ok(())
//     }
// }
