/// App configuration loaded from `assets/config.toml` at startup.
#[derive(Debug, serde::Deserialize)]
pub struct AppConfig {
    pub featured_fandom: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string("assets/config.toml")?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}