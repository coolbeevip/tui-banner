![Neon Cyber](assets/images/neon_gradient_neon_cyber.png)
# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.
Website: https://tui-banner-website.pages.dev/

[![Crates.io](https://img.shields.io/crates/v/tui-banner.svg)](https://crates.io/crates/tui-banner)
[![Docs.rs](https://docs.rs/tui-banner/badge.svg)](https://docs.rs/tui-banner/)
[![License](https://img.shields.io/crates/l/tui-banner.svg)](https://crates.io/crates/tui-banner)

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading
- Named style and palette presets
- Fluent builder API

## Quick Start

```toml
[dependencies]
tui-banner = { path = "." }
```

```rust
use tui_banner::{Align, Banner, Style};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")?
        .style(Style::NeonCyber)
        .render();

    println!("{banner}");
    Ok(())
}
```

## Examples

```bash
cargo run --example neon_gradient_neon_cyber
cargo run --example neon_gradient_arctic_tech
cargo run --example neon_gradient_aurora_flux
cargo run --example neon_gradient_deep_space
cargo run --example neon_gradient_ocean_flow
cargo run --example neon_gradient_sunset_neon
cargo run --example neon_gradient_fire_warning
cargo run --example neon_gradient_warm_luxury
cargo run --example neon_gradient_forest_sky
cargo run --example neon_gradient_earth_tone
cargo run --example neon_gradient_chrome
cargo run --example neon_gradient_royal_purple
cargo run --example neon_gradient_crt_amber
cargo run --example neon_gradient_matrix
```
