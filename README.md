# tui-banner

Rust CLI/TUI 彩色 ASCII Art Banner 渲染库 / Rust library for colorful ASCII art banners.

状态：实验性（API 可能变化） / Status: experimental (API may change).

## Features / 特性

- Grid-first 渲染管线（文本 → Glyph Grid → Effects → ANSI 输出）
- 内置 DOS Rebel (Figlet) 字体
- Truecolor / 256 色 / 无色输出 + 自动检测
- 渐变（垂直 / 水平 / 对角）
- 像素填充、点阵抖动、阴影、边缘半透明效果
- Builder API，组合性强

## Install / 安装

在 `Cargo.toml` 添加： / Add to `Cargo.toml`:

```toml
[dependencies]
tui-banner = { path = "." }
```

## Quick Start / 快速开始

```rust
use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("RUST CLI") {
        Ok(banner) => banner
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
            .render(),
        Err(err) => {
            eprintln!("tui-banner: {err}");
            String::new()
        }
    };

    println!("{banner}");
}
```

Alternatively, use `?` and return a `Result` from `main`:

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

## API Overview / API 概览

### Banner（高层 API）

- `Banner::new(text) -> Result<Banner, BannerError>`
- `font(Font::dos_rebel()? | Font::from_figlet_str(...))`
- `gradient(Gradient::vertical | horizontal | diagonal)`
- `fill(Fill::Keep | Fill::Blocks | Fill::Pixel { ... })`
- `dither().targets("░▒").dots("·:").checker(k)`
- `edge_shade(darken, ch)`（例如 `0.5, '░'`）
- `shadow((dx, dy), alpha)`
- `align(Align::Left | Center | Right)`
- `padding(u32)`
- `render() -> String`

### Low-level（底层）

- `font::render_text(...) -> Grid`
- `Gradient::apply(&mut Grid)`
- `emit::emit_ansi(&Grid, ColorMode)`

## Dither Builder / 点阵 Builder

```rust
Banner::new("HELLO")?
    .fill(Fill::Keep)
    .dither()
    .targets("░▒▓")
    .dots("·:")
    .checker(3)
    .render();
```

Patterns / 模式:

- `checker(period)`
- `noise(seed, threshold)`

## Figlet Fonts / Figlet 字体

内置 DOS Rebel 字体： / Bundled DOS Rebel:

```rust
Font::dos_rebel()?
```

加载任意 Figlet `.flf`： / Load any Figlet `.flf`:

```rust
let font = Font::from_figlet_str(flf_str)?;
```

## Examples / 示例

运行示例： / Run examples:

```bash
cargo run --example basic
cargo run --example effects
cargo run --example multiline
cargo run --example no_color
cargo run --example pixel
```

## Color Modes / 颜色模式

- `ColorMode::TrueColor`
- `ColorMode::Ansi256`
- `ColorMode::NoColor`
- `ColorMode::Auto`（默认，读取 `NO_COLOR` / `COLORTERM` / `TERM`）

## Docs / 文档

本地生成 API 文档： / Generate API docs locally:

```bash
cargo doc --no-deps --open
```

Header 检查（insert-license）： / Header check (insert-license):

```bash
insert-license check
```

Pre-commit（推荐）： / Pre-commit (recommended):

```bash
pre-commit install --hook-type pre-commit --hook-type pre-push
```

## Project Layout / 目录结构

```
src/
 ├─ banner.rs
 ├─ color.rs
 ├─ emit.rs
 ├─ effects/
 │   ├─ dither.rs
 │   ├─ outline.rs
 │   └─ shadow.rs
 ├─ fill.rs
 ├─ font/
 │   ├─ builtin.rs
 │   ├─ figlet.rs
 │   └─ mod.rs
 ├─ gradient.rs
 ├─ grid.rs
 └─ terminal.rs
assets/
 └─ fonts/
    └─ dosrebel.flf
```

## Notes / 备注

- 使用 Unicode block 字符（如 `█ ▓ ▒ ░`），请确保终端字体支持。
- Figlet 解析目前只覆盖 ASCII 32..126，暂未实现 smushing/kern 规则。

## License / 许可

Apache-2.0
