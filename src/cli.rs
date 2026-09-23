use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::*;
use std::path::PathBuf;

use crate::config::Config;
use crate::diagnostics::Severity;
use crate::engine::LintEngine;
use crate::git::get_changed_files;
use crate::lockfile::{compute_sha256, ArtifactEntry, DocgovLock};
use crate::remote::RemoteClient;
use crate::rules::lint_05_agent_directives::{
    patch_agent_directives, PatchAction, DOCGOV_DIRECTIVES_BEGIN, DOCGOV_DIRECTIVES_END,
    DOCGOV_DIRECTIVES_SNIPPET,
};

#[derive(Parser, Debug)]
#[command(
    name = "docgov",
    version,
    about = "High-performance, zero-vendoring documentation and architecture governance linter",
    long_about = "A fast, deterministic compiler-grade linter for Protocol v0.0.2 documentation governance and system invariants."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to target workspace directory (defaults to current directory)
    #[arg(short, long, global = true)]
    pub path: Option<PathBuf>,

    /// Output format
    #[arg(short, long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Text,
    Github,
    Json,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check repository compliance with all Protocol v0.0.2 invariants
    Check {
        /// Also run git-diff trigger matrix checks
        #[arg(long)]
        diff: bool,

        /// Base git reference to compare against (e.g. 'origin/main', 'HEAD~1')
        #[arg(long)]
        base: Option<String>,
    },

    /// Run only git-diff code-to-doc trigger matrix checks
    Diff {
        /// Base git reference to compare against (e.g. 'origin/main', 'HEAD~1')
        #[arg(default_value = "HEAD~1")]
        base: String,
    },

    /// Initialize standard .docgov.yml configuration and AGENTS.md in the current repository
    Init {
        /// Overwrite existing configuration or refresh directive blocks
        #[arg(long, short = 'F')]
        force: bool,
    },

    /// Sync and atomically update canonical governance documentation and directives
    Sync {
        /// Force re-fetch and re-download assets
        #[arg(long, short = 'F')]
        force: bool,
    },
}

pub fn run() -> Result<i32> {
    let cli = Cli::parse();
    let target_dir = cli.path.unwrap_or_else(|| PathBuf::from("."));

    match cli.command.unwrap_or(Commands::Check {
        diff: false,
        base: None,
    }) {
        Commands::Check { diff, base } => {
            let start = std::time::Instant::now();
            let engine = LintEngine::new(&target_dir)?;

            let changed = if diff || base.is_some() {
                Some(get_changed_files(&target_dir, base.as_deref())?)
            } else {
                None
            };

            let diagnostics = engine.run_lint(changed.as_deref())?;
            let duration = start.elapsed();

            output_diagnostics(&diagnostics, cli.format, duration);

            let error_count = diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .count();
            if error_count > 0 {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Commands::Diff { base } => {
            let start = std::time::Instant::now();
            let engine = LintEngine::new(&target_dir)?;
            let changed = get_changed_files(&target_dir, Some(&base))?;

            // Run only git trigger rule
            let ctx = crate::rules::LintContext {
                workspace_root: &engine.workspace_root,
                config: &engine.config,
                root_files: &[],
                markdown_docs: &std::collections::HashMap::new(),
                changed_files: Some(&changed),
            };

            let rule = crate::rules::lint_04_trigger::GitSyncTriggerRule;
            use crate::rules::Rule;
            let diagnostics = rule.check(&ctx)?;
            let duration = start.elapsed();

            output_diagnostics(&diagnostics, cli.format, duration);

            let error_count = diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .count();
            if error_count > 0 {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Commands::Init { force } => {
            init_repo(&target_dir, force)?;
            Ok(0)
        }
        Commands::Sync { force } => {
            sync_repo(&target_dir, force)?;
            Ok(0)
        }
    }
}

fn output_diagnostics(
    diagnostics: &[crate::diagnostics::Diagnostic],
    format: OutputFormat,
    duration: std::time::Duration,
) {
    match format {
        OutputFormat::Text => {
            if diagnostics.is_empty() {
                println!(
                    "{} All Protocol v0.0.2 documentation invariants verified in {:.3}s.",
                    "✔".green().bold(),
                    duration.as_secs_f64()
                );
                return;
            }

            println!();
            for diag in diagnostics {
                println!("{}", diag.render_terminal());
            }

            let errors = diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .count();
            let warnings = diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Warning)
                .count();

            println!(
                "{} Found {} error(s) and {} warning(s) in {:.3}s. (Check failed)\n",
                "✖".red().bold(),
                errors,
                warnings,
                duration.as_secs_f64()
            );
        }
        OutputFormat::Github => {
            for diag in diagnostics {
                println!("{}", diag.render_github());
            }
        }
        OutputFormat::Json => {
            if let Ok(json) = serde_json::to_string_pretty(diagnostics) {
                println!("{}", json);
            }
        }
    }
}

fn init_repo(dir: &std::path::Path, force: bool) -> Result<()> {
    let docgov_yml = dir.join(".docgov.yml");

    let yml_content = r#"version: "0.0.2"

# Remote Upstream & Protocol Distribution
upstream:
  source: "https://github.com/ming2k/docs-governance"
  ref: "v0.0.2"

# Canonical Governance Documentation Mirror (for Agent Context)
governance_docs:
  install: true
  target_dir: "docs/governance/documentation"

# [INV-LINT-01] Root Location Sanitization
root_sanitization:
  enforce: true
  allowed_markdown:
    - "README.md"
    - "CHANGELOG.md"
    - "CONTRIBUTING.md"
    - "AGENTS.md"
    - "LICENSE.md"
    - "SECURITY.md"

# [INV-LINT-02] Contributor Firewall Bindings
firewall:
  public_surfaces:
    - "docs/tutorials/**"
    - "docs/how-to/**"
    - "docs/reference/**"
    - "docs/explanation/**"
  internal_surfaces:
    - "docs/dev/**"

# [INV-LINT-03] Architecture & Metadata Profile
architecture:
  adr_path: "docs/adr"
  require_frontmatter:
    status_enum: ["draft", "accepted", "superseded", "rejected", "deprecated"]
    mandatory_fields: ["id", "title", "status", "date"]

# [INV-LINT-05] Agent Directives Binding
agent_directives:
  enforce: true
  targets:
    - "AGENTS.md"

# [INV-LINT-04] Code-to-Doc Trigger Bindings
triggers:
  - watch: "src/api/**"
    require_update: "docs/reference/**"
    message: "Public API modified; docs/reference/ must be synchronized in the same commit."
  - watch: "src/cli/**"
    require_update: "docs/how-to/**"
    message: "CLI syntax changed; docs/how-to/ must be synchronized in the same commit."
"#;

    if !docgov_yml.exists() || force {
        std::fs::write(&docgov_yml, yml_content)?;
        println!("{} Created {}", "+".green().bold(), docgov_yml.display());
    } else {
        println!("{} Exists: {}", "~".yellow().bold(), docgov_yml.display());
    }

    sync_repo(dir, force)
}

fn sync_repo(dir: &std::path::Path, force: bool) -> Result<()> {
    // Load configuration to discover agent directive targets, upstream and governance docs settings
    let (cfg, _) = Config::load_from_dir(dir).unwrap_or((Config::default(), None));

    // Fetch directives using remote client (checks local cache first, fallback to embedded)
    let remote_client = RemoteClient::new(&cfg.upstream.source, &cfg.upstream.r#ref);
    let (directives_content, _source_info) =
        remote_client.fetch_directives(DOCGOV_DIRECTIVES_SNIPPET)?;

    let targets = if cfg.agent_directives.targets.is_empty() {
        vec!["AGENTS.md".to_string()]
    } else {
        cfg.agent_directives.targets
    };

    let mut lock = DocgovLock::load_from_dir(dir)?.unwrap_or_else(|| {
        DocgovLock::new(&cfg.version, &cfg.upstream.source, &cfg.upstream.r#ref)
    });
    lock.protocol_version = cfg.version.clone();
    lock.upstream.source = cfg.upstream.source.clone();
    lock.upstream.r#ref = cfg.upstream.r#ref.clone();

    // 1. Synchronize agent directives (non-invasively, preserving custom guidelines & single #)
    for target in targets {
        let target_path = dir.join(&target);
        let existing = if target_path.exists() {
            Some(std::fs::read_to_string(&target_path)?)
        } else {
            None
        };

        let (new_content, action) =
            patch_agent_directives(existing.as_deref(), Some(&directives_content));

        // Compute hash of the directive block inside the patched content
        if let Some(b) = new_content.find(DOCGOV_DIRECTIVES_BEGIN) {
            if let Some(e_rel) = new_content[b..].find(DOCGOV_DIRECTIVES_END) {
                let e = b + e_rel + DOCGOV_DIRECTIVES_END.len();
                let block_hash = compute_sha256(new_content[b..e].trim());
                lock.artifacts.agent_directives = Some(ArtifactEntry {
                    target: target.clone(),
                    hash: block_hash,
                });
            }
        }

        match action {
            PatchAction::Created => {
                std::fs::write(&target_path, &new_content)?;
                println!(
                    "{} Created {} with docgov directives block",
                    "+".green().bold(),
                    target_path.display()
                );
            }
            PatchAction::Appended => {
                std::fs::write(&target_path, &new_content)?;
                println!(
                    "{} Non-invasively inserted docgov directives block into {}",
                    "+".green().bold(),
                    target_path.display()
                );
            }
            PatchAction::Updated => {
                std::fs::write(&target_path, &new_content)?;
                println!(
                    "{} Updated docgov directives block in {}",
                    "~".yellow().bold(),
                    target_path.display()
                );
            }
            PatchAction::Unchanged => {
                if force {
                    std::fs::write(&target_path, &new_content)?;
                    println!(
                        "{} Refreshed docgov directives block in {}",
                        "~".yellow().bold(),
                        target_path.display()
                    );
                } else {
                    println!(
                        "{} Directives up to date: {}",
                        "✔".green().bold(),
                        target_path.display()
                    );
                }
            }
        }
    }

    // 2. Synchronize canonical governance documentation mirror (full atomic mirror replacement & pruning)
    if cfg.governance_docs.install {
        let archive_hash =
            remote_client.sync_governance_docs(dir, &cfg.governance_docs.target_dir, force)?;
        lock.artifacts.governance_docs = Some(ArtifactEntry {
            target: cfg.governance_docs.target_dir.clone(),
            hash: archive_hash,
        });
    }

    lock.save_to_dir(dir)?;
    println!(
        "{} Updated {}",
        "✔".green().bold(),
        DocgovLock::lockfile_path(dir).display()
    );

    Ok(())
}
