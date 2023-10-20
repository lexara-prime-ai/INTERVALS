use std::time::{Duration, Instant};

fn main() {
    let interval = Duration::from_secs(1); // Set interval to 1 second
    let mut last_run = Instant::now();

    loop {
        let now = Instant::now();
        if now.duration_since(last_run) >= interval {
            // Do something periodically
            println!("Doing something periodically");

            last_run = now; // Update the last run time
        }

        // Optional, add some other work or sleep to avoid busy-waiting
        // std::thread::sleep(Duration::from_millis(100));
    }
}
