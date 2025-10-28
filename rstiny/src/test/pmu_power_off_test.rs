//! Dedicated PMU power-off tests for RK3588
//!
//! This module provides focused tests for power domain power-off functionality.
//! These tests are designed to be safe and non-disruptive to critical system functions.

use pmu_rk3588::{PowerDomain, RK3588Pmu, PMU_BASE_ADDR, PmuError};

/// Delay helper
fn delay_ms(ms: usize) {
    for _ in 0..(ms * 100_000) {
        core::hint::spin_loop();
    }
}

/// Simple power-off example (minimal version)
pub fn simple_power_off() {
    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    info!("Powering off peripheral domains...");

    // Power off safe domains
    let domains = [
        PowerDomain::GMAC,
    ];

    for domain in &domains {
        match pmu.power_off(*domain) {
            Ok(()) => info!("  ✓ {:?} powered off", domain),
            Err(e) => info!("  ✗ {:?} failed: {:?}", domain, e),
        }
    }

    info!("\nSystem shutdown...");
    // axplat::power::system_off();
}

/// Test 1: Basic power off functionality
pub fn test_basic_power_off() {
    info!("\n╔══════════════════════════════════════════════════════╗");
    info!("║  Test 1: Basic Power Off - AUDIO Domain             ║");
    info!("╚══════════════════════════════════════════════════════╝");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);
    let domain = PowerDomain::AUDIO;

    // Check initial state
    let is_on = pmu.is_power_on(domain);
    info!("Initial state: {}", if is_on { "ON" } else { "OFF" });

    if !is_on {
        info!("Domain already OFF, test skipped");
        return;
    }

    // Perform power off
    info!("Initiating power off...");
    match pmu.power_off(domain) {
        Ok(()) => {
            info!("✓ SUCCESS: Power off completed");
            delay_ms(10);

            // Verify
            let final_state = pmu.is_power_on(domain);
            if !final_state {
                info!("✓ VERIFIED: Domain is OFF");
            } else {
                warn!("✗ FAIL: Domain still appears ON");
            }
        }
        Err(e) => {
            error!("✗ FAIL: Power off failed - {:?}", e);
        }
    }

    info!("Test completed\n");
}

/// Test 2: Power off with state verification
pub fn test_power_off_with_verification() {
    info!("\n╔══════════════════════════════════════════════════════╗");
    info!("║  Test 2: Power Off with State Verification          ║");
    info!("╚══════════════════════════════════════════════════════╝");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);
    let domain = PowerDomain::SDMMC;

    info!("Testing domain: SDMMC");

    // Record initial state
    let initial = pmu.is_power_on(domain);
    info!("  Step 1: Initial state = {}", if initial { "ON" } else { "OFF" });

    // Power off
    info!("  Step 2: Executing power off...");
    let result = pmu.power_off(domain);

    // Check result
    info!("  Step 3: Checking result...");
    match result {
        Ok(()) => info!("    ✓ Power off command succeeded"),
        Err(e) => {
            warn!("    ✗ Power off command failed: {:?}", e);
            return;
        }
    }

    delay_ms(5);

    // Verify state changed
    info!("  Step 4: Verifying state change...");
    let final_state = pmu.is_power_on(domain);
    info!("    Final state = {}", if final_state { "ON" } else { "OFF" });

    if initial && !final_state {
        info!("✓ SUCCESS: State transitioned from ON to OFF");
    } else if !initial {
        info!("  Note: Domain was already OFF");
    } else {
        warn!("✗ FAIL: State did not change as expected");
    }

    info!("Test completed\n");
}

/// Test 3: Sequential power off operations
pub fn test_sequential_power_off() {
    info!("\n╔══════════════════════════════════════════════════════╗");
    info!("║  Test 3: Sequential Power Off Operations            ║");
    info!("╚══════════════════════════════════════════════════════╝");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    let domains = [
        ("AUDIO", PowerDomain::AUDIO),
        ("SDMMC", PowerDomain::SDMMC),
    ];

    for (name, domain) in domains.iter() {
        info!("Processing {}", name);

        let initial = pmu.is_power_on(*domain);
        info!("  Initial: {}", if initial { "ON" } else { "OFF" });

        if initial {
            match pmu.power_off(*domain) {
                Ok(()) => {
                    info!("  ✓ Power off successful");
                    delay_ms(5);

                    let final_state = pmu.is_power_on(*domain);
                    if !final_state {
                        info!("  ✓ Verified OFF");
                    } else {
                        warn!("  ✗ Still ON");
                    }
                }
                Err(e) => {
                    warn!("  ✗ Failed: {:?}", e);
                }
            }
        } else {
            info!("  Already OFF, skipped");
        }

        info!("");
    }

    info!("Test completed\n");
}

/// Test 4: Power off with timeout detection
pub fn test_power_off_timeout() {
    info!("\n╔══════════════════════════════════════════════════════╗");
    info!("║  Test 4: Power Off with Timeout Detection           ║");
    info!("╚══════════════════════════════════════════════════════╝");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);
    let domain = PowerDomain::AUDIO;

    info!("Testing timeout handling...");

    match pmu.power_off(domain) {
        Ok(()) => {
            info!("✓ Power off completed within timeout");
        }
        Err(PmuError::Timeout) => {
            warn!("✗ Operation timed out (this may indicate a hardware issue)");
        }
        Err(PmuError::BiuIdleFailed) => {
            warn!("✗ BIU idle request failed");
        }
        Err(e) => {
            error!("✗ Unexpected error: {:?}", e);
        }
    }

    info!("Test completed\n");
}

/// Test 5: Idempotent power off (calling twice)
pub fn test_idempotent_power_off() {
    info!("\n╔══════════════════════════════════════════════════════╗");
    info!("║  Test 5: Idempotent Power Off (Call Twice)          ║");
    info!("╚══════════════════════════════════════════════════════╝");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);
    let domain = PowerDomain::AUDIO;

    info!("First power off call...");
    match pmu.power_off(domain) {
        Ok(()) => info!("  ✓ First call succeeded"),
        Err(e) => warn!("  First call result: {:?}", e),
    }

    delay_ms(5);

    info!("Second power off call (domain should already be OFF)...");
    match pmu.power_off(domain) {
        Ok(()) => info!("  ✓ Second call succeeded (idempotent)"),
        Err(e) => warn!("  Second call result: {:?}", e),
    }

    info!("Test completed\n");
}

/// Test 6: Batch power off operation
pub fn test_batch_power_off() {
    info!("\n╔══════════════════════════════════════════════════════╗");
    info!("║  Test 6: Batch Power Off Operation                  ║");
    info!("╚══════════════════════════════════════════════════════╝");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    let domains = [
        PowerDomain::AUDIO,
        PowerDomain::SDMMC,
    ];

    info!("Powering off {} domains in batch...", domains.len());

    match pmu.power_off_multiple(&domains) {
        Ok(()) => {
            info!("✓ Batch power off succeeded");
            delay_ms(10);

            info!("Verifying final states:");
            for domain in &domains {
                let state = if pmu.is_power_on(*domain) { "ON" } else { "OFF" };
                info!("  {:?}: {}", domain, state);
            }
        }
        Err(e) => {
            error!("✗ Batch power off failed: {:?}", e);
        }
    }

    info!("Test completed\n");
}

/// Run all power-off focused tests
pub fn run_all_power_off_tests() {
    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║                                                      ║");
    info!("║     RK3588 PMU Power-Off Test Suite                 ║");
    info!("║                                                      ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    test_basic_power_off();
    test_power_off_with_verification();
    test_sequential_power_off();
    test_power_off_timeout();
    test_idempotent_power_off();
    test_batch_power_off();

    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║                                                      ║");
    info!("║     All Power-Off Tests Completed                   ║");
    info!("║                                                      ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("\n");
}

/// Quick power-off demo (single test)
pub fn quick_power_off_demo() {
    info!("\n=== Quick Power-Off Demo ===\n");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);
    let domain = PowerDomain::AUDIO;

    info!("Powering off AUDIO domain...");

    match pmu.power_off(domain) {
        Ok(()) => {
            info!("✓ Success!");
            delay_ms(5);

            if !pmu.is_power_on(domain) {
                info!("✓ Verified: AUDIO is now OFF");
            }
        }
        Err(e) => {
            error!("✗ Failed: {:?}", e);
        }
    }

    info!("\nDemo completed\n");
}

/// System power-off test - Powers off all non-critical domains to prepare for system shutdown
///
/// This test demonstrates a safe system power-down sequence by:
/// 1. Identifying all safe-to-power-off domains
/// 2. Powering them off in a safe order
/// 3. Reporting the final power state
/// 4. Triggering system shutdown
///
/// WARNING: This will power off the system!
pub fn test_system_power_off() {
    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║                                                      ║");
    info!("║         SYSTEM POWER-OFF TEST                       ║");
    info!("║                                                      ║");
    info!("║  This test will power off the system!              ║");
    info!("║                                                      ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    // Define safe domains to power off before system shutdown
    // These are peripheral domains that can be safely powered down
    let safe_domains_to_shutdown = [
        ("AUDIO", PowerDomain::AUDIO),
        ("SDMMC", PowerDomain::SDMMC),
        ("SDIO", PowerDomain::SDIO),
        ("GMAC", PowerDomain::GMAC),
        ("PCIE", PowerDomain::PCIE),
        ("CRYPTO", PowerDomain::CRYPTO),
    ];

    info!("Step 1: Checking initial power states...");
    info!("─────────────────────────────────────────────────────");
    let mut active_domains = 0;
    for (name, domain) in safe_domains_to_shutdown.iter() {
        let is_on = pmu.is_power_on(*domain);
        let status = if is_on { "ON " } else { "OFF" };
        info!("  {:<15} : {}", name, status);
        if is_on {
            active_domains += 1;
        }
    }
    info!("  Total active domains: {}", active_domains);
    info!("");

    delay_ms(500);

    info!("Step 2: Powering off peripheral domains...");
    info!("─────────────────────────────────────────────────────");

    let mut success_count = 0;
    let mut skip_count = 0;
    let mut fail_count = 0;

    for (name, domain) in safe_domains_to_shutdown.iter() {
        if !pmu.is_power_on(*domain) {
            info!("  {:<15} : Already OFF (skipped)", name);
            skip_count += 1;
            continue;
        }

        info!("  {:<15} : Powering off...", name);
        match pmu.power_off(*domain) {
            Ok(()) => {
                delay_ms(10);
                if !pmu.is_power_on(*domain) {
                    info!("  {:<15} : ✓ OFF", name);
                    success_count += 1;
                } else {
                    warn!("  {:<15} : ✗ Failed to verify OFF state", name);
                    fail_count += 1;
                }
            }
            Err(e) => {
                warn!("  {:<15} : ✗ Error: {:?}", name, e);
                fail_count += 1;
            }
        }
    }

    info!("");
    info!("Power-off summary:");
    info!("  ✓ Successfully powered off: {}", success_count);
    info!("  - Already off (skipped):    {}", skip_count);
    if fail_count > 0 {
        warn!("  ✗ Failed:                   {}", fail_count);
    }
    info!("");

    delay_ms(500);

    info!("Step 3: Final system power state...");
    info!("─────────────────────────────────────────────────────");
    for (name, domain) in safe_domains_to_shutdown.iter() {
        let status = if pmu.is_power_on(*domain) { "ON " } else { "OFF" };
        info!("  {:<15} : {}", name, status);
    }
    info!("");

    delay_ms(500);

    info!("Step 4: Initiating system shutdown...");
    info!("─────────────────────────────────────────────────────");
    info!("  PMU power state: prepared for shutdown");
    info!("  Calling system power off...");
    info!("");

    info!("╔══════════════════════════════════════════════════════╗");
    info!("║                                                      ║");
    info!("║         System shutting down...                     ║");
    info!("║         Goodbye!                                     ║");
    info!("║                                                      ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    delay_ms(1000);

    // Trigger actual system power off
    axplat::power::system_off();
}

/// Safe system shutdown test - powers off peripherals then shuts down
/// This is the same as test_system_power_off but with a clearer name
pub fn system_shutdown_test() {
    test_system_power_off();
}
