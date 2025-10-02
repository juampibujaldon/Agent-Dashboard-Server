use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub api_key: String,
    pub backend_base_url: String,
    pub server_id: String,
    pub interval_secs: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: "not-needed".into(),
            backend_base_url: "http://localhost:5000/api".into(),
            server_id: whoami::fallible::hostname().unwrap_or_else(|_| "server-01".into()),
            interval_secs: 30,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let mut settings = Self::default();

        if let Ok(v) = env::var("AGENT_API_KEY") {
            settings.api_key = v;
        }
        if let Ok(v) = env::var("AGENT_BACKEND_URL") {
            settings.backend_base_url = v;
        }
        if let Ok(v) = env::var("AGENT_SERVER_ID") {
            settings.server_id = v;
        }
        if let Ok(v) = env::var("AGENT_INTERVAL_SECS") {
            if let Ok(parsed) = v.parse::<u64>() {
                settings.interval_secs = parsed.max(5);
            }
        }

        settings
    }
}
