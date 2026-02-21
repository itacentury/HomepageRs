use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// A pinned GitHub repository with metadata for display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedRepo {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub stargazer_count: u32,
    pub primary_language: Option<Language>,
    pub updated_at: String,
}

/// A programming language with its GitHub color.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub color: Option<String>,
}

// ── GraphQL response deserialization structs ──

#[derive(Deserialize)]
struct GqlResponse {
    data: Option<GqlData>,
}

#[derive(Deserialize)]
struct GqlData {
    user: GqlUser,
}

#[derive(Deserialize)]
struct GqlUser {
    #[serde(rename = "pinnedItems")]
    pinned_items: GqlPinnedItems,
}

#[derive(Deserialize)]
struct GqlPinnedItems {
    nodes: Vec<GqlRepo>,
}

#[derive(Deserialize)]
struct GqlRepo {
    name: String,
    description: Option<String>,
    url: String,
    #[serde(rename = "stargazerCount")]
    stargazer_count: u32,
    #[serde(rename = "primaryLanguage")]
    primary_language: Option<GqlLanguage>,
    #[serde(rename = "updatedAt")]
    updated_at: String,
}

#[derive(Deserialize)]
struct GqlLanguage {
    name: String,
    color: Option<String>,
}

impl From<GqlRepo> for PinnedRepo {
    fn from(r: GqlRepo) -> Self {
        Self {
            name: r.name,
            description: r.description,
            url: r.url,
            stargazer_count: r.stargazer_count,
            primary_language: r.primary_language.map(|l| Language {
                name: l.name,
                color: l.color,
            }),
            updated_at: r.updated_at,
        }
    }
}

// ── Cache ──

struct CacheEntry {
    repos: Vec<PinnedRepo>,
    fetched_at: Instant,
}

/// Thread-safe cache for GitHub pinned repositories.
///
/// Fetches from GitHub's GraphQL API and caches results for a configurable TTL.
/// Falls back to stale cache on errors and returns `None` when no token is set.
pub struct RepoCache {
    token: Option<String>,
    username: String,
    ttl: Duration,
    cache: Mutex<Option<CacheEntry>>,
}

impl RepoCache {
    /// Create a new cache with an optional GitHub token, username, and cache TTL.
    pub fn new(token: Option<String>, username: String, ttl: Duration) -> Self {
        Self {
            token,
            username,
            ttl,
            cache: Mutex::new(None),
        }
    }

    /// Return cached repos if fresh, fetch if stale, or `None` if unavailable.
    pub async fn get_repos(&self) -> Option<Vec<PinnedRepo>> {
        let token = self.token.as_ref()?;

        let mut cache = self.cache.lock().await;

        if let Some(entry) = cache.as_ref()
            && entry.fetched_at.elapsed() < self.ttl
        {
            return Some(entry.repos.clone());
        }

        match fetch_pinned_repos(token, &self.username).await {
            Ok(repos) => {
                *cache = Some(CacheEntry {
                    repos: repos.clone(),
                    fetched_at: Instant::now(),
                });
                Some(repos)
            }
            Err(e) => {
                eprintln!("GitHub API error: failed to fetch repositories");
                eprintln!("  Details: {e}");
                cache.as_ref().map(|entry| entry.repos.clone())
            }
        }
    }
}

/// Fetch pinned repositories from GitHub's GraphQL API.
async fn fetch_pinned_repos(
    token: &str,
    username: &str,
) -> Result<Vec<PinnedRepo>, reqwest::Error> {
    let query = format!(
        r#"query {{
  user(login: "{username}") {{
    pinnedItems(first: 6, types: REPOSITORY) {{
      nodes {{
        ... on Repository {{
          name
          description
          url
          stargazerCount
          primaryLanguage {{ name color }}
          updatedAt
        }}
      }}
    }}
  }}
}}"#
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("failed to build HTTP client");

    let resp = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("Bearer {token}"))
        .header("User-Agent", "HomepageRs")
        .json(&serde_json::json!({ "query": query }))
        .send()
        .await?
        .error_for_status()?;

    let gql: GqlResponse = resp.json().await?;
    let repos = gql
        .data
        .map(|d| {
            d.user
                .pinned_items
                .nodes
                .into_iter()
                .map(Into::into)
                .collect()
        })
        .unwrap_or_default();

    Ok(repos)
}
