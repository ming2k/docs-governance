# Acceptance

This document defines the real, end-to-end acceptance procedure for adopting and configuring `docs-governance` in target repositories.

It answers: **Can an authentic user install, configure active profiles, sync, and verify documentation governance in a target repository without shortcuts or backdoors?**

---

## Scope & Prerequisites

- **Scope**: Initial standard installation, selective profile activation, drift detection, and CI verification.
- **Prerequisites**: A clean Unix environment with `bash`, `python3` (3.8+), and `git`.

---

## Core User Journeys

### Journey 1: Adopting Core Protocol & Customizing Profiles

#### Steps
1. Navigate to the `docs-governance` workspace.
2. Initialize a clean target repository:
   ```bash
   TARGET_DIR=$(mktemp -d)
   git -C "$TARGET_DIR" init
   ```
3. Run the synchronization tool:
   ```bash
   ./tools/sync.sh "$TARGET_DIR"
   ```
4. Verify standard core files and initialized `contracts.md`:
   ```bash
   ls -la "$TARGET_DIR/docs/governance/documentation/"
   ```
5. Customize `$TARGET_DIR/docs/governance/documentation/contracts.md` to enable only `core` and `validation` profiles:
   ```markdown
   ## 1. Activated Profiles
   - [x] `core`
   - [x] `validation`
   - [ ] `architecture`
   - [ ] `operations`
   ```
6. Re-run `./tools/sync.sh "$TARGET_DIR"`.
7. Execute the verification tool:
   ```bash
   ./tools/verify.sh "$TARGET_DIR"
   ```
8. Clean up:
   ```bash
   rm -rf "$TARGET_DIR"
   ```

#### Expected Outcome
- Step 3 initializes `contracts.md` and copies default profiles.
- Step 6 retains customized `contracts.md`.
- Step 7 reports `active profiles: core, validation`, verifies core and validation files, and exits with code `0`.

#### Pass / Fail Criteria
- **Pass**: All active profile files verified cleanly; inactive profiles not flagged as missing.
- **Fail**: Any non-zero exit code or false-positive drift on unselected profiles.

---

## Acceptance Scenario Matrix

| Scenario / Edge Case | Verification Path / Trigger | Expected Outcome | Pass/Fail |
|----------------------|-----------------------------|------------------|-----------|
| **Core File Tampering** | Append unauthorized text to `$TARGET_DIR/docs/governance/documentation/routing.md` | `verify.sh` reports `DRIFT: routing.md` with status 1 | Pass |
| **Active Profile Deletion** | Delete `$TARGET_DIR/docs/governance/documentation/profiles/validation/profile.md` | `verify.sh` reports `DRIFT: missing: profiles/validation/profile.md` with status 1 | Pass |
| **Inactive Profile Isolation** | Sync repository configured for `core` only | `verify.sh` succeeds with status 0, zero false-positive drift on absent profiles | Pass |
| **Contracts Customization** | Re-run `sync.sh` against modified `contracts.md` | Modified `contracts.md` is preserved without overwrite | Pass |
| **Extraneous File in Docs** | Add unmanifested file inside `docs/governance/documentation/` | `verify.sh` reports `DRIFT: not in manifest` with status 1 | Pass |

---

## Acceptance Checklist

- [ ] `sync.sh` installs core and activated profiles correctly.
- [ ] Customized `contracts.md` is preserved on subsequent sync runs.
- [ ] Inactive profiles do not trigger missing file drift in `verify.sh`.
- [ ] File tampering and deletions correctly trigger drift errors.
- [ ] `verify.sh` succeeds with status `0` on compliant target repository.
