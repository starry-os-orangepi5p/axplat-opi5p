//! GPIO 控制器平台封装

use gpio_rk3588::{RK3588Gpio, Direction, Level};
use kspin::SpinLock;

use crate::mem::phys_to_virt;

/// RK3588 GPIO 基地址（物理地址）
pub const GPIO0_BASE_PADDR: usize = 0xFD8A0000;
pub const GPIO1_BASE_PADDR: usize = 0xFEC20000;
pub const GPIO2_BASE_PADDR: usize = 0xFEC30000;
pub const GPIO3_BASE_PADDR: usize = 0xFEC40000;
pub const GPIO4_BASE_PADDR: usize = 0xFEC50000;

/// GPIO 控制器
pub struct GpioController {
    banks: [SpinLock<RK3588Gpio>; 5],
}

impl GpioController {
    /// 创建 GPIO 控制器实例
    pub const fn new() -> Self {
        // Convert physical addresses to virtual addresses
        let gpio0_vaddr = phys_to_virt(axplat::mem::pa!(GPIO0_BASE_PADDR)).as_usize();
        let gpio1_vaddr = phys_to_virt(axplat::mem::pa!(GPIO1_BASE_PADDR)).as_usize();
        let gpio2_vaddr = phys_to_virt(axplat::mem::pa!(GPIO2_BASE_PADDR)).as_usize();
        let gpio3_vaddr = phys_to_virt(axplat::mem::pa!(GPIO3_BASE_PADDR)).as_usize();
        let gpio4_vaddr = phys_to_virt(axplat::mem::pa!(GPIO4_BASE_PADDR)).as_usize();

        Self {
            banks: [
                SpinLock::new(RK3588Gpio::new(gpio0_vaddr)),
                SpinLock::new(RK3588Gpio::new(gpio1_vaddr)),
                SpinLock::new(RK3588Gpio::new(gpio2_vaddr)),
                SpinLock::new(RK3588Gpio::new(gpio3_vaddr)),
                SpinLock::new(RK3588Gpio::new(gpio4_vaddr)),
            ],
        }
    }

    /// 设置引脚为输出模式并设置电平
    ///
    /// # 参数
    /// - `bank`: GPIO bank 编号 (0-4)
    /// - `pin`: bank 内引脚编号 (0-31)
    /// - `level`: 输出电平
    pub fn set_output(&self, bank: usize, pin: u8, level: Level) {
        if bank >= 5 {
            log::error!("Invalid GPIO bank: {}", bank);
            return;
        }

        let gpio = self.banks[bank].lock();
        gpio.set_direction(pin, Direction::Output);
        gpio.set_value(pin, level);
    }

    /// 设置引脚为输入模式并读取电平
    ///
    /// # 参数
    /// - `bank`: GPIO bank 编号 (0-4)
    /// - `pin`: bank 内引脚编号 (0-31)
    ///
    /// # 返回
    /// 引脚电平
    pub fn read_input(&self, bank: usize, pin: u8) -> Level {
        if bank >= 5 {
            log::error!("Invalid GPIO bank: {}", bank);
            return Level::Low;
        }

        let gpio = self.banks[bank].lock();
        gpio.set_direction(pin, Direction::Input);
        gpio.get_value(pin)
    }

    /// 切换输出引脚电平
    pub fn toggle(&self, bank: usize, pin: u8) {
        if bank >= 5 {
            return;
        }

        let gpio = self.banks[bank].lock();
        gpio.toggle(pin);
    }

    /// 通过绝对引脚号设置输出
    ///
    /// 引脚计算: absolute_pin = bank * 32 + pin
    pub fn set_output_abs(&self, absolute_pin: u32, level: Level) {
        let bank = (absolute_pin / 32) as usize;
        let pin = (absolute_pin % 32) as u8;
        self.set_output(bank, pin, level);
    }

    /// 通过绝对引脚号读取输入
    pub fn read_input_abs(&self, absolute_pin: u32) -> Level {
        let bank = (absolute_pin / 32) as usize;
        let pin = (absolute_pin % 32) as u8;
        self.read_input(bank, pin)
    }

    /// 初始化 GPIO 控制器
    ///
    /// 验证所有 GPIO banks 是否可访问
    pub fn init(&self) {
        log::info!("Initializing GPIO controller...");

        for (i, bank) in self.banks.iter().enumerate() {
            let gpio = bank.lock();
            let ver_id = gpio.get_version_id();
            log::info!("GPIO{} Version ID: 0x{:08x}", i, ver_id);

            // RK3588 的版本 ID 应该是 0x0101157C
            if ver_id != 0x0101157C {
                log::warn!("GPIO{} unexpected version ID: 0x{:08x}, expected 0x0101157C", i, ver_id);
            }
        }

        log::info!("GPIO controller initialized");
    }
}

/// 全局 GPIO 控制器实例
static GPIO_CONTROLLER: GpioController = GpioController::new();

/// 获取全局 GPIO 控制器引用
pub fn gpio_controller() -> &'static GpioController {
    &GPIO_CONTROLLER
}

/// 初始化 GPIO 系统
pub fn init() {
    GPIO_CONTROLLER.init();
}
