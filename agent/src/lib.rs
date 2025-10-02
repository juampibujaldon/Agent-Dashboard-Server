pub mod config;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;

pub use errors::AppError;
pub type Result<T> = std::result::Result<T, AppError>;
