use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server {
    pub id: Option<String>,
    pub name: String,
    pub hostname: String,
    pub ip_address: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

impl Server {
    
    pub fn new(
        name: impl Into<String>,
        hostname: impl Into<String>,
        ip_address: impl Into<String>,
    ) -> crate::Result<Self> {
        let name = name.into();
        let hostname = hostname.into();
        let ip_address = ip_address.into();
        
        if name.trim().is_empty() {
            return Err(crate::AppError::Validation("Server name cannot be empty".into()));
        }
        
        if hostname.trim().is_empty() {
            return Err(crate::AppError::Validation("Hostname cannot be empty".into()));
        }
        
        if ip_address.trim().is_empty() {
            return Err(crate::AppError::Validation("IP address cannot be empty".into()));
        }
        
        Ok(Self {
            id: None,
            name,
            hostname,
            ip_address,
            is_active: true,
            created_at: Utc::now(),
            last_seen: None,
        })
    }
    
        pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    
    
    pub fn activate(&mut self) {
        self.is_active = true;
        self.last_seen = Some(Utc::now());
    }
    
    
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
    
    
    pub fn update_last_seen(&mut self) {
        self.last_seen = Some(Utc::now());
    }
    
    
    pub fn is_online(&self) -> bool {
        if let Some(last_seen) = self.last_seen {
            let now = Utc::now();
            let diff = now.signed_duration_since(last_seen);
            diff.num_minutes() < 5
        } else {
            false
        }
    }
}