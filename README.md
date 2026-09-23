# Documentation Governance Standard (docs-governance)

A portable, rigorous documentation governance standard and verification toolchain for software engineering repositories.

Designed for projects requiring strict architectural discipline, cross-language consistency, and AI-agent compatibility without documentation drift.

---

## 1. Overview

`docs-governance` establishes a project-neutral, protocol-versioned standard (Protocol v5.0.0) based on the **Clean-Break Architecture**: a universal **Semantic Tensor Core** paired with pluggable **Vertical Domain Entities**.

Adopting repositories mirror the specification into `docs/governance/documentation/` and use the self-contained toolchain (`sync.sh`, `verify.sh`) to assemble and verify documentation surfaces declared in `contracts.md`.

### Key Capabilities:
* **4D Spatial Coordinate Tensor**: Eliminates ad-hoc routing by deterministically placing every document into a unique `(Temperature, Lifecycle, Audience, Cognitive Mode)` coordinate.
* **System Invariants Constitution**: Codified, numbered non-negotiable negative and positive constraints (`[INV-*]`) citeable by CI linters, PR reviewers, and AI coding assistants.
* **Vertical Domain Entities**: Self-contained capability packages (`adr.md`, `rfc.md`, `living-snapshot.md`, `acceptance.md`, `testing.md`, `postmortem.md`) packaging their own invariants, lifecycle state machines, authoritative templates, and SOPs.
* **Three Knowledge Temperature Tiers**:
  * **HOT**: Living system ground truth (Diátaxis, `docs/architecture/`, `README.md`), continuously synchronized with code.
  * **WARM**: Active contracts and in-flight deliberations (`docs/adr/*.md`, `docs/rfc/*.md`), consulted just-in-time.
  * **COLD**: Retired history and negative knowledge (`docs/adr/archive/`, `docs/rfc/archive/`), **firewalled from daily AI prompt contexts**.
* **Unified Operational Lifecycle**: Integrates code-to-doc trigger matrices, PR review gates, standard intake SOPs, and adoption workflows into a single operational guide (`core/workflow.md`).
* **Cryptographic Verification & Drift Detection**: Protocol v5.0.0 `.manifest.json` tracks SHA-256 signatures for zero-drift CI enforcement.
* **Zero External Dependencies**: Toolchain executes purely on standard POSIX shell and Python 3 standard library.

---

## 2. Directory Layout & Architecture

Repositories adopting `docs-governance` organize their `docs/` hierarchy into structured, non-overlapping domains:

```text
docs/
├── governance/                     # Repository & Architecture Governance (Top-Level)
│   ├── index.md                    # Governance charter, review gates, RACI matrix
│   ├── api-design-guidelines.md    # API conventions, naming, and memory contracts
│   │
│   └── documentation/              # Mirrored Docs Governance (Managed via this spec)
│       ├── .manifest.json          # Protocol version, profile roles, and SHA-256 hashes
│       ├── contracts.md            # Activated profiles and repository path bindings
│       ├── AGENTS.md               # Directory-level AI policy protection guard
│       │
│       ├── core/                   # Universal Meta-Governance Core
│       │   ├── index.md            # Protocol charter and core navigation
│       │   ├── taxonomy.md         # 4D coordinate tensor (Temperature x Lifecycle x Audience x Mode)
│       │   ├── invariants.md       # Codified constitution of numbered system invariants (INV-*)
│       │   ├── workflow.md         # Trigger matrix, PR review gates, intake SOP, adoption
│       │   └── style.md            # Technical voice, structural syntax, link contracts
│       │
│       └── profiles/               # Pluggable Vertical Domain Entities
│           ├── architecture/       # Architecture Decision Records, Blueprints, RFCs
│           │   ├── index.md        # Domain charter and entity relationship topology
│           │   ├── adr.md          # Immutable decision records and negative knowledge
│           │   ├── rfc.md          # Pre-decision debate, golden rule, 4-step archival SOP
│           │   └── living-snapshot.md # Living blueprints and 4-step compaction SOP
│           ├── validation/         # Product Validation Profile
│           │   ├── index.md        # Dual-tier validation charter
│           │   ├── acceptance.md   # Real user journey acceptance matrices
│           │   └── testing.md      # Automated test suites, fixture matrices, commands
│           └── operations/         # Operational Knowledge Profile
│               ├── index.md        # Escalation layering (Runbook -> Postmortem -> ADR)
│               └── postmortem.md   # Blameless post-incident forensics
│
├── architecture/                   # Living Architecture Blueprints (HOT Tier)
│   └── <subsystem>.md              # Current authoritative component models and invariants
│
├── adr/                            # Architectural Decision Records (WARM Tier)
│   ├── index.md                    # Registry table of active and compacted decisions
│   ├── NNNN-<slug>.md              # Immutable accepted decision records
│   └── archive/                    # Retired decisions (COLD Tier — Compacted / Superseded)
│
├── rfc/                            # Pre-decision Proposals (WARM Tier — Optional)
│   ├── index.md                    # Registry table of in-flight and concluded proposals
│   ├── RFC-NNNN-<slug>.md          # Active in-flight proposals under deliberation
│   └── archive/                    # Concluded proposals (COLD Tier — Consensus / Withdrawn)
│
├── explanation/                    # Deep Dives & Architectural Theory (Diátaxis)
│   └── design-philosophy.md        # Core technical philosophy & system invariants
│
├── dev/                            # Internal Developer & Validation Firewall
│   ├── setup.md                    # Environment prerequisites & build workflow
│   ├── acceptance.md               # Deliverable acceptance & real user journey matrices
│   ├── testing.md                  # Programmatic test suites & implementation correctness
│   └── postmortems/                # Archived post-incident reviews (COLD Tier)
│
├── tutorials/                      # Learning-oriented beginner walk-throughs (Diátaxis)
├── how-to/                         # Goal-oriented problem-solving guides (Diátaxis)
└── reference/                      # Austere technical specifications & CLI flags (Diátaxis)
```

---

## 3. Verification & Distribution Toolchain

The repository provides shell and Python utilities in `tools/`:

### Update Downstream Repositories (`sync.sh`)
```bash
# Synchronize core protocol and activated profiles to a downstream repository:
./tools/sync.sh <target-repo-path>

# Force-overwrite locally modified templates with spec defaults:
./tools/sync.sh <target-repo-path> --force-template
```

### Verify Integrity & Zero-Drift (`verify.sh`)
```bash
# Check compliance against the cryptographic manifest:
./tools/verify.sh <target-repo-path>
```

### Recompute SHA-256 Hashes (`update-hashes.sh` — Maintainers Only)
```bash
# Recompute SHA-256 signatures in spec/.manifest.json after modifying the spec:
./tools/update-hashes.sh
```

---

## 4. Quick Start: Adopting in Your Repository

1. Run `./tools/sync.sh <repo-path>` to initialize the governance mirror and `contracts.md`.
2. Edit `<repo-path>/docs/governance/documentation/contracts.md` to check active domain profiles (`core`, `architecture`, `validation`, `operations`).
3. Re-run `./tools/sync.sh <repo-path>` to assemble the activated profile surfaces.
4. Add `./tools/verify.sh .` to your CI pull request validation matrix.
