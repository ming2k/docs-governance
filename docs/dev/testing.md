# Testing

This document details how contributors execute and extend automated tests for the `docs-governance` toolchain.

---

## Scope & Test Model

The testing architecture uses Python's standard `unittest` framework with zero external dependencies:

```text
tests/
└── test_tools.py          # Integration tests for sync.sh, verify.sh, profiles, and contracts
```

### Verification Channels

| Channel | Tool / Target | Purpose |
|---------|---------------|---------|
| Modular Tooling Tests | `tests/test_tools.py` | Validates profile activation, core isolation, drift detection, and contract preservation. |
| Manifest Integrity | `tools/verify.sh .` | Verifies that local `docs/governance/documentation/` strictly matches `standard/`. |
| Hash Recomputation | `tools/update-hashes.sh` | Updates SHA-256 signatures in `standard/.manifest.json`. |

---

## Run Tests

### Run Full Test Suite
```bash
python3 -m unittest discover -s tests -v
```

### Run Single Test Case
```bash
python3 -m unittest tests.test_tools.TestModularGovernanceTools.test_core_only_profile_activation
```

### Run Repository Self-Verification
```bash
./tools/verify.sh .
```

---

## Adding a Test

When contributing new tooling or features:
1. Place new test methods in `tests/test_tools.py` under `TestModularGovernanceTools`.
2. Use `tempfile.mkdtemp()` in `setUp()` and clean up in `tearDown()`.
3. Assert return codes and exact stdout/stderr messages.
