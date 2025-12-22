# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

[![Crates.io](https://img.shields.io/crates/v/tui-banner.svg)](https://crates.io/crates/tui-banner)
[![Docs.rs](https://docs.rs/tui-banner/badge.svg)](https://docs.rs/tui-banner/)
[![License](https://img.shields.io/crates/l/tui-banner.svg)](https://crates.io/crates/tui-banner)

![banner example](assets/images/banner.png)

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading, light sweeps
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

## Light Sweep Animation

```rust
use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let banner = Banner::new("RUST CLI")?
        .gradient(Gradient::diagonal(Palette::from_hex(&[
            "#00E5FF", "#7B5CFF", "#FF5AD9",
        ])))
        .fill(Fill::Keep)
        .align(Align::Center)
        .padding(1);

    banner.animate_sweep(5, None)?;
    Ok(())
}
```

Tip: pass `Some(Color::Rgb(...))` as the second argument to tint the sweep.

![Light Sweep Animation](assets/images/light_sweep.gif)

## Gradient Gallery

<table>
  <tr>
    <th>Vertical</th>
    <th>Horizontal</th>
  </tr>
  <tr>
    <td><img alt="Gradient Vertical" src="assets/images/gradient_vertical.png" /></td>
    <td><img alt="Gradient Horizontal" src="assets/images/gradient_horizontal.png" /></td>
  </tr>
  <tr>
    <th>Diagonal</th>
    <th>Apply</th>
  </tr>
  <tr>
    <td><img alt="Gradient Diagonal" src="assets/images/gradient_diagonal.png" /></td>
    <td><img alt="Gradient Apply" src="assets/images/gradient_apply.png" /></td>
  </tr>
</table>

## Full Gallery

<table>
  <tr>
    <th>Block</th>
    <th>Half Block</th>
  </tr>
  <tr>
    <td><img alt="Full Block" src="assets/images/full_block.png" /></td>
    <td><img alt="Full Half Block" src="assets/images/full_half_block.png" /></td>
  </tr>
  <tr>
    <th>Light Shade</th>
    <th>Char</th>
  </tr>
  <tr>
    <td><img alt="Full Light Shade" src="assets/images/full_light_shade.png" /></td>
    <td><img alt="Full Char" src="assets/images/full_char.png" /></td>
  </tr>
</table>

## Dither Gallery

<table>
  <tr>
    <th>Checker Stipple</th>
    <th>Coarse Halftone</th>
  </tr>
  <tr>
    <td><img alt="Dither Checker Stipple" src="assets/images/dither_checker_stipple.png" /></td>
    <td><img alt="Dither Coarse Halftone" src="assets/images/dither_coarse_halftone.png" /></td>
  </tr>
  <tr>
    <th>Film Grain</th>
    <th>Sparkle Noise</th>
  </tr>
  <tr>
    <td><img alt="Dither Film Grain" src="assets/images/dither_film_grain.png" /></td>
    <td><img alt="Dither Sparkle Noise" src="assets/images/dither_sparkle_noise.png" /></td>
  </tr>
</table>

## CLI Usage

Install the CLI (binary name: `tui-banner`):

```bash
Download the prebuilt binary from GitHub Releases and place it on your PATH:
https://github.com/coolbeevip/tui-banner/releases
```

```bash
tui-banner
tui-banner --text "HELLO WORLD"
tui-banner --text "HELLO WORLD" --style neon-cyber
tui-banner --text "HELLO WORLD" --gradient diagonal --palette "#00E5FF,#7B5CFF,#FF5AD9"
tui-banner --text "HELLO WORLD" --animate-sweep 3 --sweep-highlight "#DCEBFF"
tui-banner --text "HELLO WORLD" --animate-sweep 3 --sweep-highlight "#DCEBFF" --sweep-direction horizontal
tui-banner --text "HELLO WORLD" --animate-sweep 3 \
    --sweep-direction diagonal-up --sweep-width 0.3 --sweep-intensity 0.9 \
    --sweep-softness 2.5 --sweep-center 0.4 --sweep-highlight "#DCEBFF"
```

Defaults (CLI):
- `--text` is required
- Gradient: diagonal
- Palette: #00E5FF, #3A7BFF, #E6F6FF
- Fill: keep
- Dither targets: ░▒▓ (when dither enabled)
- Padding: 1
- Align: center
- Color mode: truecolor
- Sweep highlight: white

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
cargo run --example light_sweep
```
