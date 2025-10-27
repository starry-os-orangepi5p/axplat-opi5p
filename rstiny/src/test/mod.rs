mod allocator;
pub mod gpio_test;
pub mod gicv3_test;
pub mod timer_test;

pub use allocator::run_allocator_tests;
pub use gpio_test::run_all_gpio_tests;
pub use gicv3_test::{run_all_gicv3_tests, run_gicv3_hardware_test};
pub use timer_test::{run_all_timer_tests, timer_blink_demo};
