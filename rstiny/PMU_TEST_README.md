# PMU Power-Off Tests for RK3588

This directory contains comprehensive test suites for the RK3588 PMU (Power Management Unit) driver, specifically focused on power-off functionality.

## Test Files

### 1. `pmu_test.rs` - Comprehensive PMU Test Suite
Full test coverage including:
- PMU version and status reading
- Power domain status checking
- Power on/off operations
- BIU idle handling
- Batch operations
- Error handling
- State transitions

### 2. `pmu_power_off_test.rs` - Focused Power-Off Tests
Dedicated power-off test suite with 6 focused tests:

1. **Basic Power Off** - Simple power-off of AUDIO domain
2. **Power Off with Verification** - State verification after power-off
3. **Sequential Power Off** - Multiple domains powered off sequentially
4. **Timeout Detection** - Tests timeout handling mechanisms
5. **Idempotent Power Off** - Tests calling power-off twice (safe behavior)
6. **Batch Power Off** - Multiple domains powered off together

## Running Tests

### In `main.rs`, choose one of three options:

```rust
// Option 1: Run all PMU tests (comprehensive, ~2-3 minutes)
test::run_all_pmu_tests();

// Option 2: Run focused power-off tests (recommended, ~1 minute)
test::run_all_power_off_tests();

// Option 3: Quick power-off demo (fastest, ~5 seconds)
test::quick_power_off_demo();
```

### Build and Run

```bash
cd /path/to/axplat-opi5p
make build    # Build the test binary
make run      # Run on hardware (requires OrangePi 5 Plus)
```

## Test Domains

The tests use **safe** power domains that can be powered off without affecting critical system functionality:

### Safe Domains (Used in Tests)
- **AUDIO** - Audio subsystem (safe to power off)
- **SDMMC** - SD/MMC interface (safe if not in use)

### Critical Domains (NOT tested)
- GPU, VOP, VO0, VO1 - Display-related (may affect screen)
- CENTER - System interconnect
- SECURE - Security subsystem

## Test Output Example

```
╔══════════════════════════════════════════════════════╗
║     RK3588 PMU Power-Off Test Suite                 ║
╚══════════════════════════════════════════════════════╝

╔══════════════════════════════════════════════════════╗
║  Test 1: Basic Power Off - AUDIO Domain             ║
╚══════════════════════════════════════════════════════╝
Initial state: ON
Initiating power off...
✓ SUCCESS: Power off completed
✓ VERIFIED: Domain is OFF
Test completed

╔══════════════════════════════════════════════════════╗
║  Test 2: Power Off with State Verification          ║
╚══════════════════════════════════════════════════════╝
Testing domain: SDMMC
  Step 1: Initial state = ON
  Step 2: Executing power off...
    ✓ Power off command succeeded
  Step 4: Verifying state change...
    Final state = OFF
✓ SUCCESS: State transitioned from ON to OFF
Test completed

...
```

## Test Features

### Safety Features
- ✅ Only tests safe, non-critical domains
- ✅ State verification after each operation
- ✅ Timeout protection (10ms per operation)
- ✅ Graceful error handling
- ✅ Clear success/failure indicators

### Test Coverage
- ✅ Basic power-off operations
- ✅ State verification and consistency
- ✅ Timeout and error handling
- ✅ Idempotent operations
- ✅ Batch operations
- ✅ BIU idle management

### Hardware Requirements
- **Board**: OrangePi 5 Plus (RK3588)
- **PMU Base Address**: 0xFD8D8000
- **No special setup required** - tests use standard power domains

## Expected Results

### Successful Test Run
All tests should show:
- ✓ Power off commands succeed
- ✓ State changes verified (ON → OFF)
- ✓ No timeouts or errors
- ✓ Idempotent behavior (calling twice is safe)

### Possible Issues

1. **Timeout Errors**
   - Indicates hardware not responding within 10ms
   - Check if domain is already in use

2. **BIU Idle Failed**
   - Domain's bus interface couldn't enter idle state
   - May indicate active transactions

3. **State Unchanged**
   - Domain may be marked as "always-on"
   - Or hardware dependency prevents power-off

## Integration with Main Tests

The PMU tests integrate with other rstiny tests:
- Allocator tests
- GPIO/LED tests
- GICv3 interrupt tests
- Timer tests
- **PMU power tests** ← New!

## Development Notes

### Adding New Tests

1. Add test function to `pmu_power_off_test.rs`:
```rust
pub fn test_my_new_feature() {
    info!("=== My New Test ===");
    let pmu = RK3588Pmu::new(PMU_BASE_ADDR);
    // ... test code ...
}
```

2. Add to test suite:
```rust
pub fn run_all_power_off_tests() {
    // ...
    test_my_new_feature();
}
```

### Testing Other Domains

To test other domains, update the domain lists:
```rust
let domains = [
    PowerDomain::AUDIO,
    PowerDomain::YOUR_DOMAIN,  // Add here
];
```

**⚠️ WARNING**: Do not test critical domains without understanding the impact!

## References

- RK3588 TRM Chapter 7: Power Management Unit
- PMU Driver: `module-local/pmu_rk3588/`
- Linux Reference: `drivers/soc/rockchip/pm_domains.c`

## Support

For issues or questions:
1. Check test output for specific error messages
2. Verify hardware connections
3. Review PMU driver logs
4. Check domain dependencies in RK3588 TRM
