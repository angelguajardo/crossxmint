// src/entities/mod.rs

pub mod polyanet;
pub mod soloon;
pub mod cometh;

// Re-exporting structs and enums for easier access in main.rs
pub use polyanet::Polyanet;
pub use soloon::{Soloon, Color};
pub use cometh::{Cometh, Direction};
