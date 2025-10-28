//! GPIO and LED functionality tests for OrangePi 5 Plus
//!
//! This test module demonstrates GPIO operations using the board's LEDs:
//! - Blue LED: GPIO3 pin 6
//! - Green LED: GPIO3 pin 9

use axplat_aarch64_opi5p::gpio_controller;
use gpio_rk3588::Level;

/// GPIO3 bank index
const GPIO3_BANK: usize = 3;

/// Blue LED pin (GPIO3 pin 6)
const BLUE_LED_PIN: u8 = 6;

/// Green LED pin (GPIO3 pin 9)
const GREEN_LED_PIN: u8 = 9;

/// Simple delay loop
fn delay(count: usize) {
    for _ in 0..count {
        core::hint::spin_loop();
    }
}

/// Test GPIO output functionality using the blue LED
pub fn test_gpio_output() {
    info!("Testing GPIO output with Blue LED (GPIO3 pin 6)...");

    let gpio = gpio_controller();

    // Turn on blue LED
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::High);
    info!("Blue LED set to HIGH");
    delay(5_000_000);

    // Turn off blue LED
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);
    info!("Blue LED set to LOW");
    delay(5_000_000);

    info!("GPIO output test completed");
}

/// Test GPIO toggle functionality
pub fn test_gpio_toggle() {
    info!("Testing GPIO toggle with Green LED (GPIO3 pin 9)...");

    let gpio = gpio_controller();

    // Toggle green LED 5 times
    for i in 0..5 {
        gpio.toggle(GPIO3_BANK, GREEN_LED_PIN);
        info!("Green LED toggle #{}", i + 1);
        delay(3_000_000);
    }

    // Ensure LED is off
    gpio.set_output(GPIO3_BANK, GREEN_LED_PIN, Level::Low);
    info!("GPIO toggle test completed");
}

/// Test both LEDs in a pattern (blink alternately)
pub fn test_led_pattern() {
    info!("Testing LED pattern (alternate blinking)...");

    let gpio = gpio_controller();

    // Blink LEDs alternately 3 times
    for i in 0..3 {
        info!("Pattern iteration {}", i + 1);

        // Blue on, Green off
        gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::High);
        gpio.set_output(GPIO3_BANK, GREEN_LED_PIN, Level::Low);
        delay(2_000_000);

        // Blue off, Green on
        gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);
        gpio.set_output(GPIO3_BANK, GREEN_LED_PIN, Level::High);
        delay(2_000_000);
    }

    // Turn both off
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);
    gpio.set_output(GPIO3_BANK, GREEN_LED_PIN, Level::Low);

    info!("LED pattern test completed");
}

/// Test all LEDs simultaneously
pub fn test_led_simultaneous() {
    info!("Testing both LEDs simultaneously...");

    let gpio = gpio_controller();

    // Both LEDs on
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::High);
    gpio.set_output(GPIO3_BANK, GREEN_LED_PIN, Level::High);
    info!("Both LEDs ON");
    delay(5_000_000);

    // Both LEDs off
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);
    gpio.set_output(GPIO3_BANK, GREEN_LED_PIN, Level::Low);
    info!("Both LEDs OFF");

    info!("Simultaneous LED test completed");
}

/// Run all GPIO tests
pub fn run_all_gpio_tests() {
    info!("=== Starting GPIO/LED Tests ===");

    test_gpio_output();
    test_gpio_toggle();
    test_led_pattern();
    test_led_simultaneous();

    info!("=== All GPIO/LED Tests Completed ===");
}

/// Simple LED blink demo
pub fn led_blink_demo() {
    info!("Running LED blink demo (Ctrl+C to stop)...");

    let gpio = gpio_controller();

    // Blink blue LED indefinitely (for demo purposes, limited to 10 iterations)
    for i in 0..10 {
        gpio.toggle(GPIO3_BANK, BLUE_LED_PIN);
        info!("Blink {}", i + 1);
        delay(5_000_000);
    }

    // Cleanup
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);
    info!("LED blink demo completed");
}
