use serde::Deserialize;
use std::path::Path;

/// Top-level application configuration loaded from YAML.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub personal: PersonalConfig,
    pub github: GithubConfig,
    pub education: Vec<Education>,
    pub experience: Vec<Experience>,
    pub links: Vec<Link>,
}

/// Personal information displayed in the hero section and meta tags.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct PersonalConfig {
    pub name: String,
    pub title: String,
    pub meta_description: String,
    pub og_title: String,
    pub og_description: String,
}

/// GitHub integration settings.
#[derive(Debug, Clone, Deserialize)]
pub struct GithubConfig {
    pub username: String,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default = "default_cache_ttl")]
    pub cache_ttl_secs: u64,
}

fn default_cache_ttl() -> u64 {
    300
}

/// An entry in the education timeline.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct Education {
    pub r#type: String,
    pub year: String,
    pub name: String,
    pub place: String,
}

/// An entry in the work experience timeline.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct Experience {
    pub r#type: String,
    pub year: String,
    pub name: String,
    pub place: String,
}

/// A social or contact link.
#[derive(Debug, Clone, Deserialize, serde::Serialize)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub linkname: String,
}

/// Load configuration from a specific file path.
pub fn load_config_from_path(path: &Path) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let mut config: AppConfig = serde_yaml::from_str(&contents)?;
    apply_env_overrides(&mut config);
    Ok(config)
}

/// Load configuration from `CONFIG_PATH` env var (default: `config.yaml`).
pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    load_config_from_path(Path::new(&path))
}

/// Override YAML values with environment variables when set.
fn apply_env_overrides(config: &mut AppConfig) {
    if let Ok(token) = std::env::var("GITHUB_TOKEN")
        && !token.is_empty()
    {
        config.github.token = Some(token);
    }
    if let Ok(username) = std::env::var("GITHUB_USERNAME")
        && !username.is_empty()
    {
        config.github.username = username;
    }
    if let Ok(ttl) = std::env::var("CACHE_TTL_SECS")
        && let Ok(secs) = ttl.parse::<u64>()
    {
        config.github.cache_ttl_secs = secs;
    }
}
