# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

![banner](assets/banner.png)

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading
- Fluent builder API

## Quick Start

```toml
[dependencies]
tui-banner = { path = "." }
```

```rust
use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")?
        .gradient(Gradient::vertical(Palette::from_hex(&[
            "#00E5FF",
            "#7B5CFF",
            "#FF5AD9",
        ])))
        .fill(Fill::Keep)
        .dither()
        .targets("░▒▓")
        .dots("·:")
        .checker(3)
        .align(Align::Center)
        .padding(1)
        .render();

    println!("{banner}");
    Ok(())
}
```

## Examples

```bash
cargo run --example neon_gradient_neon_cyber
cargo run --example neon_gradient_arctic_tech
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
