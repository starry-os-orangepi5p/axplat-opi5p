//! PMU (Power Management Unit) tests for RK3588
//!
//! This test module demonstrates power domain control operations:
//! - Power domain on/off
//! - Power state checking
//! - Batch power operations
//! - Safe power-off sequence testing

use pmu_rk3588::{PowerDomain, RK3588Pmu, PMU_BASE_ADDR, PmuError};

/// Simple delay loop
fn delay(count: usize) {
    for _ in 0..count {
        core::hint::spin_loop();
    }
}

/// Initialize PMU controller
fn init_pmu() -> RK3588Pmu {
    RK3588Pmu::new(PMU_BASE_ADDR)
}

/// Test reading PMU version and status
pub fn test_pmu_version() {
    info!("=== PMU Version Test ===");

    let pmu = init_pmu();
    let version = pmu.version();
    let global_status = pmu.global_power_status();

    info!("PMU Version: 0x{:08X}", version);
    info!("Global Power Status: 0x{:08X}", global_status);
    info!("PMU version test completed\n");
}

/// Test checking power domain status
pub fn test_power_domain_status() {
    info!("=== Power Domain Status Test ===");

    let pmu = init_pmu();

    // Check status of various power domains
    let domains = [
        ("GPU", PowerDomain::GPU),
        ("NPU", PowerDomain::NPU),
        ("VOP", PowerDomain::VOP),
        ("VCODEC", PowerDomain::VCODEC),
        ("AUDIO", PowerDomain::AUDIO),
        ("USB", PowerDomain::USB),
    ];

    for (name, domain) in domains.iter() {
        let status = if pmu.is_power_on(*domain) { "ON" } else { "OFF" };
        info!("  {} power domain: {}", name, status);
    }

    info!("Power domain status test completed\n");
}

/// Test power off a single safe domain (AUDIO)
/// AUDIO domain is relatively safe to power off for testing
pub fn test_power_off_audio() {
    info!("=== Power Off AUDIO Domain Test ===");

    let pmu = init_pmu();
    let domain = PowerDomain::AUDIO;

    // Check initial state
    let initial_state = pmu.is_power_on(domain);
    info!("Initial AUDIO power state: {}", if initial_state { "ON" } else { "OFF" });

    if !initial_state {
        info!("AUDIO is already off, skipping test");
        return;
    }

    // Power off
    info!("Powering off AUDIO domain...");
    match pmu.power_off(domain) {
        Ok(()) => {
            info!("✓ AUDIO domain powered off successfully");
            delay(1_000_000);

            // Verify state
            let new_state = pmu.is_power_on(domain);
            if !new_state {
                info!("✓ Verified: AUDIO is OFF");
            } else {
                warn!("✗ Warning: AUDIO still appears to be ON");
            }
        }
        Err(e) => {
            error!("✗ Failed to power off AUDIO: {:?}", e);
        }
    }

    info!("Power off AUDIO test completed\n");
}

/// Test power on/off cycle for AUDIO domain
pub fn test_power_cycle_audio() {
    info!("=== Power Cycle AUDIO Domain Test ===");

    let pmu = init_pmu();
    let domain = PowerDomain::AUDIO;

    // Power off
    info!("Step 1: Power off AUDIO...");
    match pmu.power_off(domain) {
        Ok(()) => info!("✓ AUDIO powered off"),
        Err(e) => {
            warn!("Power off failed (might already be off): {:?}", e);
        }
    }
    delay(2_000_000);

    // Power on
    info!("Step 2: Power on AUDIO...");
    match pmu.power_on(domain) {
        Ok(()) => {
            info!("✓ AUDIO powered on");
            delay(2_000_000);

            // Verify
            if pmu.is_power_on(domain) {
                info!("✓ Verified: AUDIO is ON");
            } else {
                warn!("✗ Warning: AUDIO still appears to be OFF");
            }
        }
        Err(e) => {
            error!("✗ Failed to power on AUDIO: {:?}", e);
        }
    }

    info!("Power cycle AUDIO test completed\n");
}

/// Test batch power operations with safe domains
pub fn test_batch_power_operations() {
    info!("=== Batch Power Operations Test ===");

    let pmu = init_pmu();

    // Use relatively safe domains for testing
    let test_domains = [
        PowerDomain::AUDIO,
        PowerDomain::SDMMC,
    ];

    info!("Testing batch power off...");
    match pmu.power_off_multiple(&test_domains) {
        Ok(()) => {
            info!("✓ Batch power off successful");

            // Verify all are off
            for domain in &test_domains {
                let state = if pmu.is_power_on(*domain) { "ON" } else { "OFF" };
                info!("  {:?}: {}", domain, state);
            }
        }
        Err(e) => {
            warn!("Batch power off failed: {:?}", e);
        }
    }

    delay(2_000_000);

    info!("Testing batch power on...");
    match pmu.power_on_multiple(&test_domains) {
        Ok(()) => {
            info!("✓ Batch power on successful");

            // Verify all are on
            for domain in &test_domains {
                let state = if pmu.is_power_on(*domain) { "ON" } else { "OFF" };
                info!("  {:?}: {}", domain, state);
            }
        }
        Err(e) => {
            error!("Batch power on failed: {:?}", e);
        }
    }

    info!("Batch power operations test completed\n");
}

/// Test power off with timeout handling
pub fn test_power_off_with_timeout() {
    info!("=== Power Off Timeout Test ===");

    let pmu = init_pmu();

    // Try to power off AUDIO with timeout handling
    let domain = PowerDomain::AUDIO;

    info!("Attempting power off with timeout protection...");

    match pmu.power_off(domain) {
        Ok(()) => {
            info!("✓ Power off completed successfully");
        }
        Err(PmuError::Timeout) => {
            warn!("✗ Power off timed out");
        }
        Err(e) => {
            error!("✗ Power off failed: {:?}", e);
        }
    }

    info!("Power off timeout test completed\n");
}

/// Test BIU idle handling for domains with BIU interface
pub fn test_biu_idle_handling() {
    info!("=== BIU Idle Handling Test ===");

    let pmu = init_pmu();

    // GPU has BIU interface
    let domain = PowerDomain::GPU;

    info!("Testing BIU idle for GPU domain...");
    let initial_state = pmu.is_power_on(domain);
    info!("Initial GPU state: {}", if initial_state { "ON" } else { "OFF" });

    // Note: This test only demonstrates the flow
    // Actually powering off GPU might affect display
    info!("(Skipping actual power off to avoid display issues)");

    // Test domain without BIU
    let domain_no_biu = PowerDomain::GMAC;
    info!("Testing domain without BIU (GMAC)...");

    match pmu.power_off(domain_no_biu) {
        Ok(()) => info!("✓ Power off successful (no BIU idle needed)"),
        Err(e) => warn!("Power off result: {:?}", e),
    }

    info!("BIU idle handling test completed\n");
}

/// Test power state transitions
pub fn test_power_state_transitions() {
    info!("=== Power State Transitions Test ===");

    let pmu = init_pmu();
    let domain = PowerDomain::AUDIO;

    // Test multiple transitions
    let cycles = 3;
    info!("Performing {} power state transitions...", cycles);

    for i in 0..cycles {
        info!("Cycle {}/{}", i + 1, cycles);

        // OFF -> ON
        match pmu.power_on(domain) {
            Ok(()) => info!("  ✓ ON transition successful"),
            Err(e) => warn!("  ✗ ON transition failed: {:?}", e),
        }
        delay(1_000_000);

        // ON -> OFF
        match pmu.power_off(domain) {
            Ok(()) => info!("  ✓ OFF transition successful"),
            Err(e) => warn!("  ✗ OFF transition failed: {:?}", e),
        }
        delay(1_000_000);
    }

    // Restore to ON state
    info!("Restoring AUDIO to ON state...");
    let _ = pmu.power_on(domain);

    info!("Power state transitions test completed\n");
}

/// Test error handling
pub fn test_error_handling() {
    info!("=== Error Handling Test ===");

    let pmu = init_pmu();
    let domain = PowerDomain::AUDIO;

    // Test double power off
    info!("Test 1: Double power off");
    let _ = pmu.power_off(domain);
    match pmu.power_off(domain) {
        Ok(()) => info!("✓ Second power off handled gracefully (already off)"),
        Err(e) => info!("  Result: {:?}", e),
    }

    // Test double power on
    info!("Test 2: Double power on");
    let _ = pmu.power_on(domain);
    match pmu.power_on(domain) {
        Ok(()) => info!("✓ Second power on handled gracefully (already on)"),
        Err(e) => info!("  Result: {:?}", e),
    }

    info!("Error handling test completed\n");
}

/// Run all PMU tests
pub fn run_all_pmu_tests() {
    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║         RK3588 PMU Driver Test Suite                ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    test_pmu_version();
    test_power_domain_status();
    test_power_off_audio();
    test_power_cycle_audio();
    test_batch_power_operations();
    test_power_off_with_timeout();
    test_biu_idle_handling();
    test_power_state_transitions();
    test_error_handling();

    info!("╔══════════════════════════════════════════════════════╗");
    info!("║         All PMU Tests Completed                     ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("\n");
}

/// Safe power off demo - powers off AUDIO domain
pub fn pmu_power_off_demo() {
    info!("\n");
    info!("=== PMU Power Off Demo ===");
    info!("This demo safely powers off the AUDIO domain");
    info!("");

    let pmu = init_pmu();
    let domain = PowerDomain::AUDIO;

    // Show initial state
    info!("Initial state check...");
    let initial = pmu.is_power_on(domain);
    info!("  AUDIO domain: {}", if initial { "ON" } else { "OFF" });

    if initial {
        info!("\nPowering off AUDIO domain...");
        match pmu.power_off(domain) {
            Ok(()) => {
                info!("✓ Success! AUDIO domain powered off");

                // Verify
                delay(1_000_000);
                let final_state = pmu.is_power_on(domain);
                info!("  Final state: {}", if final_state { "ON" } else { "OFF" });
            }
            Err(e) => {
                error!("✗ Failed: {:?}", e);
            }
        }
    } else {
        info!("AUDIO domain is already OFF");
    }

    info!("\nDemo completed\n");
}

/// Critical domains that should NOT be powered off during testing
#[allow(dead_code)]
const CRITICAL_DOMAINS: &[PowerDomain] = &[
    PowerDomain::GPU,    // May affect display
    PowerDomain::VOP,    // Display controller
    PowerDomain::VO0,    // Video output
    PowerDomain::VO1,    // Video output
    PowerDomain::CENTER, // System interconnect
    PowerDomain::SECURE, // Security subsystem
];

/// Safe domains for power testing
#[allow(dead_code)]
const SAFE_TEST_DOMAINS: &[PowerDomain] = &[
    PowerDomain::AUDIO,
    PowerDomain::SDMMC,
];

/// Test CRU-based system reboot functionality
pub fn test_cru_reboot_first_level() {
    use pmu_rk3588::{CRU_BASE_ADDR, RebootLevel};

    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║         CRU System Reboot Test (First Level)        ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    let pmu = init_pmu();

    info!("Testing CRU-based system reboot...");
    info!("Reboot Level: First (Thorough Reset)");
    info!("This will reset almost all logic except hardware-reset-only registers");
    info!("");

    warn!("!!! WARNING: System will reboot in 3 seconds !!!");
    delay(10_000_000);
    warn!("3...");
    delay(10_000_000);
    warn!("2...");
    delay(10_000_000);
    warn!("1...");
    delay(10_000_000);

    info!("Triggering system reboot now...");

    // SAFETY: This will trigger a system reboot
    unsafe {
        pmu.reboot_via_cru(CRU_BASE_ADDR, RebootLevel::First, None);
    }

    // Should never reach here
    error!("ERROR: Reboot failed!");
}

/// Test CRU-based system reboot with second level (state-preserving)
pub fn test_cru_reboot_second_level() {
    use pmu_rk3588::{CRU_BASE_ADDR, RebootLevel};

    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║        CRU System Reboot Test (Second Level)        ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    let pmu = init_pmu();

    info!("Testing CRU-based system reboot...");
    info!("Reboot Level: Second (State-Preserving Reset)");
    info!("This will reset almost all logic but preserve GRFs and GPIOs state");
    info!("");

    warn!("!!! WARNING: System will reboot in 3 seconds !!!");
    delay(10_000_000);
    warn!("3...");
    delay(10_000_000);
    warn!("2...");
    delay(10_000_000);
    warn!("1...");
    delay(10_000_000);

    info!("Triggering system reboot now...");

    // SAFETY: This will trigger a system reboot
    unsafe {
        pmu.reboot_via_cru(CRU_BASE_ADDR, RebootLevel::Second, None);
    }

    // Should never reach here
    error!("ERROR: Reboot failed!");
}

/// Test CRU reboot with custom reset counter threshold
pub fn test_cru_reboot_with_threshold() {
    use pmu_rk3588::{CRU_BASE_ADDR, RebootLevel};

    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║      CRU System Reboot Test (Custom Threshold)      ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    let pmu = init_pmu();

    // Set reset counter threshold to 500 OSC cycles
    let threshold = 500u32;

    info!("Testing CRU-based system reboot with custom threshold...");
    info!("Reboot Level: First (Thorough Reset)");
    info!("Reset Counter Threshold: {} OSC cycles", threshold);
    info!("");

    warn!("!!! WARNING: System will reboot in 3 seconds !!!");
    delay(10_000_000);
    warn!("3...");
    delay(10_000_000);
    warn!("2...");
    delay(10_000_000);
    warn!("1...");
    delay(10_000_000);

    info!("Triggering system reboot now...");

    // SAFETY: This will trigger a system reboot
    unsafe {
        pmu.reboot_via_cru(CRU_BASE_ADDR, RebootLevel::First, Some(threshold));
    }

    // Should never reach here
    error!("ERROR: Reboot failed!");
}

/// Quick demo of CRU reboot functionality (displays info only, doesn't actually reboot)
pub fn demo_cru_reboot_info() {
    use pmu_rk3588::{CRU_BASE_ADDR, RebootLevel};

    info!("\n");
    info!("╔══════════════════════════════════════════════════════╗");
    info!("║         CRU System Reboot Information Demo          ║");
    info!("╚══════════════════════════════════════════════════════╝");
    info!("");

    info!("CRU Base Address: 0x{:08X}", CRU_BASE_ADDR);
    info!("");
    info!("Available Reboot Levels:");
    info!("  1. First Level (RebootLevel::First)");
    info!("     - Thorough reset");
    info!("     - Resets almost all logic");
    info!("     - Only preserves hardware-reset-only registers");
    info!("");
    info!("  2. Second Level (RebootLevel::Second)");
    info!("     - State-preserving reset");
    info!("     - Resets almost all logic");
    info!("     - Preserves GRFs and GPIOs state");
    info!("");
    info!("Reset Counter Threshold:");
    info!("  - Configurable from 0 to 1023 OSC clock cycles");
    info!("  - Defines the assertion time for global software reset");
    info!("  - Maximum duration: ~1ms");
    info!("");
    info!("Hardware Features:");
    info!("  - Self-de-asserting reset mechanism");
    info!("  - Magic value protection (0xfdb9 for Level 1, 0xeca8 for Level 2)");
    info!("  - Immediate effect upon register write");
    info!("");
    info!("Usage Example:");
    info!("  unsafe {{");
    info!("    pmu.reboot_via_cru(CRU_BASE_ADDR, RebootLevel::First, None);");
    info!("  }}");
    info!("");
    info!("Demo completed (system not rebooted)\n");
}
