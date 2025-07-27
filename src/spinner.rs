use crate::utils::sleep;
use std::{
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

const SPINNER: [&'static str; 4] = ["|", "/", "-", "\\"];

pub struct Spinner {
    running: Arc<AtomicBool>,
}

impl Spinner {
    pub fn new() -> Self {
        Spinner {
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&mut self, message: &str) {
        self.running.store(true, Ordering::SeqCst);

        let running = self.running.clone();
        let message = message.to_string();

        thread::spawn(move || {
            let mut index = 0;

            while running.load(Ordering::SeqCst) {
                print!("\r[{}] {}", SPINNER[index % SPINNER.len()], message);
                io::stdout().flush().unwrap();

                index += 1;
                sleep(100);
            }
        });
    }

    pub fn stop(&mut self, message: Result<&str, &str>) {
        self.running.store(false, Ordering::SeqCst);
        sleep(120); // let spinner clear

        match message {
            Ok(ok_message) => {
                println!("\r✅ {ok_message}");
            }
            Err(err_message) => {
                println!("\r❌ {err_message}");
            }
        }

        io::stdout().flush().unwrap();
    }
}
