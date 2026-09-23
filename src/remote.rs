use anyhow::{Context, Result};
use colored::Colorize;
use flate2::read::GzDecoder;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;

pub struct RemoteClient {
    pub source: String,
    pub r#ref: String,
}

impl RemoteClient {
    pub fn new(source: &str, r#ref: &str) -> Self {
        Self {
            source: source.trim_end_matches('/').to_string(),
            r#ref: r#ref.to_string(),
        }
    }

    /// Check whether source specifies a local filesystem path
    pub fn local_spec_dir(&self, workspace_root: &Path) -> Option<PathBuf> {
        let path_str = self.source.strip_prefix("file://").unwrap_or(&self.source);
        if path_str == "." || path_str == "local" || path_str == "self" {
            let ws_spec = workspace_root.join("spec");
            if ws_spec.is_dir() {
                return Some(ws_spec);
            }
            if let Ok(cur) = std::env::current_dir() {
                let cur_spec = cur.join("spec");
                if cur_spec.is_dir() {
                    return Some(cur_spec);
                }
            }
            if workspace_root.is_dir() {
                return Some(workspace_root.to_path_buf());
            }
        }
        let direct_path = Path::new(path_str);
        if direct_path.is_dir() {
            if direct_path.join("spec").is_dir() {
                return Some(direct_path.join("spec"));
            }
            return Some(direct_path.to_path_buf());
        }
        let ws_relative = workspace_root.join(path_str);
        if ws_relative.is_dir() {
            if ws_relative.join("spec").is_dir() {
                return Some(ws_relative.join("spec"));
            }
            return Some(ws_relative);
        }
        None
    }

    /// Determine local cache directory for this specific upstream and ref
    pub fn get_cache_dir(&self) -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let cache_base = PathBuf::from(home).join(".cache").join("docgov");

        // Sanitize source identifier to make safe path
        let sanitized_source = self
            .source
            .replace("https://", "")
            .replace("http://", "")
            .replace("github.com/", "")
            .replace("github:", "")
            .replace(['/', ':', '@'], "_");

        cache_base.join(sanitized_source).join(&self.r#ref)
    }

    /// Fetch directives snippet with local-spec, cache-first, and remote-network strategy.
    /// Fails deterministically if the asset is not reachable.
    pub fn fetch_directives(&self, workspace_root: Option<&Path>) -> Result<(String, String)> {
        // 1. Local workspace specification source check
        if let Some(ws) = workspace_root {
            if let Some(spec_dir) = self.local_spec_dir(ws) {
                let candidate_file = if spec_dir.join("directives.snippet").is_file() {
                    spec_dir.join("directives.snippet")
                } else if spec_dir.join("spec/directives.snippet").is_file() {
                    spec_dir.join("spec/directives.snippet")
                } else {
                    anyhow::bail!(
                        "Local governance directory '{}' does not contain 'directives.snippet'.",
                        spec_dir.display()
                    );
                };
                let content = fs::read_to_string(&candidate_file).with_context(|| {
                    format!("Failed to read local snippet at {}", candidate_file.display())
                })?;
                println!(
                    "{} Using local specification directives ({})",
                    "✔".green().bold(),
                    candidate_file.display()
                );
                return Ok((content, format!("local:{}", candidate_file.display())));
            }
        }

        let cache_dir = self.get_cache_dir();
        let cache_file = cache_dir.join("directives.snippet");

        // 2. Cache hit check: if cached locally, use immediately without network call
        if cache_file.exists() {
            if let Ok(content) = fs::read_to_string(&cache_file) {
                if !content.trim().is_empty() {
                    println!(
                        "{} Cached remote assets hit ({})",
                        "✔".green().bold(),
                        cache_file.display()
                    );
                    return Ok((content, "cache".to_string()));
                }
            }
        }

        // 3. Network fetch attempt
        let download_urls = self.candidate_download_urls();
        for url in &download_urls {
            match self.download_url(url) {
                Ok(content) if !content.trim().is_empty() => {
                    // Save to local cache for future runs
                    let _ = fs::create_dir_all(&cache_dir);
                    let _ = fs::write(&cache_file, &content);
                    println!(
                        "{} Fetched remote directives from {}",
                        "✔".green().bold(),
                        url
                    );
                    return Ok((content, url.clone()));
                }
                _ => continue,
            }
        }

        // 4. Deterministic failure when unresolvable
        anyhow::bail!(
            "Failed to fetch directives snippet for upstream '{}' (ref: '{}').\n\
             Endpoints attempted:\n\
             - {}\n\
             - {}\n\
             Asset not found in local cache or remote endpoints. Please check network connectivity or upstream release status.",
            self.source,
            self.r#ref,
            download_urls[0],
            download_urls[1]
        );
    }

    /// Atomically sync and replace the canonical governance documentation mirror
    /// completely pruning old/deleted files from prior versions.
    pub fn sync_governance_docs(
        &self,
        workspace_root: &Path,
        target_rel_path: &str,
        force: bool,
    ) -> Result<String> {
        let target_dir = workspace_root.join(target_rel_path);

        // 1. Local spec source: sync directly from local directory
        if let Some(spec_dir) = self.local_spec_dir(workspace_root) {
            return self.sync_from_local_dir(&spec_dir, &target_dir, workspace_root);
        }

        // 2. Remote spec source: fetch release archive or read cache
        let cache_dir = self.get_cache_dir();
        let cache_tar = cache_dir.join("docgov-assets.tar.gz");

        let tar_bytes: Vec<u8> = if !force && cache_tar.exists() {
            println!(
                "{} Using cached governance documentation archive ({})",
                "✔".green().bold(),
                cache_tar.display()
            );
            fs::read(&cache_tar)?
        } else {
            let (owner, repo) = self.parse_owner_repo();
            let download_url = format!(
                "https://github.com/{}/{}/releases/download/{}/docgov-assets.tar.gz",
                owner, repo, self.r#ref
            );
            match self.download_bytes(&download_url) {
                Ok(bytes) => {
                    let _ = fs::create_dir_all(&cache_dir);
                    let _ = fs::write(&cache_tar, &bytes);
                    println!(
                        "{} Downloaded governance documentation assets from {}",
                        "✔".green().bold(),
                        download_url
                    );
                    bytes
                }
                Err(err) => {
                    anyhow::bail!(
                        "Failed to download governance documentation assets for upstream '{}/{}' (ref: '{}').\n\
                         URL: {}\n\
                         Error: {}\n\
                         Release asset 'docgov-assets.tar.gz' is unreachable. Ensure the release exists and assets are published.",
                        owner,
                        repo,
                        self.r#ref,
                        download_url,
                        err
                    );
                }
            }
        };

        // Compute SHA-256 fingerprint of the tarball
        let archive_hash = crate::lockfile::compute_sha256_bytes(&tar_bytes);

        // Temporary staging directory for atomic unpack
        let staging_dir = target_dir
            .parent()
            .unwrap_or(workspace_root)
            .join(format!(".tmp_sync_{}", std::process::id()));
        if staging_dir.exists() {
            let _ = fs::remove_dir_all(&staging_dir);
        }
        fs::create_dir_all(&staging_dir)?;

        // Unpack tar.gz into staging_dir, stripping leading "spec/"
        let gz = GzDecoder::new(&tar_bytes[..]);
        let mut archive = Archive::new(gz);

        for entry_res in archive.entries()? {
            let mut entry = entry_res?;
            let path = entry.path()?;
            let path_str = path.to_string_lossy();

            let rel_path = if let Some(stripped) = path_str.strip_prefix("spec/") {
                stripped.to_string()
            } else if path_str == "spec" {
                continue;
            } else {
                path_str.to_string()
            };

            if rel_path.is_empty() || rel_path == "directives.snippet" {
                continue;
            }

            let dest_file = staging_dir.join(&rel_path);
            if entry.header().entry_type().is_dir() {
                fs::create_dir_all(&dest_file)?;
            } else {
                if let Some(parent) = dest_file.parent() {
                    fs::create_dir_all(parent)?;
                }
                entry.unpack(&dest_file)?;
            }
        }

        // Full atomic mirror replacement: prune old target completely to eliminate ghost files
        if target_dir.exists() {
            fs::remove_dir_all(&target_dir)?;
        }
        if let Some(parent) = target_dir.parent() {
            fs::create_dir_all(parent)?;
        }

        if fs::rename(&staging_dir, &target_dir).is_err() {
            copy_dir_all(&staging_dir, &target_dir, true)?;
            let _ = fs::remove_dir_all(&staging_dir);
        }

        println!(
            "{} Atomically synchronized and pruned canonical governance documentation into {}",
            "✔".green().bold(),
            target_dir.display()
        );

        Ok(archive_hash)
    }

    fn sync_from_local_dir(
        &self,
        spec_dir: &Path,
        target_dir: &Path,
        workspace_root: &Path,
    ) -> Result<String> {
        let staging_dir = target_dir
            .parent()
            .unwrap_or(workspace_root)
            .join(format!(".tmp_sync_{}", std::process::id()));
        if staging_dir.exists() {
            let _ = fs::remove_dir_all(&staging_dir);
        }
        fs::create_dir_all(&staging_dir)?;

        copy_dir_all(spec_dir, &staging_dir, false)?;

        if target_dir.exists() {
            fs::remove_dir_all(target_dir)?;
        }
        if let Some(parent) = target_dir.parent() {
            fs::create_dir_all(parent)?;
        }

        if fs::rename(&staging_dir, target_dir).is_err() {
            copy_dir_all(&staging_dir, target_dir, true)?;
            let _ = fs::remove_dir_all(&staging_dir);
        }

        let hash = compute_dir_sha256(target_dir)?;
        println!(
            "{} Atomically synchronized local governance specification into {}",
            "✔".green().bold(),
            target_dir.display()
        );

        Ok(hash)
    }

    fn candidate_download_urls(&self) -> Vec<String> {
        let (owner, repo) = self.parse_owner_repo();
        vec![
            // 1. GitHub Release asset (primary distribution point uploaded by CI)
            format!(
                "https://github.com/{}/{}/releases/download/{}/directives.snippet",
                owner, repo, self.r#ref
            ),
            // 2. Raw GitHub content (fallback for git refs before release packaging)
            format!(
                "https://raw.githubusercontent.com/{}/{}/{}/spec/directives.snippet",
                owner, repo, self.r#ref
            ),
        ]
    }

    fn parse_owner_repo(&self) -> (String, String) {
        let stripped = self
            .source
            .replace("https://github.com/", "")
            .replace("http://github.com/", "")
            .replace("github:", "");
        let parts: Vec<&str> = stripped.split('/').collect();
        if parts.len() >= 2 {
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ("ming2k".to_string(), "docs-governance".to_string())
        }
    }

    fn download_url(&self, url: &str) -> Result<String> {
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(5))
            .call()?;
        let body = response.into_string()?;
        Ok(body)
    }

    fn download_bytes(&self, url: &str) -> Result<Vec<u8>> {
        let response = ureq::get(url)
            .timeout(std::time::Duration::from_secs(10))
            .call()?;
        let mut reader = response.into_reader();
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        Ok(bytes)
    }
}

fn copy_dir_all(src: &Path, dst: &Path, include_directives: bool) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let from = entry.path();
        let file_name = entry.file_name();
        if !include_directives && file_name == "directives.snippet" {
            continue;
        }
        let to = dst.join(file_name);
        if ty.is_dir() {
            copy_dir_all(&from, &to, include_directives)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn compute_dir_sha256(dir: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    let mut entries = Vec::new();
    for entry in walkdir::WalkDir::new(dir).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_file() {
            entries.push(entry.into_path());
        }
    }
    entries.sort();
    for path in entries {
        let rel = path.strip_prefix(dir).unwrap_or(&path);
        hasher.update(rel.to_string_lossy().as_bytes());
        let content = fs::read(&path)?;
        hasher.update(&content);
    }
    Ok(format!("sha256:{}", hex::encode(hasher.finalize())))
}
