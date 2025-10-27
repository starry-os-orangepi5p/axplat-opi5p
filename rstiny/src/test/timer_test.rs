//! Timer functionality tests for OrangePi 5 Plus
//!
//! This test module demonstrates Timer operations to blink LEDs at precise intervals:
//! - Blue LED: GPIO3 pin 6 - blinks every 1 second using timer
//! - Uses TIMER_PMU (2-channel timer) with 24MHz clock

use axplat_aarch64_opi5p::gpio_controller;
use gpio_rk3588::Level;
use timer_rk3588::{RK3588Timer, TimerMode};

/// GPIO3 bank index
const GPIO3_BANK: usize = 3;

/// Blue LED pin (GPIO3 pin 6)
const BLUE_LED_PIN: u8 = 6;

/// TIMER_PMU base address
const TIMER_PMU_BASE: usize = 0xFD8F0000;

/// Timer clock frequency (24MHz)
const TIMER_FREQ: u64 = 24_000_000;

/// 1 second interval in timer ticks
const ONE_SECOND_TICKS: u64 = TIMER_FREQ;

/// Simple busy-wait delay using timer
fn timer_delay_seconds(timer: &RK3588Timer, seconds: u64) {
    let target_ticks = seconds * TIMER_FREQ;

    // Disable timer
    timer.disable();

    // Load count value
    timer.load_count(target_ticks);

    // Set to user-defined count mode (one-shot)
    timer.set_mode(TimerMode::UserDefined);

    // Disable interrupt for polling mode
    timer.disable_interrupt();

    // Enable timer
    timer.enable();

    // Wait for timer to reach target (for increment timer, wait until it reaches load_count)
    // For decrement timer, wait until it reaches 0
    let initial_value = timer.current_value();

    // Poll until timer completes
    loop {
        let current = timer.current_value();

        // For increment timer: starts at 0, counts up to target
        // For decrement timer: starts at target, counts down to 0
        // In both cases, the timer stops when reaching the target in user-defined mode

        // Check if timer has stopped (value stays constant)
        if current == initial_value && timer.current_value() == current {
            // Give it a moment to start
            for _ in 0..1000 {
                core::hint::spin_loop();
            }
            continue;
        }

        // For increment: check if reached target
        if current >= target_ticks {
            break;
        }

        // For decrement: check if reached 0
        if current == 0 && initial_value != 0 {
            break;
        }

        core::hint::spin_loop();
    }

    // Disable timer
    timer.disable();
}

/// Test timer-based LED blinking (1 second intervals)
pub fn test_timer_led_blink() {
    info!("Testing timer-based LED blinking (1s interval)...");

    let gpio = gpio_controller();
    let mut timer = RK3588Timer::new(TIMER_PMU_BASE, 0);

    // Initialize timer and detect type
    if !timer.init() {
        error!("Failed to initialize timer");
        return;
    }

    info!("Timer initialized, type: {:?}", timer.timer_type());
    info!("Timer revision: 0x{:08X}", timer.get_revision());

    // Blink LED 10 times with 1-second intervals
    for i in 0..10 {
        // Turn on blue LED
        gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::High);
        info!("Blink #{} - LED ON", i + 1);

        // Wait 1 second using timer
        timer_delay_seconds(&timer, 1);

        // Turn off blue LED
        gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);
        info!("Blink #{} - LED OFF", i + 1);

        // Wait 1 second using timer
        timer_delay_seconds(&timer, 1);
    }

    // Ensure LED is off
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);

    info!("Timer-based LED blink test completed");
}

/// Test timer accuracy by measuring multiple intervals
pub fn test_timer_accuracy() {
    info!("Testing timer accuracy...");

    let mut timer = RK3588Timer::new(TIMER_PMU_BASE, 0);

    if !timer.init() {
        error!("Failed to initialize timer");
        return;
    }

    // Test different time intervals
    let test_durations = [1, 2, 5]; // seconds

    for &duration in &test_durations {
        info!("Testing {} second delay...", duration);

        // Configure timer for free-running mode to read current time
        timer.disable();
        timer.load_count(u64::MAX); // Maximum count for free-running
        timer.set_mode(TimerMode::FreeRunning);
        timer.enable();

        let start = timer.current_value();
        timer_delay_seconds(&timer, duration);
        let end = timer.current_value();

        let elapsed_ticks = if end > start {
            end - start
        } else {
            // Handle overflow (unlikely with u64::MAX)
            (u64::MAX - start) + end
        };

        let elapsed_seconds = elapsed_ticks / TIMER_FREQ;
        info!("  Expected: {} seconds, Measured: {} seconds (ticks: {})",
              duration, elapsed_seconds, elapsed_ticks);
    }

    timer.stop();
    info!("Timer accuracy test completed");
}

/// Test timer interrupt functionality (if interrupts are configured)
pub fn test_timer_interrupt() {
    info!("Testing timer interrupt configuration...");

    let mut timer = RK3588Timer::new(TIMER_PMU_BASE, 0);

    if !timer.init() {
        error!("Failed to initialize timer");
        return;
    }

    // Configure timer with interrupt enabled
    timer.start(ONE_SECOND_TICKS, TimerMode::FreeRunning, true);

    info!("Timer started with interrupt enabled");
    info!("Interrupt status: {}", timer.get_interrupt_status());

    // Wait a bit
    for _ in 0..10_000_000 {
        core::hint::spin_loop();
    }

    // Check interrupt status
    if timer.get_interrupt_status() {
        info!("Timer interrupt triggered!");
        timer.clear_interrupt();
        info!("Interrupt cleared");
    } else {
        info!("No interrupt detected (may need interrupt handler setup)");
    }

    timer.stop();
    info!("Timer interrupt test completed");
}

/// Test timer in free-running mode
pub fn test_timer_freerunning() {
    info!("Testing timer free-running mode...");

    let gpio = gpio_controller();
    let mut timer = RK3588Timer::new(TIMER_PMU_BASE, 0);

    if !timer.init() {
        error!("Failed to initialize timer");
        return;
    }

    // Start timer in free-running mode with 1-second period
    timer.start(ONE_SECOND_TICKS, TimerMode::FreeRunning, false);

    info!("Timer started in free-running mode (1s period)");

    let mut last_value = timer.current_value();
    let mut toggle_state = false;

    // Monitor timer for 10 seconds, toggle LED each second
    for i in 0..10 {
        // Wait until timer wraps around (for increment) or reloads (for decrement)
        loop {
            let current_value = timer.current_value();

            // Detect rollover/reload
            if current_value < last_value {
                // Timer has wrapped around
                break;
            }

            last_value = current_value;
            core::hint::spin_loop();
        }

        // Toggle LED
        toggle_state = !toggle_state;
        gpio.set_output(
            GPIO3_BANK,
            BLUE_LED_PIN,
            if toggle_state { Level::High } else { Level::Low }
        );

        info!("Second {} - LED {}", i + 1, if toggle_state { "ON" } else { "OFF" });
        last_value = timer.current_value();
    }

    // Cleanup
    timer.stop();
    gpio.set_output(GPIO3_BANK, BLUE_LED_PIN, Level::Low);

    info!("Free-running timer test completed");
}

/// Run all timer tests
pub fn run_all_timer_tests() {
    info!("=== Starting Timer Tests ===");

    test_timer_led_blink();
    test_timer_accuracy();
    test_timer_interrupt();
    test_timer_freerunning();

    info!("=== All Timer Tests Completed ===");
}

/// Simple timer demo - blink blue LED every 1 second
pub fn timer_blink_demo() {
    info!("Running timer blink demo (10 blinks)...");
    test_timer_led_blink();
    info!("Timer blink demo completed");
}
