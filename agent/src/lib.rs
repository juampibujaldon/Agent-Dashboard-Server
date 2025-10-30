pub mod config;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub mod traits;

pub use errors::AppError;
pub type Result<T> = std::result::Result<T, AppError>;
