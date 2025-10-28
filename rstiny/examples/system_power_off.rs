//! System Power-Off Example
//!
//! This example demonstrates a safe system shutdown sequence using the PMU driver.
//! It powers off peripheral domains before triggering system shutdown.
//!
//! WARNING: Running this will power off the system!
//!
//! To use this example, modify main.rs to call:
//! ```
//! test::test_system_power_off();
//! ```

use pmu_rk3588::{PowerDomain, RK3588Pmu, PMU_BASE_ADDR};

/// Example: Safe system power-off sequence
///
/// This demonstrates:
/// 1. Checking which domains are currently active
/// 2. Powering off safe peripheral domains
/// 3. Verifying power-off completion
/// 4. Calling system shutdown
pub fn example_system_power_off() {
    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    println!("=== System Power-Off Sequence ===\n");

    // Step 1: Define safe domains to power off
    let domains_to_shutdown = [
        PowerDomain::AUDIO,
        PowerDomain::SDMMC,
        PowerDomain::SDIO,
        PowerDomain::GMAC,
        PowerDomain::PCIE,
        PowerDomain::CRYPTO,
    ];

    // Step 2: Check current state
    println!("Current power states:");
    for domain in &domains_to_shutdown {
        let state = if pmu.is_power_on(*domain) { "ON" } else { "OFF" };
        println!("  {:?}: {}", domain, state);
    }
    println!();

    // Step 3: Power off all domains
    println!("Powering off domains...");
    match pmu.power_off_multiple(&domains_to_shutdown) {
        Ok(()) => println!("✓ All domains powered off successfully"),
        Err(e) => println!("✗ Error during power off: {:?}", e),
    }
    println!();

    // Step 4: Verify final state
    println!("Final power states:");
    for domain in &domains_to_shutdown {
        let state = if pmu.is_power_on(*domain) { "ON" } else { "OFF" };
        println!("  {:?}: {}", domain, state);
    }
    println!();

    // Step 5: System shutdown
    println!("Initiating system shutdown...");
    // axplat::power::system_off(); // Uncomment to actually power off
}

/// Simple power-off example (minimal version)
pub fn simple_power_off() {
    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    println!("Powering off peripheral domains...");

    // Power off safe domains
    let domains = [
        PowerDomain::GMAC,
    ];

    for domain in &domains {
        match pmu.power_off(*domain) {
            Ok(()) => println!("  ✓ {:?} powered off", domain),
            Err(e) => println!("  ✗ {:?} failed: {:?}", domain, e),
        }
    }

    println!("\nSystem shutdown...");
    // axplat::power::system_off();
}
