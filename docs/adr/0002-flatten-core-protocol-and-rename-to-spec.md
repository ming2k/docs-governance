---
id: ADR-0002
title: Flatten Core Protocol and Rename Standard to Spec
status: accepted
date: 2026-08-25
scope: core/topology
---

# 0002. Flatten Core Protocol and Rename Standard to Spec

- Status: Accepted
- Date: 2026-08-25
- Deciders: Repository Maintainers
- Consulted: Core Contributors
- Informed: All Adopting Projects

## Context and Problem Statement

In Protocol v3.0.0 (ADR-0001), `docs-governance` decoupled monolithic governance into a micro-core protocol and pluggable domain profiles. To enforce physical isolation between mandatory rules and optional extensions, the core protocol was placed in a dedicated `core/` subdirectory (`standard/core/`).

While this structure achieved domain separation, real-world adoption revealed friction:
1. **Excessive Nesting**: Downstream repositories required deeply nested paths (`docs/governance/documentation/core/routing.md`), imposing friction on documentation readers and AI agent toolchains.
2. **Missing Root Index**: In `docs/governance/documentation/`, only `contracts.md` and `AGENTS.md` resided at the root level; the canonical entry point `index.md` was nested inside `core/`, degrading navigation UX across web documentation viewers.
3. **Repository Directory Semantics**: The upstream source directory was named `standard/`, which was redundant with the repository name and less intuitive than standard engineering specification directories (`spec/`).

## Decision Drivers

- **Navigation & Human UX**: The root documentation governance directory must provide an immediate `index.md` entry point without requiring subfolder traversal.
- **Path Ergonomics**: Minimize path depth for universal core governance files while preserving clean separation for optional profiles.
- **Clear Engineering Semantics**: The upstream directory should accurately express specification assets (`spec/`).
- **Seamless Migration**: Toolchain must automatically detect and clean up legacy v3 `core/` directories during sync.

## Considered Options

- **Option 1: Status Quo (Keep `standard/core/`)**: Retain separate `core/` folder for maximum physical isolation.
- **Option 2: Rename `standard/` to `src/` and keep `core/`**: Adopt conventional `src/` but leave path nesting unresolved.
- **Option 3: Rename `standard/` to `spec/` and flatten `core/` to root (Chosen)**:
  - Upstream asset directory renamed to `spec/`.
  - Core protocol files (`index.md`, `routing.md`, `style-guide.md`, `common-patterns.md`, `review-checklist.md`, `update-checklist.md`, `adoption.md`, `intake-sop.md`) live directly at the root of `spec/` and downstream `docs/governance/documentation/`.
  - Pluggable domain profiles remain isolated in `profiles/<name>/`.
  - Bump protocol version to `4.0.0` (Major version bump) and equip `tools/sync.sh` with automatic legacy `core/` directory cleanup.

## Decision Outcome

Chosen option: **Option 3: Rename `standard/` to `spec/` and flatten `core/` to root**.

### Directory Topology (Protocol v4.0.0)

```text
spec/
├── .manifest.json          # Protocol v4.0.0 manifest & cryptographic SHA-256 signatures
├── AGENTS.md               # Directory-level AI policy protection guard
├── contracts.md            # Activated profiles and repository bindings template
├── index.md                # Immediate documentation governance index & catalog
├── routing.md              # 4-Gate priority routing cascade
├── style-guide.md          # Voice, style, and formatting rules
├── common-patterns.md      # Structural pattern library for recurring repository docs
├── review-checklist.md     # PR review gates
├── update-checklist.md     # Code-to-doc change trigger matrix
├── adoption.md             # Adoption guide & checklist
├── intake-sop.md           # Tiered standard admission SOP
└── profiles/               # Pluggable Domain Profiles
    ├── validation/         # Product validation (acceptance vs testing)
    ├── architecture/       # Architecture Decision Records (ADR lifecycle & compaction)
    └── operations/         # Operational knowledge layering (triage, postmortems)
```

### Consequences

#### Positive
- Immediate discovery: Opening `docs/governance/documentation/` immediately displays `index.md`.
- Reduced path depth: Core routing and style guidelines live at `docs/governance/documentation/*.md`.
- Clean relative linking: Sibling files no longer require awkward `../` traversal to reach `contracts.md` or `profiles/`.
- Clear directory semantics: `spec/` explicitly communicates specification assets without confusing software `src/` expectations.
- Automatic migration: `tools/sync.sh` automatically removes legacy `core/` directory from downstream repositories during synchronization.

#### Negative / Trade-offs
- Breaking change: Requires Protocol v4.0.0 version bump; existing v3 repos will show drift until synchronized with upgraded toolchain.
