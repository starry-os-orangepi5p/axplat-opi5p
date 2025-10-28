# Quick Start: System Power-Off Test

## 🚀 Quick Run Guide

### 1. Enable the Test

Edit `src/main.rs` line 65:

```rust
// Before (commented out):
// test::test_system_power_off();

// After (uncommented):
test::test_system_power_off();
```

### 2. Build and Run

```bash
cd axplat-opi5p
make build
make run
```

### 3. What Happens

The system will:
1. ✅ Check current power states
2. ✅ Power off 6 peripheral domains (AUDIO, SDMMC, SDIO, GMAC, PCIE, CRYPTO)
3. ✅ Verify all power-offs succeeded
4. ⚠️ **Shutdown the system**

---

## 📝 Test Options

### Option A: Full System Shutdown (Powers off system)
```rust
test::test_system_power_off();
```

### Option B: All Power-Off Tests (Safe, doesn't shutdown)
```rust
test::run_all_power_off_tests();
```

### Option C: Quick Demo (Safe, doesn't shutdown)
```rust
test::quick_power_off_demo();
```

---

## 🎯 One-Line Summary

**Powers off peripheral domains, verifies completion, then shuts down the system.**

---

## ⚠️ Important Notes

- **Will actually power off the system!**
- Only powers off safe peripheral domains
- Does NOT touch critical domains (GPU, VOP, CENTER, etc.)
- Fully logged with ✓/✗ status indicators

---

## 📊 Example Output (Last 20 Lines)

```
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

---

## 🔧 Customization

### Change Domains to Power Off

Edit `src/test/pmu_power_off_test.rs` line 314:

```rust
let safe_domains_to_shutdown = [
    ("AUDIO", PowerDomain::AUDIO),
    ("SDMMC", PowerDomain::SDMMC),
    // Add your domains here
];
```

### Disable Actual Shutdown (for testing)

Comment out line 410:

```rust
// axplat::power::system_off();  // ← Comment this
```

---

## 📚 More Info

- Full documentation: `SYSTEM_POWER_OFF_TEST.md`
- All PMU tests: `PMU_TEST_README.md`
- PMU driver: `../../pmu_rk3588/README.md`

---

## ✅ Build Status

```bash
$ cargo build --release --target aarch64-unknown-none-softfloat
   Compiling pmu_rk3588 v0.1.0
   Compiling rstiny v0.1.0
    Finished release [optimized] target(s)
```

**Status**: ✅ Builds successfully, ready to run!
