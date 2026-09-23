# Documentation Governance Standard (docs-governance)

A portable, rigorous documentation governance standard and verification toolchain for software engineering repositories.

Designed for projects requiring strict architectural discipline, cross-language consistency, and AI-agent compatibility without documentation drift.

---

## 1. Overview

`docs-governance` establishes a project-neutral, protocol-versioned standard based on the **Clean-Break Architecture**: a universal **Semantic Tensor Core** paired with pluggable **Vertical Domain Entities**, driven by the high-performance Rust CLI **`docgov`**.

Adopting repositories initialize governance via `docgov init`, maintaining a clean canonical mirror under `docs/governance/documentation/` and non-invasive AI assistant directives in `AGENTS.md`.

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

## 3. Verification & Distribution Toolchain (`docgov`)

`docgov` is a standalone, compiler-grade Rust CLI linter and governance driver for software repositories.

### Installation

```bash
# Via mise (Recommended):
mise use -g cargo:docgov

# Or via cargo:
cargo install docgov
```

### Commands

```bash
# 1. Initialize repository governance (creates .docgov.yml, .docgov.lock, patches AGENTS.md):
docgov init

# 2. Fast compiler-grade static verification (< 20ms in local dev and CI):
docgov check

# 3. Synchronize & atomically prune canonical governance documentation mirror:
docgov sync
```

---

## 4. Quick Start: Adopting in Your Repository

1. Install `docgov` via `mise use -g cargo:docgov` or `cargo install docgov`.
2. In your repository root, run `docgov init`.
3. Verify invariants at any time with `docgov check`.
4. Add `docgov check` to your CI pull request validation matrix.
