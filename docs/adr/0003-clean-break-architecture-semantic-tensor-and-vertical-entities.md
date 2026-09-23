---
id: ADR-0003
title: "Clean-Break Architecture: Semantic Tensor Core and Vertical Domain Entities"
status: accepted
date: 2026-08-25
scope: core/taxonomy
---

# 0003. Clean-Break Architecture: Semantic Tensor Core and Vertical Domain Entities

- Status: Accepted
- Date: 2026-08-25
- Deciders: Repository Maintainers
- Consulted: Core Contributors
- Informed: All Adopting Repositories

## Context and Problem Statement

Previous iterations of `docs-governance` (v1.0.0 through v4.0.0) incrementally evolved by patching traditional "document-as-article" Markdown files. While this preserved superficial familiarity, it introduced foundational architectural contradictions:

1. **Information Primitive Mixing**: Core files (`routing.md`, `style-guide.md`) and domain profile guides (`profiles/architecture/profile.md`) blended five orthogonal information primitives: spatial taxonomy, binding negative invariants, markdown schema templates, procedural SOPs, and narrative philosophy.
2. **Routing Tensor Breakdown**: The sequential 4-Gate priority waterfall in `routing.md` could not cleanly accommodate living architecture blueprints (`docs/architecture/`), in-flight pre-decision proposals (`docs/rfc/`), or cold storage archives (`docs/*/archive/`).
3. **AI Context Inefficiency**: AI coding assistants requiring authoritative behavioral constraints (`Invariants`) were forced to ingest expansive narrative prose, causing token bloat, attentional dispersion, and architectural hallucinations from outdated debates.
4. **Domain Profile Fragmentation**: Domain capabilities were split awkwardly between sprawling `profile.md` guides and disjointed `*.pattern.md` files, creating duplicate definitions and cognitive friction.

A clean break from legacy file-container thinking is required to establish an uncompromising, long-termist knowledge architecture.

## Decision Drivers

- **First-Principles Orthogonality**: Separate taxonomy, binding rules, operational workflows, and schema templates into pure, dedicated primitives.
- **Unified 4-Dimensional Coordinate Tensor**: Every document in any repository must map deterministically to a unique 4D coordinate `(Temperature, Lifecycle, Audience, Cognitive Mode)`.
- **Codified Invariant Constitution**: All non-negotiable rules must be formal, numbered invariants (`INV-*`) easily indexable and citeable by CI linters, PR reviewers, and AI agents.
- **Vertical Entity Slices**: Eliminate monolithic `profile.md` guides. Package domain extensions as self-contained capability units (ADR, RFC, Living Snapshot, Acceptance, Testing, Postmortem), each containing its own invariants, state machine, template, and SOP.
- **Zero Legacy Baggage**: Clean break without legacy compatibility compromises.

## Considered Options

- **Option 1: Incremental In-File Patching (Status Quo)**: Continue adding sections to existing `routing.md` and `profile.md` files. Rejected due to escalating cognitive debt, structural leaks, and AI context contamination.
- **Option 2: Extreme Micro-Snippet Atomization**: Shred all knowledge into hundreds of granular paragraph-sized files (`invariants/inv-01.md`, `schemas/adr-field-1.md`). Rejected because human navigation and cross-file linking become unmanageable without custom IDE compilers.
- **Option 3: Semantic Tensor Core + Vertical Domain Entities (Chosen)**:
  - Consolidate universal governance into a streamlined `core/` plane: `taxonomy.md` (4D coordinates), `invariants.md` (numbered constitution), `workflow.md` (trigger matrix, PR review gates, intake SOP, adoption), and `style.md` (syntax, voice, link rules).
  - Structure domain profiles into autonomous vertical entities (`adr.md`, `rfc.md`, `living-snapshot.md`, `acceptance.md`, `testing.md`, `postmortem.md`), each with standardized internal anatomy: Metadata, Invariants, Lifecycle/SOP, and Authoritative Template.

## Decision Outcome

Chosen option: **Option 3: Semantic Tensor Core + Vertical Domain Entities** (Protocol v5.0.0).

### Directory Topology (Protocol v5.0.0)

```text
spec/
├── .manifest.json             # Cryptographic SHA-256 verification manifest
├── AGENTS.md                  # Directory guard referencing core invariants
├── contracts.md               # Repository profile declaration & path binding template
│
├── core/                      # ─── Universal Governance Core ───
│   ├── index.md               # Protocol charter and navigation map
│   ├── taxonomy.md            # 4D coordinate tensor (Temperature, Lifecycle, Audience, Mode)
│   ├── invariants.md          # Codified constitution of numbered system invariants (INV-*)
│   ├── workflow.md            # Unified trigger matrix, PR review gates, intake SOP, adoption
│   └── style.md               # Technical voice, structural syntax, and link invariants
│
└── profiles/                  # ─── Pluggable Domain Capability Entities ───
    ├── architecture/          # Architecture Governance Profile
    │   ├── index.md           # Domain charter and entity relationship topology
    │   ├── adr.md             # ADR Entity: Invariants, state machine, negative knowledge, template
    │   ├── rfc.md             # RFC Entity: Invariants, deliberation lifecycle, 4-step archival SOP, template
    │   └── living-snapshot.md # Living Architecture Entity: Invariants, Duality, 4-step compaction SOP, template
    │
    ├── validation/            # Product Validation Profile
    │   ├── index.md           # Domain charter and dual-tier validation philosophy
    │   ├── acceptance.md      # Acceptance Entity: User journey schema, scenario matrix, template
    │   └── testing.md         # Testing Entity: Suite command table, fixture mapping, template
    │
    └── operations/            # Operational Knowledge Profile
        ├── index.md           # Domain charter and triage escalation layering
        └── postmortem.md      # Postmortem Entity: Blameless timeline, root-cause contracts, template
```

### Invariant Codification Standard

Every invariant follows the canonical identifier schema `INV-<DOMAIN>-<NUMBER>`:
- `INV-CORE-*`: Universal firewall, zero-drift, and structural invariants.
- `INV-TEMP-*`: Knowledge temperature and archival isolation invariants.
- `INV-ARCH-*`: Architectural immutability, RFC closure, and compaction invariants.
- `INV-VAL-*`: User journey parity and test verification invariants.
- `INV-TOOL-*`: Toolchain dependency and offline execution invariants.

## Consequences

### Positive
- **Deterministic Routing**: The 4D tensor eliminates all boundary ambiguities; snapshots, RFCs, and archives have explicit native coordinates.
- **Maximized AI Signal-to-Noise Ratio**: AI coding agents can load `spec/core/invariants.md` for immediate compliance enforcement without ingesting thousands of words of human prose.
- **Self-Contained Ergonomics**: When human developers author an ADR or RFC, all required schemas, constraints, and SOPs exist in a single self-contained entity file.
- **Architectural Longevity**: Clean separation between invariants (rules), taxonomy (space), workflows (time), and entities (domain capabilities) ensures future extensibility without technical debt.

### Negative / Trade-offs
- **Migration Work**: Repositories adopting v5.0.0 require running `tools/sync.sh` to relocate core files into `core/` and update profile imports. Toolchain automation handles legacy file cleanup during sync.
