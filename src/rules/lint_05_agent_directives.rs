use crate::diagnostics::Diagnostic;
use crate::lockfile::{compute_sha256, DocgovLock};
use crate::rules::{LintContext, Rule};
use anyhow::Result;

pub const DOCGOV_DIRECTIVES_BEGIN: &str = "<!-- BEGIN DOCGOV DIRECTIVES -->";
pub const DOCGOV_DIRECTIVES_END: &str = "<!-- END DOCGOV DIRECTIVES -->";

pub const DOCGOV_DIRECTIVES_SNIPPET: &str = r#"<!-- BEGIN DOCGOV DIRECTIVES -->
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
<!-- END DOCGOV DIRECTIVES -->"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchAction {
    Created,
    Appended,
    Updated,
    Unchanged,
}

pub fn patch_agent_directives(
    existing_content: Option<&str>,
    custom_snippet: Option<&str>,
) -> (String, PatchAction) {
    let snippet = custom_snippet.unwrap_or(DOCGOV_DIRECTIVES_SNIPPET).trim();

    match existing_content {
        None => {
            let content = format!("# Agent Directives\n\n{}\n", snippet);
            (content, PatchAction::Created)
        }
        Some(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                let content = format!("# Agent Directives\n\n{}\n", snippet);
                return (content, PatchAction::Created);
            }

            if let Some(begin_idx) = text.find(DOCGOV_DIRECTIVES_BEGIN) {
                if let Some(end_rel) = text[begin_idx..].find(DOCGOV_DIRECTIVES_END) {
                    let end_idx = begin_idx + end_rel + DOCGOV_DIRECTIVES_END.len();
                    let before = &text[..begin_idx];
                    let after = &text[end_idx..];
                    let current_block = &text[begin_idx..end_idx];
                    if current_block.trim() == snippet {
                        return (text.to_string(), PatchAction::Unchanged);
                    }
                    let mut result = String::new();
                    result.push_str(before);
                    result.push_str(snippet);
                    result.push_str(after);
                    return (result, PatchAction::Updated);
                }
            }

            // Marker does not exist: append non-invasively
            let mut result = text.trim_end().to_string();
            result.push_str("\n\n");
            result.push_str(snippet);
            result.push('\n');
            (result, PatchAction::Appended)
        }
    }
}

pub struct AgentDirectivesRule;

impl Rule for AgentDirectivesRule {
    fn id(&self) -> &'static str {
        "INV-LINT-05"
    }

    fn description(&self) -> &'static str {
        "Agent Directives Binding: AI assistant configuration must contain docgov anchor directives"
    }

    fn check(&self, ctx: &LintContext) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();

        if !ctx.config.agent_directives.enforce {
            return Ok(diagnostics);
        }

        let lock = DocgovLock::load_from_dir(ctx.workspace_root).ok().flatten();

        for target in &ctx.config.agent_directives.targets {
            let target_path = ctx.workspace_root.join(target);
            let rel_target = std::path::PathBuf::from(target);

            if !target_path.exists() {
                diagnostics.push(
                    Diagnostic::error(
                        self.id(),
                        format!("Missing agent directives configuration file '{}'", target),
                        rel_target,
                    )
                    .with_suggestion("Run `docgov init` to create it with the non-invasive docgov directives block."),
                );
                continue;
            }

            let content = match std::fs::read_to_string(&target_path) {
                Ok(c) => c,
                Err(_) => {
                    diagnostics.push(Diagnostic::error(
                        self.id(),
                        format!("Failed to read agent configuration file '{}'", target),
                        rel_target,
                    ));
                    continue;
                }
            };

            let begin_opt = content.find(DOCGOV_DIRECTIVES_BEGIN);
            let end_opt = begin_opt.and_then(|b| {
                content[b..]
                    .find(DOCGOV_DIRECTIVES_END)
                    .map(|e| b + e + DOCGOV_DIRECTIVES_END.len())
            });

            match (begin_opt, end_opt) {
                (Some(begin_idx), Some(end_idx)) => {
                    // Check against lockfile if present
                    if let Some(ref lockfile) = lock {
                        if let Some(ref artifact) = lockfile.artifacts.agent_directives {
                            if artifact.target == *target {
                                let block_slice = content[begin_idx..end_idx].trim();
                                let actual_hash = compute_sha256(block_slice);
                                if actual_hash != artifact.hash {
                                    let line_num = content[..begin_idx].lines().count().max(1);
                                    diagnostics.push(
                                        Diagnostic::warning(
                                            self.id(),
                                            format!(
                                                "Directives block in '{}' drifted from .docgov.lock (expected {}, got {})",
                                                target, artifact.hash, actual_hash
                                            ),
                                            rel_target.clone(),
                                        )
                                        .with_location(line_num, 1)
                                        .with_suggestion("Run `docgov init` to re-synchronize directives with upstream lockfile."),
                                    );
                                }
                            }
                        }
                    }
                }
                _ => {
                    let line_num = content.lines().count().max(1);
                    diagnostics.push(
                        Diagnostic::error(
                            self.id(),
                            format!(
                                "Missing docgov anchor directives block in '{}'",
                                target
                            ),
                            rel_target,
                        )
                        .with_location(line_num, 1)
                        .with_suggestion("Run `docgov init` to insert the non-invasive docgov directives block without overwriting existing instructions."),
                    );
                }
            }
        }

        Ok(diagnostics)
    }
}
