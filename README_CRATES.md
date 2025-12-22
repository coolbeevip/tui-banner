# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading, light sweeps
- Named style and palette presets
- Fluent builder API

CLI note: download the `tui-banner` binary from GitHub Releases and place it on your PATH.

```text
https://github.com/coolbeevip/tui-banner/releases
```

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

## Custom Example

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
