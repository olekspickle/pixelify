//! //! ![axum-template]()
//!
//! # Overview
//! This library is for easy image pixelation
//! The use is simple:
//!
//! ```bash
//! pixelify <image> [-o OUTPUT]
//! ```

pub mod algorhytm;
pub mod cli;

pub use algorhytm::BoxBlur;
