use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

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

    /// Fetch directives snippet with cache-first strategy and embedded fallback
    pub fn fetch_directives(&self, fallback: &str) -> Result<(String, String)> {
        let cache_dir = self.get_cache_dir();
        let cache_file = cache_dir.join("directives.snippet");

        // 1. Cache hit check: if cached locally, use immediately without network call
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

        // 2. Network fetch attempt
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

        // 3. Fallback to embedded baseline if offline or remote not available yet
        println!(
            "{} Remote not reachable or release asset pending; using built-in baseline directives",
            "ℹ".cyan().bold()
        );
        Ok((fallback.to_string(), "embedded".to_string()))
    }

    fn candidate_download_urls(&self) -> Vec<String> {
        let (owner, repo) = self.parse_owner_repo();
        vec![
            // 1. GitHub Release asset (primary distribution point uploaded by CI)
            format!(
                "https://github.com/{}/{}/releases/download/{}/directives.snippet",
                owner, repo, self.r#ref
            ),
            // 2. Raw GitHub content (fallback)
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
}
