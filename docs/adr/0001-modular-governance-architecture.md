---
id: ADR-0001
title: Modular Governance Architecture (Micro-Core & Profile Extensions)
status: accepted
date: 2026-08-24
scope: core/modularization
---

# 0001. Modular Governance Architecture (Micro-Core & Profile Extensions)

- Status: Accepted
- Date: 2026-08-24
- Deciders: Repository Maintainers
- Consulted: Core Contributors
- Informed: All Adopting Projects

## Context and Problem Statement

`docs-governance` previously operated as a flat monolithic standard (v2.x), distributing all governance documents (`routing.md`, `style-guide.md`, `adr-workflow.md`, `knowledge-layers.md`, `validation.md`, etc.) as a uniform bundle into `docs/governance/documentation/`.

While this flat model worked for general-purpose applications, it introduced critical architectural contradictions:
1. **Scope Overreach**: Fundamental meta-governance (routing, style, review gates) was tightly coupled with specialized engineering methodologies (product validation, incident postmortems, architecture decision lifecycles).
2. **False Drift Penalties**: Downstream projects lacking UI/interaction (e.g., pure algorithmic libraries, embedded firmware) did not need `experience_scenarios.md` or postmortem frameworks, yet removing them triggered CI failure (`DRIFT: missing: ...`).
3. **Pattern-Philosophy Duplication**: `common-docs.md` duplicated templates for documents whose philosophy was simultaneously defined in top-level standalone documents.
4. **Lack of Intake Governance**: Without a clear separation between core meta-rules and domain extensions, the standard risked becoming an uncurated dumping ground for arbitrary best practices.

## Decision Drivers

- **Orthogonality & Separation of Concerns**: Core governance must remain 100% universal across all programming languages and software domains.
- **Contract-Driven Flexibility**: Downstream repositories must be able to declare which domain methodologies they adopt without being penalized for omitted unused files.
- **Strict Anti-Drift Verification**: Verification must remain cryptographic, deterministic, and offline-compatible in CI pipelines.
- **Intake Discipline**: A formal tiered admission model must gate new additions to prevent standard bloat.

## Considered Options

- **Option 1: Monolithic Flat Standard (Status Quo v2.x)**: Keep all documents in a single directory and force every adopting project to take every file.
- **Option 2: Minimalist Stripped Core (Lean)**: Remove all domain methodologies entirely from `docs-governance`, keeping only routing, style, and generic checklists.
- **Option 3: Modular Governance Architecture (Micro-Core + Domain Profile Extensions)**: Split the standard into a mandatory universal Core Protocol (`core/`), pluggable Domain Profiles (`profiles/validation`, `profiles/architecture`, `profiles/operations`), and a Contract-Driven Assembly engine driven by `contracts.md`.

## Decision Outcome

Chosen option: **Option 3: Modular Governance Architecture (Micro-Core + Domain Profile Extensions)**.

### Architecture Topology

The standard is organized into three distinct planes:

```text
standard/
├── core/                                # Universal Meta-Governance (Mandatory)
│   ├── AGENTS.md                        # Policy protection guard
│   ├── index.md                         # Core navigation & standard charter
│   ├── routing.md                       # 4-Gate priority routing cascade
│   ├── style-guide.md                   # Writing style, tone, formatting
│   ├── common-patterns.md               # Baseline repository document patterns (README, setup, etc.)
│   ├── review-checklist.md              # Universal PR review gates
│   ├── update-checklist.md              # Code-to-doc change trigger matrix
│   └── intake-sop.md                    # Tier 0-3 standard admission SOP
│
├── profiles/                            # Domain Methodology Extensions (Pluggable)
│   ├── validation/                      # Product Validation Profile (Journeys, Scenarios, Testing)
│   │   ├── profile.md                   # Validation philosophy ("Skip journey, not experience")
│   │   ├── acceptance.pattern.md        # acceptance.md pattern & template
│   │   ├── scenarios.pattern.md         # experience_scenarios.md pattern & template
│   │   └── testing.pattern.md           # testing.md pattern & template
│   │
│   ├── architecture/                    # Architecture Decision Profile (ADRs)
│   │   ├── profile.md                   # Immutable decision lifecycle & immutability philosophy
│   │   └── adr.pattern.md               # ADR structure & index pattern
│   │
│   └── operations/                      # Operational Knowledge Profile (Incident Triage & Postmortems)
│       ├── profile.md                   # Knowledge layering (Triage -> Postmortem -> ADR)
│       └── postmortem.pattern.md        # Postmortem structure & template
│
├── contracts.md                         # Repository Contract Template (Editable)
└── .manifest.json                       # Version 3.0.0 cryptographic manifest
```

### Contract-Driven Assembly

Downstream repositories declare active profiles in `docs/governance/documentation/contracts.md`:
```markdown
## Activated Profiles
- [x] core (Always active)
- [x] validation
- [x] architecture
- [ ] operations
```

- `sync.sh` synchronizes `core/` and the explicitly activated profiles under `profiles/`.
- `verify.sh` inspects `contracts.md`, verifies all files in `core/` against `.manifest.json`, and verifies only the active profiles. Inactive profiles are ignored, eliminating false-positive drift.

### Tiered Intake SOP (Standard Evolution)

New content must pass formal admission gates:
- **Tier 0 (Core Protocol)**: Universal meta-rules affecting all projects (Major/Minor version bump).
- **Tier 1 (Core Patterns)**: Single document patterns universal to >=90% of repositories (`common-patterns.md`).
- **Tier 2 (Domain Profiles)**: Cohesive, standalone engineering methodologies packaged in `profiles/<name>/`.
- **Tier 3 (Project-Specific)**: Technologies, frameworks, or domain rules that belong exclusively in the adopter's private governance (`docs/governance/<domain>.md`).

### Consequences

#### Positive
- Universal core is lightweight, unopinionated, and universally applicable.
- Downstream repositories configure exactly what they need without drift penalties.
- Domain methodologies are cohesive packages containing both architectural philosophy and structural document patterns.
- Clear admission criteria prevent standard bloat.

#### Negative
- Toolchain (`sync.sh`, `verify.sh`, `update-hashes.sh`) requires parsing `contracts.md` profile declarations.
- Upgrading from v2.x to v3.0.0 requires updating downstream directory paths and manifest version (bumped to 3.0.0).

## Links

- Protocol Version: `3.0.0`
- Implementation PR/Commit: Modular Governance Architecture Refactoring
