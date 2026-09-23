---
id: ADR-0005
title: Decouple Engine from Embedded Assets and Enforce Single Source of Truth
status: accepted
date: 2026-09-23
scope: core/tooling
---

# 0005. Decouple Engine from Embedded Assets and Enforce Single Source of Truth

- Status: Accepted
- Date: 2026-09-23
- Deciders: Repository Maintainers, AI Systems Architects
- Consulted: Core Contributors
- Informed: All Adopting Repositories

## Context and Problem Statement

To provide cold-start convenience and offline tolerance during early iterations, `docgov` embedded upstream governance specifications directly into the compiled binary via `build.rs` and `include_bytes!(docgov-assets.tar.gz)`. Additionally, the default AI directives snippet was duplicated as a hardcoded Rust string constant (`DOCGOV_DIRECTIVES_SNIPPET`).

While well-intentioned for initial standalone trials, this introduced severe architectural flaws:
1. **Silent Version Drift (双源真理与版本脱节)**: If an adopting repository configured `spec_ref = "v0.0.1"` but ran a newer `docgov` CLI, network latency or pending Release asset generation would trigger the embedded fallback, silently replacing downstream's declared contract with the CLI binary's compile-time version.
2. **Private Upstream Specification Pollution**: For enterprise repositories with proprietary upstream governance profiles (`spec_source = "github:corp/governance"`), falling back to embedded assets would overwrite internal corporate standards with public defaults.
3. **Dual Source of Truth & Maintenance Burden**: Directives existed in two places: `spec/directives.snippet` and Rust source code. Any update required manual code synchronization and a full CLI binary release cycle.
4. **Engine-Specification Inversion**: A protocol linter engine should remain an agnostic execution runtime; binding canonical documentation data into the binary violated separation of concerns.

## Decision Drivers

- **Zero Silent Drift**: Downstream configuration contracts (`spec_source` and `spec_ref`) must be strictly honored or fail deterministically.
- **Single Source of Truth**: Specification text and directives must live solely within versioned protocol repositories, never duplicated as source code constants.
- **Pure Decoupled Engine**: The `docgov` executable must be a clean, lightweight verification and synchronization engine without embedded data blobs.
- **First-Class Local Sources**: Support local filesystem paths (`source = "."` / `"local"`) for origin dogfooding and air-gapped environments without phantom network dependencies.

## Considered Options

- **Option 1: Retain Embedded Baseline with Version Matching Checks**: Only fall back if the requested `spec_ref` matches the binary's version string.
  - *Rejected*: Still maintains dual-source-of-truth in source code, bloats binary size, and requires CLI releases whenever specification prose is adjusted.
- **Option 2: Embed Only Minimal Directives, Fetch Rest Remotely**: Keep `DOCGOV_DIRECTIVES_SNIPPET` in Rust and fetch tarballs remotely.
  - *Rejected*: Compromised solution. Leaves the directives snippet drifting between `spec/directives.snippet` and Rust code.
- **Option 3: Clean Decoupling with Strict Contract Resolution (Chosen)**:
  - Delete `build.rs`, `EMBEDDED_ASSETS_TAR_GZ`, and `DOCGOV_DIRECTIVES_SNIPPET`.
  - Resolution chain: Local source (if configured or workspace spec exists) -> Local cache (`~/.cache/docgov/...`) -> Remote release asset / raw git source -> Explicit deterministic error.
  - First-class local path support for self-hosting in meta-repositories.

## Decision Outcome

Chosen option: **Option 3: Clean Decoupling with Strict Contract Resolution**.

### Architectural Contract

1. **Resolution Pipeline**:
   - `fetch_directives()` and `sync_governance_docs()` query local spec directories first if `upstream.source` is local (`.` / `local` / `file://`).
   - For remote upstreams, queries check `~/.cache/docgov/{source}/{ref}/`.
   - On cache miss, requests GitHub Releases / raw git endpoints.
   - If unresolvable, returns an explicit error with all attempted endpoints. Never silently substitutes embedded fallback data.
2. **Deterministic Locking**:
   - Every synchronization records exact SHA-256 digests in `.docgov.lock`.
   - `docgov check` verifies local state against `.docgov.lock` without making network calls.

## Rejected Alternatives

- **Silent Embedded Fallback**: Rejected because it violates semantic versioning contracts and creates silent configuration drift.
- **Hardcoded Markdown Constants in Rust Source**: Rejected because documentation standards must evolve independently of compiler release cycles.

## Pros and Cons of the Selected Architecture

### Pros
- **Architectural Purity**: Elimination of `build.rs` accelerates build times, trims binary size, and cleans repository footprint.
- **Auditable Predictability**: Downstream repositories get exactly what their lockfile and `.docgov.yml` declare.
- **Zero Drift Risk**: Single source of truth for directives and specification documents.
- **Seamless Local Development**: Origin developers can sync and test modifications to `spec/` instantly using `docgov sync --local` or `source: "."`.

### Cons
- Initial cold-start of a brand new downstream repository requires either internet connectivity to fetch release assets once or an existing local cache. (Mitigated by persistent caching in `~/.cache/docgov/`).
