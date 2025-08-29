pub mod config;
pub mod models;
pub mod services;
pub mod errors;
pub mod repositories;
pub mod handlers;

pub use errors::AppError;
pub type Result<T> = std::result::Result<T, AppError>;