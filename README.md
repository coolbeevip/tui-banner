# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

[![Crates.io](https://img.shields.io/crates/v/tui-banner.svg)](https://crates.io/crates/tui-banner)
[![Docs.rs](https://docs.rs/tui-banner/badge.svg)](https://docs.rs/tui-banner/)
[![License](https://img.shields.io/crates/l/tui-banner.svg)](https://crates.io/crates/tui-banner)

![banner example](assets/images/gradient_diagonal.png)

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
tui-banner = "0.1.4"
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

## Custom Gradient Example

```rust
use tui_banner::{Align, Banner, ColorMode, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    let banner = Banner::new("RUST CLI")?
        .color_mode(ColorMode::TrueColor)
        .gradient(Gradient::vertical(Palette::from_hex(&[
            "#00E5FF", // cyan
            "#3A7BFF", // blue
            "#E6F6FF", // ice
        ])))
        .fill(Fill::Keep)
        .dither()
        .targets("░▒▓")
        .checker(3)
        .align(Align::Center)
        .padding(1)
        .render();

    println!("{banner}");
    Ok(())
}
```

Run it with:

```bash
cargo run --example gradient_custom
```

## Gradient Gallery

| Vertical | Horizontal | Diagonal | Apply |
| --- | --- | --- | --- |
| ![Gradient Vertical](assets/images/gradient_vertical.png) | ![Gradient Horizontal](assets/images/gradient_horizontal.png) | ![Gradient Diagonal](assets/images/gradient_diagonal.png) | ![Gradient Apply](assets/images/gradient_apply.png) |

## Full Gallery

| Block | Half Block | Light Shade | Char |
| --- | --- | --- | --- |
| ![Full Block](assets/images/full_block.png) | ![Full Half Block](assets/images/full_half_block.png) | ![Full Light Shade](assets/images/full_light_shade.png) | ![Full Char](assets/images/full_char.png) |

## Dither Gallery

| Checker Stipple | Coarse Halftone | Film Grain | Sparkle Noise |
| --- | --- | --- | --- |
| ![Dither Checker Stipple](assets/images/dither_checker_stipple.png) | ![Dither Coarse Halftone](assets/images/dither_coarse_halftone.png) | ![Dither Film Grain](assets/images/dither_film_grain.png) | ![Dither Sparkle Noise](assets/images/dither_sparkle_noise.png) |

## Examples

```bash
cargo run --example gradient_neon_cyber
cargo run --example gradient_arctic_tech
cargo run --example gradient_aurora_flux
cargo run --example gradient_deep_space
cargo run --example gradient_ocean_flow
cargo run --example gradient_sunset_neon
cargo run --example gradient_fire_warning
cargo run --example gradient_warm_luxury
cargo run --example gradient_forest_sky
cargo run --example gradient_earth_tone
cargo run --example gradient_chrome
cargo run --example gradient_royal_purple
cargo run --example gradient_crt_amber
cargo run --example gradient_matrix
cargo run --example dither_checker_stipple
cargo run --example dither_coarse_halftone
cargo run --example dither_film_grain
cargo run --example dither_sparkle_noise
```
