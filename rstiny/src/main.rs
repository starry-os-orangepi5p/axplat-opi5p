#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate log;

extern crate alloc;
extern crate axplat_aarch64_opi5p;

mod config;
mod utils;

mod test;

fn init_kernel(cpu_id: usize, arg: usize) {
    // Initialize trap, console, time.
    axplat::init::init_early(cpu_id, arg);

    // Initialize platform peripherals (not used in this example).
    axplat::init::init_later(cpu_id, arg);
}

#[axplat::main]
pub fn rust_main(cpu_id: usize, arg: usize) -> ! {
    utils::mem::clear_bss();
    init_kernel(cpu_id, arg);

    axplat::console_println!("Hello, ArceOS!");

    utils::logging::log_init();

    info!("Logging initialized. This is an info message.");

    test::run_allocator_tests();

    // Run GPIO/LED tests
    test::run_all_gpio_tests();

    // Run GICv3 driver tests
    test::run_all_gicv3_tests();

    // Run GICv3 hardware integration test (requires actual hardware)
    // Uncomment to test with real hardware:
    // test::run_gicv3_hardware_test();

    // Run Timer tests - blink blue LED every 1 second
    info!("Starting timer tests...");
    test::timer_blink_demo();

    axplat::power::system_off()
}

#[unsafe(no_mangle)]
pub extern "C" fn __axplat_secondary_main(cpu_id: usize) -> ! {
    axplat::init::init_early(cpu_id, 0);
    axplat::init::init_later(cpu_id, 0);

    axplat::console_println!("Secondary CPU {} started", cpu_id);

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(all(target_os = "none", not(test)))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    axplat::console_println!("{info}");
    axplat::power::system_off()
}
