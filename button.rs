use rust_gpiozero::*;

pub fn test_button(gpio_num: u8) -> bool {
    // Create a button which is attached to Pin 17
    let button = Button::new(gpio_num);
    button.value()
}
