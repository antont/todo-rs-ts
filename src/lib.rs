#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("features 'postgres' and 'sqlite' are mutually exclusive");

pub mod error;
pub mod handlers;
pub mod models;
