use docgov::config::Config;
use docgov::engine::LintEngine;
use docgov::rules::lint_04_trigger::GitSyncTriggerRule;
use docgov::rules::lint_05_agent_directives::{
    patch_agent_directives, PatchAction, DOCGOV_DIRECTIVES_BEGIN, DOCGOV_DIRECTIVES_END,
};
use docgov::rules::LintContext;
use docgov::rules::Rule;
use std::fs;
use tempfile::tempdir;

const SAMPLE_SNIPPET: &str = include_str!("../spec/directives.snippet");

#[test]
fn test_lint_01_root_sanitizer() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create allowed file and unapproved file
    fs::write(root.join("README.md"), "# Hello").unwrap();
    fs::write(root.join("FORBIDDEN.md"), "# Secret Notes").unwrap();

    let engine = LintEngine::new(root).unwrap();
    let diagnostics = engine.run_lint(None).unwrap();

    let root_errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-01")
        .collect();

    assert_eq!(root_errors.len(), 1);
    assert!(root_errors[0].message.contains("FORBIDDEN.md"));
}

#[test]
fn test_lint_02_contributor_firewall() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Setup public and internal docs
    let tut_dir = root.join("docs/tutorials");
    let dev_dir = root.join("docs/dev");
    fs::create_dir_all(&tut_dir).unwrap();
    fs::create_dir_all(&dev_dir).unwrap();

    fs::write(dev_dir.join("setup.md"), "# Setup").unwrap();
    // Public doc linking into internal dev space
    fs::write(
        tut_dir.join("quickstart.md"),
        "# Quickstart\n\nFor details, see [Dev Setup](../dev/setup.md).\n",
    )
    .unwrap();

    let engine = LintEngine::new(root).unwrap();
    let diagnostics = engine.run_lint(None).unwrap();

    let firewall_errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-02")
        .collect();

    assert_eq!(firewall_errors.len(), 1);
    assert_eq!(firewall_errors[0].line, Some(3));
    assert!(firewall_errors[0].message.contains("docs/dev/setup.md"));
}

#[test]
fn test_lint_03_frontmatter_validation() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let adr_dir = root.join("docs/adr");
    fs::create_dir_all(&adr_dir).unwrap();

    // 1. Missing frontmatter
    fs::write(adr_dir.join("0001-no-fm.md"), "# ADR 1\nContent").unwrap();

    // 2. Missing mandatory field
    fs::write(
        adr_dir.join("0002-missing-id.md"),
        "---\ntitle: Foo\nstatus: accepted\ndate: 2026-08-25\n---\n# ADR 2",
    )
    .unwrap();

    // 3. Invalid status enum
    fs::write(
        adr_dir.join("0003-bad-status.md"),
        "---\nid: ADR-0003\ntitle: Bar\nstatus: finished\ndate: 2026-08-25\n---\n# ADR 3",
    )
    .unwrap();

    // 4. Superseded without superseded_by
    fs::write(
        adr_dir.join("0004-superseded-no-pointer.md"),
        "---\nid: ADR-0004\ntitle: Baz\nstatus: superseded\ndate: 2026-08-25\n---\n# ADR 4",
    )
    .unwrap();

    // 5. Valid ADR
    fs::write(
        adr_dir.join("0005-valid.md"),
        "---\nid: ADR-0005\ntitle: Qux\nstatus: accepted\ndate: 2026-08-25\n---\n# ADR 5",
    )
    .unwrap();

    let engine = LintEngine::new(root).unwrap();
    let diagnostics = engine.run_lint(None).unwrap();

    let adr_errors: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-03")
        .collect();

    assert_eq!(adr_errors.len(), 4);
    assert!(adr_errors
        .iter()
        .any(|d| d.message.contains("Missing YAML frontmatter")));
    assert!(adr_errors.iter().any(|d| d
        .message
        .contains("Missing mandatory frontmatter field: 'id'")));
    assert!(adr_errors
        .iter()
        .any(|d| d.message.contains("Invalid status 'finished'")));
    assert!(adr_errors.iter().any(|d| d
        .message
        .contains("must provide a valid 'superseded_by' pointer")));
}

#[test]
fn test_lint_04_git_sync_trigger() {
    let config = Config {
        triggers: vec![docgov::config::TriggerConfig {
            watch: "src/api/**".to_string(),
            require_update: "docs/reference/**".to_string(),
            message: None,
        }],
        ..Default::default()
    };

    let root_path = std::path::PathBuf::from("/test");

    // Case 1: src/api modified but docs/reference NOT modified -> Error
    let changed1 = vec![std::path::PathBuf::from("src/api/auth.rs")];
    let ctx1 = LintContext {
        workspace_root: &root_path,
        config: &config,
        root_files: &[],
        markdown_docs: &std::collections::HashMap::new(),
        changed_files: Some(&changed1),
    };
    let diags1 = GitSyncTriggerRule.check(&ctx1).unwrap();
    assert_eq!(diags1.len(), 1);
    assert_eq!(diags1[0].rule_id, "INV-LINT-04");

    // Case 2: src/api modified and docs/reference also modified -> Pass
    let changed2 = vec![
        std::path::PathBuf::from("src/api/auth.rs"),
        std::path::PathBuf::from("docs/reference/auth.md"),
    ];
    let ctx2 = LintContext {
        workspace_root: &root_path,
        config: &config,
        root_files: &[],
        markdown_docs: &std::collections::HashMap::new(),
        changed_files: Some(&changed2),
    };
    let diags2 = GitSyncTriggerRule.check(&ctx2).unwrap();
    assert_eq!(diags2.len(), 0);
}

#[test]
fn test_render_formats() {
    let diag = docgov::Diagnostic::error(
        "INV-LINT-02",
        "Public doc links into internal dev",
        "docs/tutorials/intro.md",
    )
    .with_location(14, 5)
    .with_snippet("[Dev](../dev/setup.md)")
    .with_suggestion("Move concept to public doc");

    let gh = diag.render_github();
    assert_eq!(
        gh,
        "::error file=docs/tutorials/intro.md,line=14,col=5,title=[INV-LINT-02]::Public doc links into internal dev"
    );

    let term = diag.render_terminal();
    assert!(term.contains("[INV-LINT-02]"));
    assert!(term.contains("docs/tutorials/intro.md:14:5"));
}

#[test]
fn test_lint_05_agent_directives_rule() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Missing target file -> error
    let engine = LintEngine::new(root).unwrap();
    let diagnostics = engine.run_lint(None).unwrap();
    let errs: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-05")
        .collect();
    assert_eq!(errs.len(), 1);
    assert!(errs[0]
        .message
        .contains("Missing agent directives configuration file"));

    // 2. Existing file without docgov marker block -> error
    fs::write(
        root.join("AGENTS.md"),
        "# Project Instructions\n\nCustom rules here.\n",
    )
    .unwrap();
    let diagnostics = engine.run_lint(None).unwrap();
    let errs: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-05")
        .collect();
    assert_eq!(errs.len(), 1);
    assert!(errs[0]
        .message
        .contains("Missing docgov anchor directives block"));

    // 3. File with marker block -> passes
    let (patched, action) = patch_agent_directives(
        Some(&fs::read_to_string(root.join("AGENTS.md")).unwrap()),
        SAMPLE_SNIPPET,
    );
    assert_eq!(action, PatchAction::Appended);
    fs::write(root.join("AGENTS.md"), patched).unwrap();

    let diagnostics = engine.run_lint(None).unwrap();
    let errs: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-05")
        .collect();
    assert_eq!(errs.len(), 0);
}

#[test]
fn test_patch_agent_directives_preserves_single_h1_and_existing_content() {
    let original =
        "# My Custom Coding Assistant Guidelines\n\n## Section 1\nSome developer instructions.\n";
    let (patched, action) = patch_agent_directives(Some(original), SAMPLE_SNIPPET);

    assert_eq!(action, PatchAction::Appended);
    // Preserves original content verbatim
    assert!(patched.starts_with(original.trim_end()));
    // Contains the markers
    assert!(patched.contains(DOCGOV_DIRECTIVES_BEGIN));
    assert!(patched.contains(DOCGOV_DIRECTIVES_END));

    // Crucial check: MUST NOT usurp or introduce another top-level '#' (H1) heading!
    let h1_lines: Vec<&str> = patched
        .lines()
        .filter(|l| l.starts_with("# ") && !l.starts_with("## "))
        .collect();
    assert_eq!(
        h1_lines.len(),
        1,
        "There must be exactly ONE top-level H1 heading in the document!"
    );
    assert_eq!(h1_lines[0], "# My Custom Coding Assistant Guidelines");

    // The inserted section must use secondary heading level (H2: '## Documentation Governance Directives')
    assert!(patched.contains("## Documentation Governance Directives"));

    // Test idempotency: running again on patched text returns Unchanged
    let (reこと, action2) = patch_agent_directives(Some(&patched), SAMPLE_SNIPPET);
    assert_eq!(action2, PatchAction::Unchanged);
    assert_eq!(reこと, patched);

    // Test update: modifying the interior of the marker block is cleanly updated
    let mutated = patched.replace("### 1. Machine Invariants", "### 1. Outdated Invariants");
    let (updated, action3) = patch_agent_directives(Some(&mutated), SAMPLE_SNIPPET);
    assert_eq!(action3, PatchAction::Updated);
    assert!(updated.contains("### 1. Machine Invariants"));
    assert!(!updated.contains("### 1. Outdated Invariants"));
    assert!(updated.starts_with(original.trim_end()));

    // Test creation from None: creates with H1 header
    let (created, action4) = patch_agent_directives(None, SAMPLE_SNIPPET);
    assert_eq!(action4, PatchAction::Created);
    assert!(created.starts_with("# Agent Directives\n\n"));
    assert!(created.contains("## Documentation Governance Directives"));
}

#[test]
fn test_lockfile_drift_detection() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (patched, _) = patch_agent_directives(None, SAMPLE_SNIPPET);
    fs::write(root.join("AGENTS.md"), &patched).unwrap();

    let mut lock = docgov::lockfile::DocgovLock::new(
        "0.0.1",
        "https://github.com/ming2k/docs-governance",
        "v0.0.1",
    );
    // Purposely set an old/invalid hash in lockfile
    lock.artifacts.agent_directives = Some(docgov::lockfile::ArtifactEntry {
        target: "AGENTS.md".to_string(),
        hash: "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_string(),
    });
    lock.save_to_dir(root).unwrap();

    let engine = LintEngine::new(root).unwrap();
    let diagnostics = engine.run_lint(None).unwrap();
    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule_id == "INV-LINT-05" && d.severity == docgov::Severity::Warning)
        .collect();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("drifted from .docgov.lock"));
}

#[test]
fn test_sync_governance_docs_atomic_prune() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // 1. Create a business doc outside the managed mirror
    let user_adr = root.join("docs/adr/0001-my-feature.md");
    fs::create_dir_all(user_adr.parent().unwrap()).unwrap();
    fs::write(&user_adr, "# Business ADR\n").unwrap();

    // 2. Create an obsolete ghost file inside the managed mirror
    let ghost_file = root.join("docs/governance/documentation/obsolete_ghost_rule.md");
    fs::create_dir_all(ghost_file.parent().unwrap()).unwrap();
    fs::write(
        &ghost_file,
        "This is an obsolete rule from an older version\n",
    )
    .unwrap();

    assert!(ghost_file.exists());

    // 3. Run sync_governance_docs from local repository spec
    let client = docgov::remote::RemoteClient::new(".", "local");
    client
        .sync_governance_docs(root, "docs/governance/documentation", true)
        .unwrap();

    // 4. Verify canonical files were unpacked
    assert!(root
        .join("docs/governance/documentation/core/invariants.md")
        .exists());
    assert!(root
        .join("docs/governance/documentation/core/taxonomy.md")
        .exists());

    // 5. Verify the obsolete ghost file was completely pruned!
    assert!(
        !ghost_file.exists(),
        "Obsolete file must be pruned during full atomic replacement!"
    );

    // 6. Verify user's business doc was completely untouched!
    assert!(user_adr.exists());
    assert_eq!(fs::read_to_string(&user_adr).unwrap(), "# Business ADR\n");
}

#[test]
fn test_remote_client_fails_deterministically_without_embedded_fallback() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Point to a non-existent remote repo and non-existent version
    let client = docgov::remote::RemoteClient::new(
        "https://github.com/nonexistent-org-99999/nonexistent-repo-99999",
        "v99.99.99",
    );

    // Fetching directives must fail explicitly, never silently falling back to embedded constants!
    let directives_res = client.fetch_directives(None);
    assert!(
        directives_res.is_err(),
        "Must fail when remote endpoint is not found, never silently fallback!"
    );
    let err_msg = directives_res.err().unwrap().to_string();
    assert!(
        err_msg.contains("Failed to fetch directives snippet for upstream"),
        "Error message must be clear: {err_msg}"
    );

    // Syncing governance docs must fail explicitly without embedded tar fallback!
    let sync_res = client.sync_governance_docs(root, "docs/governance/documentation", true);
    assert!(
        sync_res.is_err(),
        "Must fail when remote release archive is not found, never silently fallback!"
    );
    let err_msg = sync_res.err().unwrap().to_string();
    assert!(
        err_msg.contains("Failed to download governance documentation assets"),
        "Error message must be clear: {err_msg}"
    );
}

#[test]
fn test_local_spec_sync_deterministic_hash_and_directives() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Setup local spec in workspace
    let spec_dir = root.join("spec");
    fs::create_dir_all(spec_dir.join("core")).unwrap();
    let snippet_content = "<!-- BEGIN DOCGOV DIRECTIVES -->\n## Custom Local Directives\n<!-- END DOCGOV DIRECTIVES -->";
    fs::write(spec_dir.join("directives.snippet"), snippet_content).unwrap();
    fs::write(spec_dir.join("core/invariants.md"), "# Local Invariants\n").unwrap();

    let client = docgov::remote::RemoteClient::new(".", "local");
    let (fetched_snippet, source_info) = client.fetch_directives(Some(root)).unwrap();
    assert_eq!(fetched_snippet, snippet_content);
    assert!(source_info.starts_with("local:"));

    let hash = client
        .sync_governance_docs(root, "docs/governance/documentation", true)
        .unwrap();
    assert!(hash.starts_with("sha256:"));

    // Verify documentation was created from local spec
    assert!(root
        .join("docs/governance/documentation/core/invariants.md")
        .exists());
    assert_eq!(
        fs::read_to_string(root.join("docs/governance/documentation/core/invariants.md")).unwrap(),
        "# Local Invariants\n"
    );
    // Directives snippet must not leak into documentation mirror
    assert!(!root
        .join("docs/governance/documentation/directives.snippet")
        .exists());
}

#[test]
fn test_xdg_cache_dir_resolution() {
    let client =
        docgov::remote::RemoteClient::new("https://github.com/ming2k/docs-governance", "v0.0.4");

    // Case 1: When XDG_CACHE_HOME is explicitly set, it must be strictly prioritized
    let custom_cache = tempdir().unwrap();
    std::env::set_var("XDG_CACHE_HOME", custom_cache.path());
    let cache_dir = client.get_cache_dir();
    assert!(
        cache_dir.starts_with(custom_cache.path()),
        "Cache dir must reside within XDG_CACHE_HOME when set! Got: {}",
        cache_dir.display()
    );
    assert!(cache_dir.to_string_lossy().contains("docgov"));
    assert!(cache_dir.to_string_lossy().contains("v0.0.4"));

    // Case 2: Clean up env var
    std::env::remove_var("XDG_CACHE_HOME");
    let fallback_dir = client.get_cache_dir();
    assert!(
        fallback_dir.to_string_lossy().contains("docgov"),
        "Fallback cache dir must still be namespaced under docgov"
    );
}
