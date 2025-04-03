use chrono::Local;
use std::time::Duration;
use std::thread::sleep;
fn main() {

    loop {
        let t = Local::now();
        let time = t.format("%H:%M:%S").to_string();
        if time == "16:22:00" {
            println!("now")
        }
        sleep(Duration::from_millis(1000));
    }
}