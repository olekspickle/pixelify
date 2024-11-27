//! ![input](examples/in.png) | ![output](examples/out.png)
//!
//! # Overview
//! This library is for easy image pixelation
//! The use is simple:
//!
//! ```bash
//! pixelify <image> [-o OUTPUT]
//! cargo run --bin pixelify pine-forest.png -s 20
//! ```
//!
//! # Features
//! - [x] [box-blur](https://en.wikipedia.org/wiki/Box_blur)
//! - [ ] randomize the resampling to get more interesting results
//! - [ ] [Lanczos_resampling](https://en.wikipedia.org/wiki/Lanczos_resampling)

pub mod algorhytm;
pub mod cli;

pub use algorhytm::BoxBlur;
