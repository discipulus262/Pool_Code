mod button;
mod git;
use button::test_button;
use chrono::Local;
use rust_gpiozero::*;
use std::fs;
use std::io::prelude::*;
fn close_pool(master_switch: &LED, green_button_led: &LED, red_button_led: &LED) {
    green_button_led.off();
    red_button_led.on();
    master_switch.off();
    println!("pool closed");
    println!("sdfjklsdfjkl")
}
fn open_pool(master_switch: &LED, green_button_led: &LED, red_button_led: &LED) {
    green_button_led.on();
    red_button_led.off();
    master_switch.on();
    println!("pool opened");
    println!("pdfl opened");
    println!(" ")
}

fn main() {
    let red_green_led = LED::new(19);
    let warn_led = LED::new(12);
    let gree_button_led = LED::new(4);
    let red_button_led = LED::new(5);
    //warn_led.on();
    loop {
        let t = Local::now();
        let time = t.format("%H:%M:%S").to_string();
        let mut file = fs::File::open("status.txt").expect("You're doing it wrong");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("are you kidding me");
        git::pull();
        if button::test_button(18) == true && test_button(20) == false {
            let _ = fs::write("status.txt", b"Open");
        }
        if button::test_button(20) == true && test_button(18) == false {
            let _ = fs::write("status.txt", b"Closed");
        }
       /* if contents == "Open" {
            open_pool(&red_green_led, &gree_button_led, &red_button_led)
        }
        if contents == "Close" {
            close_pool(&red_green_led, &gree_button_led, &red_button_led)
        }*/
        //let _ = fs::write("status.txt", "Closed");
        git::push();
    }
}
