use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start_time = Instant::now();
    let duration = Duration::from_secs(120);

    loop {
        let elapsed_time = start_time.elapsed();
        let remaining_time = duration
            .checked_sub(elapsed_time)
            .unwrap_or_else(|| Duration::from_secs(0));

        if remaining_time.as_secs() == 0 {
            println!("Time's up!");
            break;
        }

        println!("{:?} seconds remaining...", format_duration(remaining_time));

        thread::sleep(Duration::from_secs(1));
    }
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = (seconds / 60) % 60;
    let hours = seconds / 3600;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds % 60)
}
