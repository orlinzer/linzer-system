// #[allow(dead_code)]
// use std::io::Result;
// use std::net::{SocketAddr, UdpSocket};
// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

// /// Represents a listener to a remote node
// pub struct Listener {
//     /// The thread used to receive and send messages
//     thread: Option<thread::JoinHandle<Result<()>>>,

//     /// The channel used to communicate with the manager thread
//     channel: (Option<mpsc::Sender<String>>, Option<mpsc::Receiver<String>>),

//     /// The address of the remote node
//     address: Arc<Mutex<String>>,

//     /// The socket used to communicate with the remote node
//     socket: Arc<Mutex<Option<UdpSocket>>>,
// }

// /// Implements the listener to a remote node
// impl Listener {
//     /// Creates a new listener
//     ///
//     /// # Arguments
//     ///
//     /// * `address` - The address of the remote node
//     ///
//     /// # Returns
//     ///
//     /// A new listener to the remote node
//     pub fn new(address: &String) -> Self {
//         Self {
//             thread: None,
//             channel: (None, None),
//             address: Arc::new(Mutex::new(address.clone())),
//             socket: Arc::new(Mutex::new(None)),
//         }
//     }

//     /// Starts the listener
//     pub fn start(&mut self) -> Result<()> {
//         // Check if the thread is already running
//         if self.thread.is_some() {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::AlreadyExists,
//                 "Thread already exists",
//             ));
//         }

//         // Initialize the manager
//         self.initialize()?;

//         // Clone the manager for the thread
//         let mut self_clone = self.clone();

//         // Create a channel to communicate with the manager thread
//         let (sx, tx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

//         // Set the channel
//         self.channel = (Some(sx), None);
//         self_clone.channel = (None, Some(tx));

//         // Start the manager thread
//         self.thread = Some(thread::spawn(move || {
//             // Start the main loop
//             loop {
//                 match self_clone.main_loop() {
//                     Ok(result) => {
//                         if result == -1 {
//                             break Ok(());
//                         }
//                     }
//                     Err(e) => {
//                         break Err(e);
//                     }
//                 }
//             }
//         }));

//         Ok(())
//     }

//     pub fn stop(&mut self) -> Result<()> {
//         // TODO: Stop the socket

//         // Stop the manager thread
//         if let Some(thread) = self.thread.take() {
//             // Send a message to the manager thread
//             if let Some(sender) = &self.channel.0 {
//                 sender.send("stop".to_string()).map_err(|e| {
//                     std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))
//                 })?;
//             }

//             thread.join().map_err(|e| {
//                 std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))
//             })??;
//         }

//         Ok(())
//     }

//     fn clone(&self) -> Self {
//         Self {
//             thread: None,
//             channel: (None, None),
//             address: self.address.clone(),
//             // TODO clone the socket
//             socket: Arc::new(Mutex::new(None)),
//         }
//     }

//     fn initialize(&mut self) -> Result<()> {
//         // Parse the address
//         let parsed_address = self
//             .address
//             .as_ref()
//             .lock()
//             .map_err(|e| {
//                 std::io::Error::new(std::io::ErrorKind::Other, format!("Lock error: {:?}", e))
//             })?
//             .parse::<SocketAddr>()
//             .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

//         // Bind the socket to the address
//         let socket = UdpSocket::bind(parsed_address).expect("Failed to bind socket");
//         *self.socket.lock().unwrap() = Some(socket);

//         Ok(())
//     }

//     fn main_loop(&mut self) -> Result<i32> {
//         // Receive messages from the channel
//         match self.channel.1.as_ref().unwrap().try_recv() {
//             Ok(message) => {
//                 if message == "stop" {
//                     println!("Manager main loop stop");
//                     return Ok(-1);
//                 }
//             }
//             Err(_) => {}
//         }

//         let mut buffer = [0; 1024];
//         let socket = self.socket.lock().map_err(|e| {
//             std::io::Error::new(std::io::ErrorKind::Other, format!("Lock error: {:?}", e))
//         })?;
//         let socket = socket.as_ref().ok_or_else(|| {
//             std::io::Error::new(std::io::ErrorKind::NotConnected, "Socket not connected")
//         })?;
//         let (size, _) = socket
//             .recv_from(&mut buffer)
//             .expect("Failed to receive message");

//         let message = String::from_utf8_lossy(&buffer[..size]);
//         println!("Received message: {}", message);

//         Ok(0)
//     }
// }
