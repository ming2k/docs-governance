---
id: ADR-0004
title: Protocol v6 Zero-Vendoring Architecture, Flat Knowledge Topology, and Layered Invariants
status: accepted
date: 2026-08-25
scope: core/tooling
---

# 0004. Protocol v6: Zero-Vendoring Architecture, Flat Knowledge Topology, and Layered Invariants

- Status: Accepted
- Date: 2026-08-25
- Deciders: Repository Maintainers
- Consulted: Core Contributors, AI Systems Architects
- Informed: All Adopting Repositories

## Context and Problem Statement

Protocol v5.0.0 established the 4D spatial coordinate tensor and numbered system invariants (`INV-*`). However, practical application across adopting repositories exposed significant architectural friction and legacy compromises:

1. **Vendoring Bloat & Toolchain Intrusiveness**: Adopting repositories were forced to vendor bash scripts (`tools/sync.sh`, `tools/verify.sh`), cryptographic manifests (`.manifest.json`), and mirror the entire specification under `docs/governance/documentation/`. This caused repository bloat, sync friction, and governance fatigue.
2. **Jargon Leakage & Directory Bloat**: Theoretical abstractions like `docs/governance/` or hypothetical `docs/diataxis/` leaked methodology names into directory structures, creating awkward URLs, deep nesting, and relative link friction.
3. **Crude Archival Blinding (`[INV-TEMP-02]`)**: Protocol v5 physically relocated superseded documents to `**/archive/**` and instructed AI tooling to exclude them globally. This created broken links, fragmented git history, and blinded AI agents to historical rationale ("Chesterton's Fence"), preventing root-cause retrospectives and negative knowledge retrieval.
4. **Domain-Based Invariant Conflation**: Grouping invariants by domain (`INV-CORE`, `INV-ARCH`, etc.) mixed deterministic AST-checkable rules with high-order cognitive reasoning principles, creating ambiguity for automated linters and AI agents.

An uncompromising, forward-looking architecture is needed to achieve zero-intrusiveness, flat topology, and AI-native precision.

## Decision Drivers

- **Zero-Vendoring & Decoupled Tooling**: Downstream repositories must never vendor governance scripts or spec mirrors. Tooling must exist as a standalone, zero-dependency CLI (`docgov`).
- **Scope Symmetry**: Repository-wide governance covering root sanitization and code-to-doc triggers must reside at the root (`.docgov.yml`), eliminating scope inversion.
- **Flat Knowledge Topology**: Knowledge surfaces in `docs/` must directly map to cognitive modes (`tutorials/`, `how-to/`, `reference/`, `explanation/`, `adr/`, `dev/`) without meta-jargon intermediaries.
- **Metadata Over Physical Relocation**: Deprecate physical `archive/` directories. Retain documents in-place and govern lifecycle via standard Frontmatter (`draft`, `accepted`, `superseded`, `rejected`).
- **Layered Invariant Bifurcation**: Separate invariants by execution engine into machine-checkable (`[INV-LINT-*]`) and AI/cognitive (`[INV-AGENT-*]`).

## Considered Options

- **Option 1: Incremental Patching of Protocol v5.0**: Keep vendoring scripts while softening `INV-TEMP-02`. Rejected because vendoring and directory mirroring remain fundamentally intrusive.
- **Option 2: Pure Natural Language Guidelines**: Replace codified invariant IDs with informal prose. Rejected because natural language lacks addressability, suppressibility, CI machine hooks, and LLM attention-anchor tokens.
- **Option 3: Protocol v6.0 Clean Break: Zero-Vendoring CLI + Flat Topology + Layered Invariants (Chosen)**:
  - Deliver verification via a standalone CLI (`docgov`).
  - Configure governance via a single root manifest (`.docgov.yml`).
  - Anchor AI behavior via a concise, low-entropy root `AGENTS.md` (< 30 lines).
  - Flatten `docs/` directly into cognitive quadrants and internal zones.
  - Govern document lifecycles in-place via Frontmatter metadata.
  - Codify invariants into `[INV-LINT-*]` (AST machine rules) and `[INV-AGENT-*]` (agent cognitive protocols).

## Decision Outcome

Chosen option: **Option 3: Protocol v6.0 Clean Break**.

### Repository Physical Topology (Protocol v6.0.0)

```text
my-project/
├── .docgov.yml                 # Declarative repository governance configuration
├── AGENTS.md                   # Low-entropy AI agent directives & invariant mapping (< 30 lines)
├── src/                        # Monitored source code surfaces
└── docs/                       # Flat knowledge surfaces
    ├── tutorials/              # Learning-oriented guides
    ├── how-to/                 # Task-oriented problem solving
    ├── reference/              # Information-oriented technical specs
    ├── explanation/            # Understanding-oriented architectural concepts
    ├── adr/                    # Architecture Decision Records (governed via Frontmatter)
    └── dev/                    # Internal contributor surface (protected by firewall)
        ├── setup.md            # Developer bootstrap
        ├── testing.md          # Internal testing standards
        └── postmortems/        # Blameless post-incident reviews
```

### Invariant Codification Standard (Protocol v6.0.0)

Invariants are strictly partitioned into two operational tiers:

#### Tier 1: Deterministic Machine Layer (`[INV-LINT-*]`)
Enforced deterministically by the `docgov` CLI engine in `< 50ms`:
- **`[INV-LINT-01]` Location Sanitization**: Prohibit unapproved Markdown files at the repository root; permit only whitelisted root entries (`README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `AGENTS.md`, `LICENSE.md`).
- **`[INV-LINT-02]` Contributor Firewall**: AST-parse public documentation (`docs/{tutorials,how-to,reference,explanation}/`) and disallow any relative link pointing into internal spaces (`docs/dev/**`).
- **`[INV-LINT-03]` Frontmatter Schema Integrity**: Validate that all records in `docs/adr/` contain valid Frontmatter with standardized status enums (`draft`, `accepted`, `superseded`, `rejected`).
- **`[INV-LINT-04]` Code-Doc Synchronization Trigger**: Analyze Git staged changes; if monitored source paths in `src/` are modified, verify that corresponding documentation surfaces are synchronized in the same change.
- **`[INV-LINT-05]` Agent Directives Binding**: Ensure that target AI assistant instructions (default `AGENTS.md`) contain the non-invasive docgov directives block without overwriting existing guidelines or hijacking the top-level `#` title.

#### Tier 2: Cognitive & Agent Protocol Layer (`[INV-AGENT-*]`)
Enforced by AI model reasoning, PR review templates, and human architectural review:
- **`[INV-AGENT-01]` Negative Knowledge Mandate**: Every ADR must include a dedicated `Rejected Alternatives & Negative Knowledge` section detailing why alternative solutions were discarded.
- **`[INV-AGENT-02]` Context Routing & Chesterton's Fence**:
  - *Generative Isolation*: When generating code or proposing new solutions, agents must filter out documents with `status: superseded` or `status: rejected` to prevent resurrecting dead patterns.
  - *Archeological Retrieval*: When prompted for refactoring, root-cause investigation, or "why not" questions, agents must actively retrieve superseded/rejected records as negative constraints.
- **`[INV-AGENT-03]` Blameless Postmortem Structure**: Postmortems in `docs/dev/postmortems/` must focus strictly on timeline reconstruction, detection gaps, and systemic defense failures. Personal human blame is strictly prohibited.

## Consequences

### Positive
- **Zero Repository Pollution**: Downstream repositories contain zero copied shell scripts, zero mirrored spec trees, and zero cryptographic checksum manifests.
- **URL & Navigation Ergonomics**: Flattened `docs/` eliminates meta-jargon folders (`diataxis/`, `governance/`), aligning directly with modern static site generator file-system routing.
- **Historical Integrity**: In-place Frontmatter lifecycle tracking preserves Git file history, eliminates link rot, and retains negative knowledge for AI retrieval without polluting daily prompt contexts.
- **High-Signal AI Alignment**: Partitioning invariants into machine-verifiable `LINT` rules and agent `AGENT` rules provides clear boundaries and deterministic compliance.

### Negative / Trade-offs
- **Tooling Requirement**: Requires building and distributing the standalone `docgov` CLI.
- **Specification Migration**: Existing Protocol v5.0.0 repositories require running a migration workflow to flatten paths, update frontmatter, and replace vendored tools with `.docgov.yml`.
