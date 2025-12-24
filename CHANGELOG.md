# Changelog

## [0.2.3]
### Added
- Frame rendering API (`Frame`, `FrameStyle`, `FrameChars`) and `Banner::frame` to draw borders.
- Frame paint support for solid colors and gradients.
- CLI frame options: `--frame`, `--frame-chars`, `--frame-color`, `--frame-gradient`,
  `--frame-palette`, and `--frame-preset`.
- Frame example (`examples/frame_gradient.rs`) plus documentation assets.

### Added
- CLI: `--trim-vertical` and `--no-trim-vertical` to control trimming of blank rows.

### Changed
- CLI: trim blank rows at the top/bottom by default (use `--no-trim-vertical` to keep them).

## [0.2.2]
### Added
- `animate_roll` API and CLI option `--animate-roll`.
- `--wave-dim` and `--wave-bright` options for `--animate-wave`.
- Wave/roll animation demo assets in documentation.

### Changed
- `animate_wave` now accepts optional dim/bright tuning parameters.

## [0.2.1]
### Added
- `animate_wave` API and CLI option `--animate-wave`.
- Workspace CLI crate (`tui-banner-cli`) and `tui-banner` binary.

### Changed
- CLI moved out of `src/main.rs` into the workspace crate.

## [0.2.0]
### Added
- Light sweep effect (`LightSweep`) and `animate_sweep` API.
- Light sweep example and demo assets.

## [0.1.4]
### Added
- Dither and gradient example assets.
- New dither examples (`examples/dither_*`).

### Changed
- Documentation updates.

## [0.1.3]
### Changed
- Website/docs cleanup (removed `www/` assets).

## [0.1.2]
### Added
- Crates.io README (`README_CRATES.md`).

### Changed
- Website/docs updates.

## [0.1.1]
### Added
- Style presets (`Style`) and palette presets (`Preset`).
- Dot dithering API (`dither`, `checker`, `noise`, target selection).
- CI + pre-commit config, plus expanded examples/docs.

## [0.1.0]
### Added
- Initial release: banner rendering with gradients, fills (solid/blocks/pixel), outlines/shadows,
  alignment/padding/kerning/line-gap, ANSI output, and Figlet font support.
