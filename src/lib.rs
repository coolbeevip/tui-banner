#![deny(missing_docs)]
//! Colorful ASCII art banner rendering for Rust CLI/TUI.
//!
//! ## Quick Start
//! ```rust
//! use tui_banner::{Align, Banner, Fill, Font, Gradient, Palette};
//!
//! let banner = Banner::new("RUST CLI")
//!     .font(Font::dos_rebel())
//!     .gradient(Gradient::vertical(Palette::from_hex(&[
//!         "#00E5FF",
//!         "#7B5CFF",
//!         "#FF5AD9",
//!     ])))
//!     .fill(Fill::Keep)
//!     .dither()
//!     .targets("░▒▓")
//!     .dots("·:")
//!     .checker(3)
//!     .align(Align::Center)
//!     .padding(1)
//!     .render();
//!
//! let _ = banner;
//! ```

/// High-level banner builder API.
pub mod banner;
/// Color types and palettes.
pub mod color;
/// Visual effects (dither, outline, shadow).
pub mod effects;
/// ANSI output emitter.
pub mod emit;
/// Fill and dither configuration.
pub mod fill;
/// Fonts and glyph rendering.
pub mod font;
/// Gradient definitions.
pub mod gradient;
/// Grid and layout types.
pub mod grid;
/// Terminal capability detection.
pub mod terminal;

pub use banner::Banner;
pub use color::{Color, ColorMode, Palette};
pub use effects::outline::EdgeShade;
pub use fill::{Dither, DitherMode, Fill};
pub use font::{Font, figlet::FigletError};
pub use gradient::{Gradient, GradientDirection};
pub use grid::{Align, Padding};
