pub struct Settings {
    pub api_key: String,
    pub server_port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api_key: "default_key_123".to_string(),
            server_port: 8080,
        }
    }
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }
}