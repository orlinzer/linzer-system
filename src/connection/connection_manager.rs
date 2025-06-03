#[allow(dead_code)]
use std::io::Result;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;

/// Represents a connection to a remote node
pub struct ConnectionManager {
    /// The address of the remote node
    address: Arc<Mutex<String>>,

    /// The socket used to communicate with the remote node
    socket: Option<UdpSocket>,

    /// The thread used to send and receive messages
    thread: Option<thread::JoinHandle<Result<()>>>,
}

/// Implements the connection to a remote node
impl ConnectionManager {
    /// Creates a new connection to a remote node
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the remote node
    ///
    /// # Returns
    ///
    /// A new connection to the remote node
    pub fn new(address: &String) -> Result<Self> {
        Ok(Self {
            address: Arc::new(Mutex::new(address.clone())),
            socket: None,
            thread: None,
        })
    }

    /// Starts the connection
    ///
    /// # Returns
    ///
    /// An error if the connection fails to start
    pub fn start(&mut self) -> Result<()> {
        self.thread = Some(thread::spawn(move || Ok(())));

        Ok(())
    }

    /// Stops the connection
    ///
    /// # Returns
    ///
    /// An error if the connection fails to stop
    pub fn stop(&mut self) -> Result<()> {
        println!("Connection Stopped");

        Ok(())
    }
}
