# System Power-Off Test for RK3588

This document describes the system power-off test that demonstrates a complete system shutdown sequence using the PMU driver.

## Overview

The `test_system_power_off()` function provides a **safe and orderly system shutdown** by:
1. Checking current power states of all peripheral domains
2. Powering off safe peripheral domains in sequence
3. Verifying power-off completion
4. Triggering system shutdown via `axplat::power::system_off()`

⚠️ **WARNING**: This test will actually power off the system!

## Test Function

### `test_system_power_off()`

Located in: `src/test/pmu_power_off_test.rs`

**What it does:**
- Powers off 6 safe peripheral domains: AUDIO, SDMMC, SDIO, GMAC, PCIE, CRYPTO
- Reports detailed status for each step
- Counts successes, skips, and failures
- Calls `axplat::power::system_off()` at the end

## How to Run

### Method 1: Enable in main.rs (Recommended)

Edit `src/main.rs` and uncomment line 65:

```rust
// Option 4: System power-off test (WARNING: Powers off the system!)
test::test_system_power_off();  // ← Uncomment this line
```

Then build and run:
```bash
make build
make run
```

### Method 2: Use the alias function

```rust
test::system_shutdown_test();  // Same as test_system_power_off()
```

## Expected Output

```
╔══════════════════════════════════════════════════════╗
║                                                      ║
║         SYSTEM POWER-OFF TEST                       ║
║                                                      ║
║  This test will power off the system!              ║
║                                                      ║
╚══════════════════════════════════════════════════════╝

Step 1: Checking initial power states...
─────────────────────────────────────────────────────
  AUDIO           : ON
  SDMMC           : ON
  SDIO            : OFF
  GMAC            : ON
  PCIE            : OFF
  CRYPTO          : ON
  Total active domains: 4

Step 2: Powering off peripheral domains...
─────────────────────────────────────────────────────
  AUDIO           : Powering off...
  AUDIO           : ✓ OFF
  SDMMC           : Powering off...
  SDMMC           : ✓ OFF
  SDIO            : Already OFF (skipped)
  GMAC            : Powering off...
  GMAC            : ✓ OFF
  PCIE            : Already OFF (skipped)
  CRYPTO          : Powering off...
  CRYPTO          : ✓ OFF

Power-off summary:
  ✓ Successfully powered off: 4
  - Already off (skipped):    2

Step 3: Final system power state...
─────────────────────────────────────────────────────
  AUDIO           : OFF
  SDMMC           : OFF
  SDIO            : OFF
  GMAC            : OFF
  PCIE            : OFF
  CRYPTO          : OFF

Step 4: Initiating system shutdown...
─────────────────────────────────────────────────────
  PMU power state: prepared for shutdown
  Calling system power off...

╔══════════════════════════════════════════════════════╗
║                                                      ║
║         System shutting down...                     ║
║         Goodbye!                                     ║
║                                                      ║
╚══════════════════════════════════════════════════════╝

[System powers off]
```

## Test Phases

### Phase 1: Initial State Check
- Reads power state of all target domains
- Counts how many are currently active
- Provides baseline for comparison

### Phase 2: Power-Off Sequence
- Iterates through each domain
- Skips domains already off
- Powers off active domains
- Verifies each power-off operation
- Tracks success/skip/failure counts

### Phase 3: Verification
- Re-checks all domain states
- Displays final power configuration
- Ensures all intended domains are off

### Phase 4: System Shutdown
- Displays shutdown message
- Waits 1 second for logs to flush
- Calls `axplat::power::system_off()`
- System powers down

## Safe Domains

The test powers off these **peripheral domains** which are safe to disable:

| Domain  | Function | Safe to Power Off? |
|---------|----------|-------------------|
| AUDIO   | Audio codec | ✅ Yes |
| SDMMC   | SD card | ✅ Yes (if not booting from SD) |
| SDIO    | SDIO interface | ✅ Yes |
| GMAC    | Gigabit Ethernet | ✅ Yes |
| PCIE    | PCIe controller | ✅ Yes (if no PCIe devices) |
| CRYPTO  | Crypto engine | ✅ Yes |

## NOT Powered Off

These **critical domains** are NOT touched by the test:

- **GPU** - Graphics (would kill display)
- **VOP** - Video output (display controller)
- **VO0/VO1** - Video output channels
- **CENTER** - System interconnect (critical!)
- **SECURE** - Security subsystem
- **NPU** - Neural processing (if in use)
- **VCODEC** - Video codec (if in use)

## Use Cases

### 1. System Shutdown Hook
Add to your system shutdown sequence:
```rust
pub fn system_shutdown() {
    // Save state, flush logs, etc.
    cleanup_resources();

    // Power off peripherals
    test_system_power_off();
}
```

### 2. Low Power Mode Preparation
Before entering deep sleep:
```rust
pub fn prepare_sleep() {
    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    // Power off unused peripherals
    pmu.power_off_multiple(&[
        PowerDomain::AUDIO,
        PowerDomain::GMAC,
        PowerDomain::CRYPTO,
    ]).unwrap();
}
```

### 3. Testing PMU Functionality
Verify PMU driver works correctly:
```rust
// Test without actual shutdown
test_system_power_off_no_shutdown(); // Powers off domains only
```

## Safety Notes

### ✅ Safe to Use When:
- System is about to shut down anyway
- All important data is saved
- No critical operations in progress
- Testing PMU driver functionality

### ⚠️ Be Careful When:
- SD card is mounted (SDMMC domain)
- Network operations in progress (GMAC domain)
- PCIe devices are active (PCIE domain)

### ❌ Do NOT Use When:
- System must stay running
- Critical services are active
- Unsaved data exists
- You're testing on production hardware

## Troubleshooting

### Issue: Domain fails to power off
**Symptom**: `✗ Error: Timeout` or `BiuIdleFailed`

**Possible causes:**
1. Domain is currently in use
2. Hardware has pending transactions
3. Dependency on another domain

**Solution:**
- Check if domain is actually being used
- Wait for operations to complete
- Review RK3588 TRM for dependencies

### Issue: System doesn't actually power off
**Symptom**: Test completes but system stays on

**Possible causes:**
1. `axplat::power::system_off()` not implemented
2. PSCI call failed
3. Firmware doesn't support power off

**Solution:**
- Check PSCI support in firmware
- Verify ATF/U-Boot configuration
- Check kernel power management settings

### Issue: System hangs during test
**Symptom**: Test stops mid-execution

**Possible causes:**
1. Powered off a critical domain
2. Dependency issue between domains
3. Hardware bug

**Solution:**
- Review which domain caused hang
- Check RK3588 TRM for dependencies
- Modify domain list if needed

## Customization

### Change Domains to Power Off

Edit the domain list in `test_system_power_off()`:

```rust
let safe_domains_to_shutdown = [
    ("AUDIO", PowerDomain::AUDIO),
    ("SDMMC", PowerDomain::SDMMC),
    // Add more domains:
    ("YOUR_DOMAIN", PowerDomain::YOUR_DOMAIN),
];
```

### Power Off Without System Shutdown

Create a modified version:

```rust
pub fn power_off_peripherals_only() {
    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);

    let domains = [/* ... */];

    // Power off domains
    for (name, domain) in domains.iter() {
        let _ = pmu.power_off(*domain);
    }

    // DON'T call system_off() here
}
```

### Add Pre-Shutdown Hooks

```rust
pub fn test_system_power_off_with_hooks() {
    // Pre-shutdown hook
    info!("Running pre-shutdown tasks...");
    save_state();
    flush_logs();

    // Run power-off test
    test_system_power_off();
}
```

## Related Functions

- `quick_power_off_demo()` - Quick demo without shutdown
- `run_all_power_off_tests()` - Full test suite
- `pmu.power_off()` - Power off single domain
- `pmu.power_off_multiple()` - Power off multiple domains

## References

- RK3588 TRM Chapter 7: Power Management Unit
- PMU Driver: `module-local/pmu_rk3588/`
- Power-Off Tests: `src/test/pmu_power_off_test.rs`
- System Power API: `axplat::power::system_off()`

## Summary

The system power-off test provides a **safe, verified, and well-logged** shutdown sequence that:
- ✅ Only powers off safe peripheral domains
- ✅ Verifies each step
- ✅ Provides detailed logging
- ✅ Handles errors gracefully
- ✅ Actually shuts down the system

Perfect for:
- Testing PMU driver
- Implementing shutdown sequences
- Preparing for low-power modes
- Demonstrating safe power management
