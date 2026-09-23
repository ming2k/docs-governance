# AGENTS.md

Instructions for AI coding assistants working in the `docs-governance` repository.

---

## 1. Repository Identity & Mission

`docs-governance` is a portable, protocol-versioned documentation governance standard and verification toolchain for software engineering repositories.

The repository follows the **Clean-Break Architecture** (Protocol v5.0.0):
1. **`spec/core/`**: The universal meta-governance protocol:
   - `taxonomy.md`: 4-Dimensional spatial coordinate tensor (Temperature x Lifecycle x Audience x Cognitive Mode).
   - `invariants.md`: Codified constitution of numbered system invariants (`[INV-*]`).
   - `workflow.md`: Unified code-to-doc trigger matrix, PR review gates, intake SOP, and adoption.
   - `style.md`: Technical voice, structural syntax, and link contracts.
   - `index.md`: Protocol charter and core directory navigation.
2. **`spec/profiles/`**: Pluggable vertical domain capability entities:
   - `architecture/`: `adr.md`, `living-snapshot.md`, `rfc.md`, `index.md`.
   - `validation/`: `acceptance.md`, `testing.md`, `index.md`.
   - `operations/`: `postmortem.md`, `index.md`.
3. **`spec/contracts.md`**: Template for downstream repository contract and profile declarations.
4. **`tools/`**: Self-contained verification and distribution utilities (`sync.sh`, `verify.sh`, `update-hashes.sh`).
5. **`docs/`**: The self-hosted (dogfooding) documentation for `docs-governance` itself.

---

## 2. Policy & Modification Guardrails

### A. The `spec/` Directory
- `spec/` defines portable governance policy adopted across downstream repositories.
- AI assistants may read `spec/` and suggest improvements, but must only edit it upon explicit maintainer instruction.
- **Whenever `spec/` files change**:
  1. Follow the **Four-Tier Admission Filter** defined in `spec/core/workflow.md` (Tier 0 Core vs Tier 1 Pattern vs Tier 2 Profile).
  2. Evaluate protocol version bump (`protocol_version` in `spec/.manifest.json`) per semantic versioning rules.
  3. Run `./tools/update-hashes.sh` to update cryptographic SHA-256 signatures in `spec/.manifest.json`.
  4. Run `./tools/sync.sh .` and `./tools/verify.sh .` to update and verify the local `docs/governance/documentation/` mirror.

### B. The `docs/` Directory
- Follow the 4D Spatial Tensor defined in `docs/governance/documentation/core/taxonomy.md`.
- Adhere to the System Invariants in `docs/governance/documentation/core/invariants.md`:
  - `[INV-CORE-01]`: Never link from public documentation into `docs/dev/`.
  - `[INV-TEMP-02]`: Default to excluding `**/archive/**` from daily prompt contexts and search indices.
  - `[INV-VAL-01]`: `docs/dev/acceptance.md` verifies real user journeys from a cold start.
  - `[INV-VAL-02]`: `docs/dev/testing.md` verifies programmatic implementation correctness and automated test suites.

---

## 3. Toolchain & Testing Invariants

- **`[INV-TOOL-01]` Zero External Dependencies**: All tools in `tools/` must rely strictly on standard POSIX shell (`/bin/bash` or `/bin/sh`) and Python 3 standard library (`hashlib`, `json`, `os`, `shutil`, `sys`).
- **Automated Verification**: Before completing any task, run:
  ```bash
  python3 -W error -m unittest discover -s tests -v
  ./tools/verify.sh .
  ```

<!-- BEGIN DOCGOV DIRECTIVES -->
## Documentation Governance Directives

You are bound by repository invariants. Violations will fail CI (`docgov check`).

### 1. Machine Invariants (Pre-Submit Checklist)
- `[INV-LINT-01] Location Sanitization`: Never create arbitrary Markdown files at the repository root.
- `[INV-LINT-02] Contributor Firewall`: Public docs (`docs/{tutorials,how-to,reference,explanation}/`) must NEVER link into internal docs (`docs/dev/`).
- `[INV-LINT-03] Frontmatter Schema`: ADRs must contain valid Frontmatter with standardized status enum.
- `[INV-LINT-04] Code-Doc Sync`: Modifying monitored paths in `src/` requires updating `docs/` in the same change.
- `[INV-LINT-05] Agent Directives Binding`: Ensure this docgov directives block is retained in agent configuration.

### 2. Cognitive & Architecture Protocols (Thinking Framework)
- `[INV-AGENT-01] Negative Knowledge`: Every new ADR MUST contain a 'Rejected Alternatives' section explaining why discarded options were not chosen.
- `[INV-AGENT-02] Context Routing & Chesterton's Fence`:
  - In feature generation: NEVER use docs marked `status: superseded` or `status: rejected` as active designs (prevents resurrecting dead patterns).
  - In refactoring/investigation: MUST retrieve `superseded` docs as negative constraints (learn from historical failure modes).
- `[INV-AGENT-03] Blameless Postmortem`: Postmortems MUST analyze system defense failures and detection gaps. Attribution of personal human blame is strictly prohibited.

### 3. Canonical Governance Knowledge & Context
Before drafting or restructuring documentation, inspect the local governance specifications:
- 4D Coordinate Tensor: `docs/governance/documentation/core/taxonomy.md`
- System Invariants Constitution: `docs/governance/documentation/core/invariants.md`
- Technical Voice & Link Contracts: `docs/governance/documentation/core/style.md`
- ADR & Architecture RFC Standard: `docs/governance/documentation/profiles/architecture/adr.md`
- Quality & Verification Guides: `docs/governance/documentation/profiles/validation/testing.md`

### 4. Fast Verification
Before completing any task, run:
```bash
docgov check
```
<!-- END DOCGOV DIRECTIVES -->
