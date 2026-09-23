use crate::diagnostics::Diagnostic;
use crate::rules::{LintContext, Rule};
use crate::utils::matches_glob;
use anyhow::Result;
use serde_yaml::Value;

pub struct FrontmatterSchemaRule;

impl Rule for FrontmatterSchemaRule {
    fn id(&self) -> &'static str {
        "INV-LINT-03"
    }

    fn description(&self) -> &'static str {
        "Frontmatter Schema & Lifecycle Integrity: ADRs must declare valid frontmatter and status"
    }

    fn check(&self, ctx: &LintContext) -> Result<Vec<Diagnostic>> {
        let mut diagnostics = Vec::new();
        let adr_glob = format!(
            "{}/**/*.md",
            ctx.config.architecture.adr_path.trim_end_matches('/')
        );

        for (rel_path, doc) in ctx.markdown_docs {
            if !matches_glob(rel_path, &adr_glob) {
                continue;
            }

            let file_name = rel_path.file_name().unwrap_or_default().to_string_lossy();
            if file_name == "index.md" || file_name == "README.md" || file_name == "template.md" {
                continue;
            }

            // 1. Must have Frontmatter
            let fm = match &doc.frontmatter {
                Some(f) => f,
                None => {
                    diagnostics.push(
                        Diagnostic::error(
                            self.id(),
                            "Missing YAML frontmatter in ADR",
                            rel_path,
                        )
                        .with_location(1, 1)
                        .with_suggestion("Add frontmatter block (---) with 'id', 'title', 'status', and 'date'."),
                    );
                    continue;
                }
            };

            // 2. Frontmatter must be a mapping
            let map = match &fm.value {
                Value::Mapping(m) => m,
                _ => {
                    diagnostics.push(
                        Diagnostic::error(
                            self.id(),
                            "Frontmatter must be a YAML key-value mapping",
                            rel_path,
                        )
                        .with_location(1, 1),
                    );
                    continue;
                }
            };

            // 3. Mandatory fields check
            for field in &ctx.config.architecture.require_frontmatter.mandatory_fields {
                if !map.contains_key(field.as_str()) || map.get(field.as_str()).unwrap().is_null() {
                    diagnostics.push(
                        Diagnostic::error(
                            self.id(),
                            format!("Missing mandatory frontmatter field: '{}'", field),
                            rel_path,
                        )
                        .with_location(fm.start_line, 1)
                        .with_suggestion(format!("Declare '{}' in the frontmatter header.", field)),
                    );
                }
            }

            // 4. Status enum check
            if let Some(status_val) = map.get("status") {
                let status_str = status_val.as_str().unwrap_or("").to_lowercase();
                let valid_enum = &ctx.config.architecture.require_frontmatter.status_enum;
                if !valid_enum.contains(&status_str) {
                    diagnostics.push(
                        Diagnostic::error(
                            self.id(),
                            format!(
                                "Invalid status '{}'. Must be one of: {}",
                                status_str,
                                valid_enum.join(", ")
                            ),
                            rel_path,
                        )
                        .with_location(fm.start_line, 1),
                    );
                }

                // 5. If superseded, require superseded_by
                if status_str == "superseded" {
                    let has_valid_superseded_by = match map.get("superseded_by") {
                        Some(val) => {
                            if let Some(s) = val.as_str() {
                                !s.trim().is_empty()
                            } else {
                                false
                            }
                        }
                        None => false,
                    };

                    if !has_valid_superseded_by {
                        diagnostics.push(
                            Diagnostic::error(
                                self.id(),
                                "ADR marked as 'superseded' must provide a valid 'superseded_by' pointer (e.g. 'ADR-0042')",
                                rel_path,
                            )
                            .with_location(fm.start_line, 1)
                            .with_suggestion("Add 'superseded_by: ADR-NNNN' to frontmatter."),
                        );
                    }
                }
            }
        }

        Ok(diagnostics)
    }
}
