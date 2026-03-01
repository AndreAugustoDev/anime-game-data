use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;
use serde::de::DeserializeOwned;

use super::GameDataSource;

const COMMITS_API_URL: &str = "https://gitlab.com/api/v4/projects/53216109/repository/commits";
const REPO_BASE_URL: &str = "https://gitlab.com/Dimbreath/AnimeGameData/-/raw";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct GitLabCommitEntry {
    id: String,
    short_id: String,
    created_at: String,
    parent_ids: Vec<String>,
    title: String,
    message: String,
    author_name: Option<String>,
    author_email: Option<String>,
    author_date: Option<String>,
    committer_name: Option<String>,
    committer_email: Option<String>,
    committed_date: Option<String>,
    web_url: String,
}

pub struct Dimbreath {
    client: reqwest::Client,
}

impl Dimbreath {
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: reqwest::Client::builder()
                .gzip(true)
                .timeout(Duration::from_secs(30))
                .build()?,
        })
    }
}

impl GameDataSource for Dimbreath {
    async fn get_latest_hash(&self) -> Result<String> {
        let commits = self
            .client
            .get(COMMITS_API_URL)
            .query(&[("per_page", "1")])
            .send()
            .await
            .context("Failed to fetch commits")?
            .json::<Vec<GitLabCommitEntry>>()
            .await
            .context("Failed to parse commits")?;

        commits
            .first()
            .map(|c| c.id.clone())
            .context("No commits found")
    }

    async fn get_json_file<T: DeserializeOwned>(&self, git_ref: &str, path: &str) -> Result<T> {
        let url = format!("{REPO_BASE_URL}/{git_ref}/{path}");
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Failed to send request for {}", url))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Request to {} failed with status: {}",
                url,
                response.status()
            ));
        }

        response
            .json::<T>()
            .await
            .with_context(|| format!("Failed to parse {}", url))
    }
}
