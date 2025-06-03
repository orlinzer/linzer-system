use std::{io::Result, sync::mpsc, thread};

/// The Threaded struct hold the necessary components to handle thread
pub struct Threaded {
    /// The thread of the manager
    thread: Option<thread::JoinHandle<Result<()>>>,

    /// The channel used to communicate with the manager thread
    channel: (Option<mpsc::Sender<String>>, Option<mpsc::Receiver<String>>),
}

/// Implements the Threaded struct
impl Threaded {
    /// Creates a new Threaded struct
    pub fn new() -> Self {
        Self {
            thread: None,
            channel: (None, None),
        }
    }

    // pub fn start(&mut self) -> Result<()> {
    //     if self.thread.is_some() {
    //         return Err(std::io::Error::new(
    //             std::io::ErrorKind::AlreadyExists,
    //             "Thread already exists",
    //         ));
    //     }
    //
    //     let mut task = self
    //         .task
    //         .take()
    //         .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Task not found"))?;
    //     // self.initialize()?;
    //     task.initialize()?;
    //
    //     // let mut self_clone = self.clone();
    //
    //     let (sx, tx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    //     self.channel = (Some(sx), None);
    //     // self_clone.channel = (None, Some(tx));
    //
    //     log::info!("Thread is starting");
    //
    //     self.thread = Some(thread::spawn(move || {
    //         // let _rx = self_clone.channel.1.as_ref().unwrap();
    //         let _rx = tx;
    //
    //         loop {
    //             // match self_clone.main_loop() {
    //             match task.main_loop() {
    //                 Ok(result) => {
    //                     if result == -1 {
    //                         log::info!("Thread is stopping");
    //                         // self.task.finalize()?;
    //                         break Ok(());
    //                     }
    //                 }
    //                 Err(e) => {
    //                     break Err(e);
    //                 }
    //             }
    //         }
    //     }));
    //
    //     Ok(())
    // }
    //
    // pub fn stop(&mut self) -> Result<()> {
    //     if self.thread.is_none() {
    //         return Err(std::io::Error::new(
    //             std::io::ErrorKind::NotFound,
    //             "Thread not found",
    //         ));
    //     }
    //
    //     if let Some(sender) = &self.channel.0 {
    //         sender
    //             .send("stop".to_string())
    //             .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
    //     }
    //
    //     self.thread
    //         .take()
    //         .unwrap()
    //         .join()
    //         .expect("Thread panicked")?;
    //
    //     Ok(())
    // }
}

impl Clone for Threaded {
    fn clone(&self) -> Self {
        Threaded {
            thread: None,
            channel: (None, None),
        }
    }
}

/// Represents a threaded task
pub trait ThreadedTask: Clone + Send + 'static {
    fn get_threaded(&self) -> &mut Threaded;

    fn start(&mut self) -> Result<()> {
        log::info!("Thread is starting");

        let threaded = self.get_threaded();

        if threaded.thread.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Thread already exists",
            ));
        }

        let mut self_clone = self.clone();

        let (msx, mtx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let (tsx, ttx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        threaded.channel = (Some(msx), Some(ttx));

        log::info!("Thread is starting");

        threaded.thread = Some(thread::spawn(move || {
            let threaded_clone = self_clone.get_threaded();
            threaded_clone.channel = (Some(tsx), Some(mtx));

            self_clone.initialize()?;

            loop {
                match self_clone.main_loop() {
                    Ok(result) => {
                        if result == -1 {
                            log::info!("Thread is stopping");
                            // self.task.finalize()?;
                            break Ok(());
                        }
                    }
                    Err(e) => {
                        break Err(e);
                    }
                }
            }
        }));

        Ok(())
    }

    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;

    fn restart(&mut self) -> Result<()> {
        log::info!("Thread is restarting");
        self.stop()?;
        self.start()
    }

    /// Initializes the thread
    fn initialize(&mut self) -> Result<()> {
        log::info!("Thread is initializing");
        Ok(())
    }

    /// Starts the thread
    fn main_loop(&mut self) -> Result<i32> {
        log::info!("Thread is running");
        Ok(0)
    }

    /// Finalizes the thread
    fn finalize(&mut self) -> Result<()> {
        log::info!("Thread is finalizing");
        Ok(())
    }
}
