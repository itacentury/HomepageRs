use std::path::Path;

#[test]
fn test_load_config_from_path() {
    let config = site::config::load_config_from_path(Path::new("tests/fixtures/test_config.yaml"))
        .expect("should load test config");

    assert_eq!(config.personal.name, "Test User");
    assert_eq!(config.personal.title, "Test Title");
    assert_eq!(config.github.username, "testuser");
    assert_eq!(config.github.cache_ttl_secs, 60);
    assert!(config.github.token.is_none());
    assert_eq!(config.education.len(), 1);
    assert_eq!(config.experience.len(), 1);
    assert_eq!(config.links.len(), 1);
}

#[test]
fn test_config_missing_file() {
    let result = site::config::load_config_from_path(Path::new("nonexistent.yaml"));
    assert!(result.is_err());
}
