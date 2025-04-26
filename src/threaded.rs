// use std::{io::Result, sync::mpsc, thread};

// pub struct Threaded {
//     /// The thread of the manager
//     thread: Option<thread::JoinHandle<Result<()>>>,

//     /// The channel used to communicate with the manager thread
//     channel: (Option<mpsc::Sender<String>>, Option<mpsc::Receiver<String>>),
// }

// /// Implements the Threaded struct
// impl Threaded {
//     pub fn new() -> Self {
//         Self {
//             thread: None,
//             channel: (None, None),
//         }
//     }

//     pub fn start(&mut self) -> Result<()> {
//         if self.thread.is_some() {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::AlreadyExists,
//                 "Thread already exists",
//             ));
//         }

//         self.initialize()?;

//         let mut self_clone = self.clone();

//         let (sx, tx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

//         self.channel = (Some(sx), None);
//         self_clone.channel = (None, Some(tx));

//         self.thread = Some(thread::spawn(move || {
//             let _rx = self_clone.channel.1.as_ref().unwrap();

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

//     #[allow(dead_code)]
//     pub fn stop(&mut self) -> Result<()> {
//         if self.thread.is_none() {
//             return Err(std::io::Error::new(
//                 std::io::ErrorKind::NotFound,
//                 "Thread not found",
//             ));
//         }

//         if let Some(sender) = &self.channel.0 {
//             sender
//                 .send("stop".to_string())
//                 .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
//         }

//         self.thread
//             .take()
//             .unwrap()
//             .join()
//             .expect("Thread panicked")?;

//         Ok(())
//     }

//     fn initialize(&mut self) -> Result<()> {
//         Ok(())
//     }

//     fn main_loop(&mut self) -> Result<i32> {
//         Ok(0)
//     }
// }

// impl Clone for Threaded {
//     fn clone(&self) -> Self {
//         Threaded {
//             thread: None,
//             channel: (None, None),
//         }
//     }
// }
