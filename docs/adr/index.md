# Architecture Decision Records

This directory records immutable architectural decisions for `docs-governance`.

For the full decision lifecycle, see [ADR Profile](../governance/documentation/profiles/architecture/adr.md) (or `docs/governance/documentation/profiles/architecture/adr.md`).

## Index

| ID | Title | Status | Scope | Decision Summary & Primary Invariant | Date | Living Snapshot |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| [0001](0001-modular-governance-architecture.md) | Modular Governance Architecture (Micro-Core & Profile Extensions) | Accepted | core | Decoupled monolithic governance into universal Micro-Core and pluggable Domain Profiles activated via `contracts.md`. | 2026-08-24 | - |
| [0002](0002-flatten-core-protocol-and-rename-to-spec.md) | Flatten Core Protocol and Rename Standard to Spec | Accepted | spec | Renamed upstream asset directory to `spec/` and flattened core layout to improve root indexing and path ergonomics. | 2026-08-25 | - |
| [0003](0003-clean-break-architecture-semantic-tensor-and-vertical-entities.md) | Clean-Break Architecture: Semantic Tensor Core and Vertical Domain Entities | Accepted | core, profiles | Established 4D coordinate tensor core, codified numbered `[INV-*]` invariants, and decoupled profiles into autonomous vertical entity slices. | 2026-08-25 | - |
| [0004](0004-protocol-v6-zero-vendoring-and-layered-invariants.md) | Protocol v6: Zero-Vendoring Architecture, Flat Knowledge Topology, and Layered Invariants | Accepted | core, tooling, agents | Eliminated vendoring bloat, flattened docs layout, introduced frontmatter lifecycle in-place, and bifurcated invariants into `[INV-LINT-*]` and `[INV-AGENT-*]`. | 2026-08-25 | - |
