//! GPIO controller platform wrapper

use gpio_rk3588::{RK3588Gpio, Direction, Level};
use kspin::SpinNoIrq;

use crate::mem::phys_to_virt;

/// RK3588 GPIO base addresses (physical addresses)
pub const GPIO0_BASE_PADDR: usize = 0xFD8A0000;
pub const GPIO1_BASE_PADDR: usize = 0xFEC20000;
pub const GPIO2_BASE_PADDR: usize = 0xFEC30000;
pub const GPIO3_BASE_PADDR: usize = 0xFEC40000;
pub const GPIO4_BASE_PADDR: usize = 0xFEC50000;

/// GPIO controller
pub struct GpioController {
    banks: [SpinNoIrq<RK3588Gpio>; 5],
}

impl GpioController {
    /// Create a GPIO controller instance
    pub const fn new() -> Self {
        // Convert physical addresses to virtual addresses
        let gpio0_vaddr = phys_to_virt(axplat::mem::pa!(GPIO0_BASE_PADDR)).as_usize();
        let gpio1_vaddr = phys_to_virt(axplat::mem::pa!(GPIO1_BASE_PADDR)).as_usize();
        let gpio2_vaddr = phys_to_virt(axplat::mem::pa!(GPIO2_BASE_PADDR)).as_usize();
        let gpio3_vaddr = phys_to_virt(axplat::mem::pa!(GPIO3_BASE_PADDR)).as_usize();
        let gpio4_vaddr = phys_to_virt(axplat::mem::pa!(GPIO4_BASE_PADDR)).as_usize();

        Self {
            banks: [
                SpinNoIrq::new(RK3588Gpio::new(gpio0_vaddr)),
                SpinNoIrq::new(RK3588Gpio::new(gpio1_vaddr)),
                SpinNoIrq::new(RK3588Gpio::new(gpio2_vaddr)),
                SpinNoIrq::new(RK3588Gpio::new(gpio3_vaddr)),
                SpinNoIrq::new(RK3588Gpio::new(gpio4_vaddr)),
            ],
        }
    }

    /// Set pin to output mode and set level
    ///
    /// # Parameters
    /// - `bank`: GPIO bank number (0-4)
    /// - `pin`: Pin number within bank (0-31)
    /// - `level`: Output level
    pub fn set_output(&self, bank: usize, pin: u8, level: Level) {
        if bank >= 5 {
            log::error!("Invalid GPIO bank: {}", bank);
            return;
        }

        let gpio = self.banks[bank].lock();
        gpio.set_direction(pin, Direction::Output);
        gpio.set_value(pin, level);
    }

    /// Set pin to input mode and read level
    ///
    /// # Parameters
    /// - `bank`: GPIO bank number (0-4)
    /// - `pin`: Pin number within bank (0-31)
    ///
    /// # Returns
    /// Pin level
    pub fn read_input(&self, bank: usize, pin: u8) -> Level {
        if bank >= 5 {
            log::error!("Invalid GPIO bank: {}", bank);
            return Level::Low;
        }

        let gpio = self.banks[bank].lock();
        gpio.set_direction(pin, Direction::Input);
        gpio.get_value(pin)
    }

    /// Toggle output pin level
    pub fn toggle(&self, bank: usize, pin: u8) {
        if bank >= 5 {
            return;
        }

        let gpio = self.banks[bank].lock();
        gpio.toggle(pin);
    }

    /// Set output by absolute pin number
    ///
    /// Pin calculation: absolute_pin = bank * 32 + pin
    pub fn set_output_abs(&self, absolute_pin: u32, level: Level) {
        let bank = (absolute_pin / 32) as usize;
        let pin = (absolute_pin % 32) as u8;
        self.set_output(bank, pin, level);
    }

    /// Read input by absolute pin number
    pub fn read_input_abs(&self, absolute_pin: u32) -> Level {
        let bank = (absolute_pin / 32) as usize;
        let pin = (absolute_pin % 32) as u8;
        self.read_input(bank, pin)
    }

    /// Initialize GPIO controller
    ///
    /// Verify that all GPIO banks are accessible
    pub fn init(&self) {
        log::info!("Initializing GPIO controller...");

        for (i, bank) in self.banks.iter().enumerate() {
            let gpio = bank.lock();
            let ver_id = gpio.get_version_id();
            log::info!("GPIO{} Version ID: 0x{:08x}", i, ver_id);

            // RK3588 version ID should be 0x0101157C
            if ver_id != 0x0101157C {
                log::warn!("GPIO{} unexpected version ID: 0x{:08x}, expected 0x0101157C", i, ver_id);
            }
        }

        log::info!("GPIO controller initialized");
    }
}

/// Global GPIO controller instance
static GPIO_CONTROLLER: GpioController = GpioController::new();

/// Get global GPIO controller reference
pub fn gpio_controller() -> &'static GpioController {
    &GPIO_CONTROLLER
}

/// Initialize GPIO system
pub fn init() {
    GPIO_CONTROLLER.init();
}
