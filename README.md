# tui-banner

Cinematic ANSI banners for Rust CLI/TUI.

## Features

- Grid-first rendering pipeline
- Bundled DOS Rebel (Figlet) font + load any `.flf`
- Truecolor / 256-color / no-color output with auto-detect
- Gradients, pixel fill, dithering, shadows, edge shading
- Named style and palette presets
- Fluent builder API

## Color Gallery

<table>
  <tr>
    <td align="center"><strong>Neon Cyber</strong><br><img src="assets/examples/neon_gradient_neon_cyber.png" alt="Neon Cyber" width="360"></td>
    <td align="center"><strong>Arctic Tech</strong><br><img src="assets/examples/neon_gradient_arctic_tech.png" alt="Arctic Tech" width="360"></td>
  </tr>
  <tr>
    <td align="center"><strong>Aurora Flux</strong><br><img src="assets/examples/neon_gradient_aurora_flux.png" alt="Aurora Flux" width="360"></td>
    <td align="center"><strong>Deep Space</strong><br><img src="assets/examples/neon_gradient_deep_space.png" alt="Deep Space" width="360"></td>
  </tr>
  <tr>
    <td align="center"><strong>Ocean Flow</strong><br><img src="assets/examples/neon_gradient_ocean_flow.png" alt="Ocean Flow" width="360"></td>
    <td align="center"><strong>Sunset Neon</strong><br><img src="assets/examples/neon_gradient_sunset_neon.png" alt="Sunset Neon" width="360"></td>
  </tr>
  <tr>
    <td align="center"><strong>Fire Warning</strong><br><img src="assets/examples/neon_gradient_fire_warning.png" alt="Fire Warning" width="360"></td>
    <td align="center"><strong>Warm Luxury</strong><br><img src="assets/examples/neon_gradient_warm_luxury.png" alt="Warm Luxury" width="360"></td>
  </tr>
  <tr>
    <td align="center"><strong>Forest Sky</strong><br><img src="assets/examples/neon_gradient_forest_sky.png" alt="Forest Sky" width="360"></td>
    <td align="center"><strong>Earth Tone</strong><br><img src="assets/examples/neon_gradient_earth_tone.png" alt="Earth Tone" width="360"></td>
  </tr>
  <tr>
    <td align="center"><strong>Chrome</strong><br><img src="assets/examples/neon_gradient_chrome.png" alt="Chrome" width="360"></td>
    <td align="center"><strong>Royal Purple</strong><br><img src="assets/examples/neon_gradient_royal_purple.png" alt="Royal Purple" width="360"></td>
  </tr>
  <tr>
    <td align="center"><strong>CRT Amber</strong><br><img src="assets/examples/neon_gradient_crt_amber.png" alt="CRT Amber" width="360"></td>
    <td align="center"><strong>Matrix</strong><br><img src="assets/examples/neon_gradient_matrix.png" alt="Matrix" width="360"></td>
  </tr>
</table>

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
