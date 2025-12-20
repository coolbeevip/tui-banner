// Copyright (c) 2025 Lei Zhang
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

use std::env;
use std::fs;
use std::path::PathBuf;

use tui_banner::{
    Align, Banner, Color, ColorMode, Dither, Fill, Font, Gradient, GradientDirection, LightSweep,
    Palette, Preset, Style, SweepDirection,
};

const DEFAULT_PALETTE: [&str; 3] = ["#00E5FF", "#3A7BFF", "#E6F6FF"];

#[derive(Default)]
struct CliOptions {
    text_flag: Option<String>,
    text_positional: Vec<String>,
    font: Option<PathBuf>,
    style: Option<Style>,
    preset: Option<Preset>,
    gradient: Option<GradientDirection>,
    palette: Option<Vec<String>>,
    fill: Option<FillKind>,
    fill_char: Option<char>,
    pixel_dither: Option<DitherSpec>,
    pixel_dither_dots: Option<String>,
    dither: Option<DitherSpec>,
    dither_targets: Option<String>,
    dither_dots: Option<String>,
    shadow: Option<ShadowSpec>,
    edge_shade: Option<EdgeShadeSpec>,
    align: Option<Align>,
    padding: Option<tui_banner::Padding>,
    width: Option<usize>,
    max_width: Option<usize>,
    kerning: Option<usize>,
    line_gap: Option<usize>,
    color_mode: Option<ColorMode>,
    light_sweep: bool,
    sweep_direction: Option<SweepDirection>,
    sweep_center: Option<f32>,
    sweep_width: Option<f32>,
    sweep_intensity: Option<f32>,
    sweep_softness: Option<f32>,
    animate_sweep: Option<u64>,
    sweep_highlight: Option<Color>,
}

#[derive(Clone, Copy)]
enum FillKind {
    Keep,
    Blocks,
    Solid,
    Pixel,
}

#[derive(Clone, Copy)]
enum DitherSpec {
    Checker { period: u8 },
    Noise { seed: u32, threshold: u8 },
}

#[derive(Clone, Copy)]
struct ShadowSpec {
    offset: (i32, i32),
    alpha: f32,
}

#[derive(Clone, Copy)]
struct EdgeShadeSpec {
    darken: f32,
    ch: char,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("tui-banner: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let opts = parse_args()?;
    let text = resolve_text(&opts)?;
    let mut banner = Banner::new(text).map_err(|err| err.to_string())?;

    if let Some(font_path) = opts.font.as_ref() {
        let data = fs::read_to_string(font_path)
            .map_err(|err| format!("failed to read font {:?}: {err}", font_path))?;
        let font = Font::from_figlet_str(&data).map_err(|err| format!("{err:?}"))?;
        banner = banner.font(font);
    }

    if let Some(style) = opts.style {
        banner = banner.style(style);
    }

    let color_mode = opts.color_mode.unwrap_or(ColorMode::TrueColor);
    banner = banner.color_mode(color_mode);

    let fill = build_fill(
        opts.fill.or(Some(FillKind::Keep)),
        opts.fill_char,
        opts.pixel_dither,
        opts.pixel_dither_dots.as_deref(),
    )?;
    if let Some(fill) = fill {
        banner = banner.fill(fill);
    }

    if let Some(shadow) = opts.shadow {
        banner = banner.shadow(shadow.offset, shadow.alpha);
    }

    if let Some(edge_shade) = opts.edge_shade {
        banner = banner.edge_shade(edge_shade.darken, edge_shade.ch);
    }

    let align = opts.align.unwrap_or(Align::Center);
    banner = banner.align(align);

    let padding = opts
        .padding
        .unwrap_or_else(|| tui_banner::Padding::uniform(1));
    banner = banner.padding(padding);

    if let Some(width) = opts.width {
        banner = banner.width(width);
    }

    if let Some(max_width) = opts.max_width {
        banner = banner.max_width(max_width);
    }

    if let Some(kerning) = opts.kerning {
        banner = banner.kerning(kerning);
    }

    if let Some(line_gap) = opts.line_gap {
        banner = banner.line_gap(line_gap);
    }

    let gradient = resolve_gradient(&opts)?;
    if let Some(gradient) = gradient {
        banner = banner.gradient(gradient);
    }

    if should_apply_sweep(&opts) {
        let sweep = build_sweep(&opts)?;
        banner = banner.light_sweep(sweep);
    }

    banner = apply_dot_dither(banner, &opts)?;

    if let Some(speed) = opts.animate_sweep {
        let highlight = opts.sweep_highlight;
        banner
            .animate_sweep(speed, highlight)
            .map_err(|err| err.to_string())?;
        return Ok(());
    }

    println!("{}", banner.render());
    Ok(())
}

fn parse_args() -> Result<CliOptions, String> {
    let mut opts = CliOptions::default();
    let args: Vec<String> = env::args().skip(1).collect();
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        if arg == "--help" || arg == "-h" {
            print_help();
            std::process::exit(0);
        }

        if arg.starts_with("--") {
            let (flag, inline) = split_arg(arg);
            match flag {
                "--text" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    if opts.text_flag.is_some() {
                        return Err("`--text` specified more than once".to_string());
                    }
                    opts.text_flag = Some(value);
                }
                "--font" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.font = Some(PathBuf::from(value));
                }
                "--style" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.style = Some(parse_style(&value)?);
                }
                "--preset" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.preset = Some(parse_preset(&value)?);
                }
                "--gradient" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.gradient = Some(parse_gradient_dir(&value)?);
                }
                "--palette" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    let entries = parse_list(&value);
                    if entries.is_empty() {
                        return Err("`--palette` expects at least one color".to_string());
                    }
                    opts.palette.get_or_insert_with(Vec::new).extend(entries);
                }
                "--fill" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.fill = Some(parse_fill(&value)?);
                }
                "--fill-char" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.fill_char = Some(parse_char(&value)?);
                }
                "--pixel-dither-checker" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    let period = parse_u8(&value, flag)?;
                    if opts.pixel_dither.is_some() {
                        return Err("only one pixel dither mode can be set".to_string());
                    }
                    opts.pixel_dither = Some(DitherSpec::Checker { period });
                }
                "--pixel-dither-noise" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    let (seed, threshold) = parse_seed_threshold(&value, flag)?;
                    if opts.pixel_dither.is_some() {
                        return Err("only one pixel dither mode can be set".to_string());
                    }
                    opts.pixel_dither = Some(DitherSpec::Noise { seed, threshold });
                }
                "--pixel-dither-dots" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    validate_dots(&value)?;
                    opts.pixel_dither_dots = Some(value);
                }
                "--dither-checker" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    let period = parse_u8(&value, flag)?;
                    if opts.dither.is_some() {
                        return Err("only one dither mode can be set".to_string());
                    }
                    opts.dither = Some(DitherSpec::Checker { period });
                }
                "--dither-noise" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    let (seed, threshold) = parse_seed_threshold(&value, flag)?;
                    if opts.dither.is_some() {
                        return Err("only one dither mode can be set".to_string());
                    }
                    opts.dither = Some(DitherSpec::Noise { seed, threshold });
                }
                "--dither-targets" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.dither_targets = Some(value);
                }
                "--dither-dots" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    validate_dots(&value)?;
                    opts.dither_dots = Some(value);
                }
                "--shadow" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.shadow = Some(parse_shadow(&value)?);
                }
                "--edge-shade" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.edge_shade = Some(parse_edge_shade(&value)?);
                }
                "--align" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.align = Some(parse_align(&value)?);
                }
                "--padding" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.padding = Some(parse_padding(&value)?);
                }
                "--width" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.width = Some(parse_usize(&value, flag)?);
                }
                "--max-width" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.max_width = Some(parse_usize(&value, flag)?);
                }
                "--kerning" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.kerning = Some(parse_usize(&value, flag)?);
                }
                "--line-gap" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.line_gap = Some(parse_usize(&value, flag)?);
                }
                "--color-mode" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.color_mode = Some(parse_color_mode(&value)?);
                }
                "--light-sweep" => {
                    opts.light_sweep = true;
                }
                "--sweep-direction" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.sweep_direction = Some(parse_sweep_direction(&value)?);
                }
                "--sweep-center" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.sweep_center = Some(parse_f32(&value, flag)?);
                }
                "--sweep-width" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.sweep_width = Some(parse_f32(&value, flag)?);
                }
                "--sweep-intensity" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.sweep_intensity = Some(parse_f32(&value, flag)?);
                }
                "--sweep-softness" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.sweep_softness = Some(parse_f32(&value, flag)?);
                }
                "--animate-sweep" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.animate_sweep = Some(parse_u64(&value, flag)?);
                }
                "--sweep-highlight" => {
                    let value = take_value(flag, inline, &args, &mut index)?;
                    opts.sweep_highlight = Some(parse_color(&value)?);
                }
                _ => return Err(format!("unknown flag: {flag}")),
            }
        } else {
            opts.text_positional.push(arg.clone());
        }
        index += 1;
    }

    validate_options(&opts)?;
    Ok(opts)
}

fn resolve_text(opts: &CliOptions) -> Result<String, String> {
    if opts.text_flag.is_some() && !opts.text_positional.is_empty() {
        return Err("provide text either via `--text` or positional arguments".to_string());
    }
    if let Some(text) = &opts.text_flag {
        return Ok(text.clone());
    }
    if !opts.text_positional.is_empty() {
        return Ok(opts.text_positional.join(" "));
    }
    Ok("RUST CLI".to_string())
}

fn resolve_gradient(opts: &CliOptions) -> Result<Option<Gradient>, String> {
    let mut gradient_dir = opts.gradient;
    if gradient_dir.is_none() {
        if opts.style.is_some() && opts.palette.is_none() && opts.preset.is_none() {
            return Ok(None);
        }
        gradient_dir = Some(GradientDirection::Diagonal);
    }

    let direction = gradient_dir.unwrap_or(GradientDirection::Diagonal);

    let palette = if let Some(palette) = &opts.palette {
        let list: Vec<&str> = palette.iter().map(String::as_str).collect();
        let palette = Palette::from_hex(&list);
        if palette.colors().is_empty() {
            return Err("`--palette` did not contain any valid colors".to_string());
        }
        palette
    } else if let Some(preset) = opts.preset {
        Palette::preset(preset)
    } else {
        Palette::from_hex(&DEFAULT_PALETTE)
    };

    let gradient = match direction {
        GradientDirection::Vertical => Gradient::vertical(palette),
        GradientDirection::Horizontal => Gradient::horizontal(palette),
        GradientDirection::Diagonal => Gradient::diagonal(palette),
    };
    Ok(Some(gradient))
}

fn build_fill(
    fill: Option<FillKind>,
    fill_char: Option<char>,
    pixel_dither: Option<DitherSpec>,
    pixel_dither_dots: Option<&str>,
) -> Result<Option<Fill>, String> {
    let Some(fill) = fill else {
        if fill_char.is_some() || pixel_dither.is_some() || pixel_dither_dots.is_some() {
            return Err("`--fill` is required when setting fill-related options".to_string());
        }
        return Ok(None);
    };

    let fill = match fill {
        FillKind::Keep => Fill::Keep,
        FillKind::Blocks => Fill::Blocks,
        FillKind::Solid => {
            let ch = fill_char.ok_or("`--fill solid` requires `--fill-char`")?;
            Fill::Solid(ch)
        }
        FillKind::Pixel => {
            let ch = fill_char.ok_or("`--fill pixel` requires `--fill-char`")?;
            if let Some(spec) = pixel_dither {
                let dots = pixel_dither_dots.unwrap_or("·");
                let dither = build_dither(spec, dots)?;
                Fill::pixel_with_dither(ch, dither)
            } else {
                Fill::pixel(ch)
            }
        }
    };

    Ok(Some(fill))
}

fn build_dither(spec: DitherSpec, dots: &str) -> Result<Dither, String> {
    match spec {
        DitherSpec::Checker { period } => Ok(Dither::checker(period, dots)),
        DitherSpec::Noise { seed, threshold } => Ok(Dither::noise(seed, threshold, dots)),
    }
}

fn apply_dot_dither(mut banner: Banner, opts: &CliOptions) -> Result<Banner, String> {
    if opts.dither.is_none() {
        if opts.dither_targets.is_some() || opts.dither_dots.is_some() {
            return Err(
                "`--dither-checker` or `--dither-noise` is required when setting dither options"
                    .to_string(),
            );
        }
        return Ok(banner);
    }

    let mut builder = banner.dither();
    if let Some(targets) = &opts.dither_targets {
        builder = builder.targets(targets);
    } else {
        builder = builder.targets("░▒▓");
    }
    if let Some(dots) = &opts.dither_dots {
        builder = builder.dots(dots);
    }

    banner = match opts.dither.unwrap() {
        DitherSpec::Checker { period } => builder.checker(period),
        DitherSpec::Noise { seed, threshold } => builder.noise(seed, threshold),
    };

    Ok(banner)
}

fn should_apply_sweep(opts: &CliOptions) -> bool {
    opts.light_sweep
        || opts.sweep_center.is_some()
        || opts.sweep_width.is_some()
        || opts.sweep_intensity.is_some()
        || opts.sweep_softness.is_some()
        || opts.sweep_direction.is_some()
}

fn build_sweep(opts: &CliOptions) -> Result<LightSweep, String> {
    let direction = opts.sweep_direction.unwrap_or(SweepDirection::DiagonalDown);
    let mut sweep = LightSweep::new(direction);
    if let Some(center) = opts.sweep_center {
        sweep = sweep.center(center);
    }
    if let Some(width) = opts.sweep_width {
        sweep = sweep.width(width);
    }
    if let Some(intensity) = opts.sweep_intensity {
        sweep = sweep.intensity(intensity);
    }
    if let Some(softness) = opts.sweep_softness {
        sweep = sweep.softness(softness);
    }
    Ok(sweep)
}

fn validate_options(opts: &CliOptions) -> Result<(), String> {
    if opts.animate_sweep.is_some() && should_apply_sweep(opts) {
        return Err(
            "`--animate-sweep` cannot be combined with static light sweep options".to_string(),
        );
    }
    if opts.sweep_highlight.is_some() && opts.animate_sweep.is_none() {
        return Err("`--sweep-highlight` requires `--animate-sweep`".to_string());
    }
    if opts.pixel_dither.is_some() && !matches!(opts.fill, Some(FillKind::Pixel)) {
        return Err("pixel dither options require `--fill pixel`".to_string());
    }
    if opts.pixel_dither.is_none() && opts.pixel_dither_dots.is_some() {
        return Err("`--pixel-dither-dots` requires a pixel dither mode".to_string());
    }
    Ok(())
}

fn split_arg(arg: &str) -> (&str, Option<&str>) {
    arg.split_once('=')
        .map_or((arg, None), |(k, v)| (k, Some(v)))
}

fn take_value(
    flag: &str,
    inline: Option<&str>,
    args: &[String],
    index: &mut usize,
) -> Result<String, String> {
    if let Some(value) = inline {
        return Ok(value.to_string());
    }
    *index += 1;
    if *index >= args.len() {
        return Err(format!("missing value for {flag}"));
    }
    Ok(args[*index].clone())
}

fn parse_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn parse_fill(value: &str) -> Result<FillKind, String> {
    match normalize(value).as_str() {
        "keep" => Ok(FillKind::Keep),
        "blocks" => Ok(FillKind::Blocks),
        "solid" => Ok(FillKind::Solid),
        "pixel" => Ok(FillKind::Pixel),
        _ => Err("`--fill` must be keep, blocks, solid, or pixel".to_string()),
    }
}

fn parse_style(value: &str) -> Result<Style, String> {
    match normalize(value).as_str() {
        "neon-cyber" => Ok(Style::NeonCyber),
        "arctic-tech" => Ok(Style::ArcticTech),
        "sunset-neon" => Ok(Style::SunsetNeon),
        "forest-sky" => Ok(Style::ForestSky),
        "chrome" => Ok(Style::Chrome),
        "crt-amber" => Ok(Style::CrtAmber),
        "ocean-flow" => Ok(Style::OceanFlow),
        "deep-space" => Ok(Style::DeepSpace),
        "fire-warning" => Ok(Style::FireWarning),
        "warm-luxury" => Ok(Style::WarmLuxury),
        "earth-tone" => Ok(Style::EarthTone),
        "royal-purple" => Ok(Style::RoyalPurple),
        "matrix" => Ok(Style::Matrix),
        "aurora-flux" => Ok(Style::AuroraFlux),
        other => Err(format!("unknown style: {other}")),
    }
}

fn parse_preset(value: &str) -> Result<Preset, String> {
    match normalize(value).as_str() {
        "neon-cyber" => Ok(Preset::NeonCyber),
        "arctic-tech" => Ok(Preset::ArcticTech),
        "sunset-neon" => Ok(Preset::SunsetNeon),
        "forest-sky" => Ok(Preset::ForestSky),
        "chrome" => Ok(Preset::Chrome),
        "crt-amber" => Ok(Preset::CrtAmber),
        "ocean-flow" => Ok(Preset::OceanFlow),
        "deep-space" => Ok(Preset::DeepSpace),
        "fire-warning" => Ok(Preset::FireWarning),
        "warm-luxury" => Ok(Preset::WarmLuxury),
        "earth-tone" => Ok(Preset::EarthTone),
        "royal-purple" => Ok(Preset::RoyalPurple),
        "matrix" => Ok(Preset::Matrix),
        "aurora-flux" => Ok(Preset::AuroraFlux),
        other => Err(format!("unknown preset: {other}")),
    }
}

fn parse_gradient_dir(value: &str) -> Result<GradientDirection, String> {
    match normalize(value).as_str() {
        "vertical" => Ok(GradientDirection::Vertical),
        "horizontal" => Ok(GradientDirection::Horizontal),
        "diagonal" | "diag" => Ok(GradientDirection::Diagonal),
        other => Err(format!("unknown gradient direction: {other}")),
    }
}

fn parse_align(value: &str) -> Result<Align, String> {
    match normalize(value).as_str() {
        "left" => Ok(Align::Left),
        "center" => Ok(Align::Center),
        "right" => Ok(Align::Right),
        other => Err(format!("unknown alignment: {other}")),
    }
}

fn parse_color_mode(value: &str) -> Result<ColorMode, String> {
    match normalize(value).as_str() {
        "auto" => Ok(ColorMode::Auto),
        "truecolor" | "true-color" => Ok(ColorMode::TrueColor),
        "ansi256" | "ansi-256" => Ok(ColorMode::Ansi256),
        "no-color" | "nocolor" | "none" => Ok(ColorMode::NoColor),
        other => Err(format!("unknown color mode: {other}")),
    }
}

fn parse_sweep_direction(value: &str) -> Result<SweepDirection, String> {
    match normalize(value).as_str() {
        "horizontal" => Ok(SweepDirection::Horizontal),
        "vertical" => Ok(SweepDirection::Vertical),
        "diagonal" | "diag-down" | "diagonal-down" => Ok(SweepDirection::DiagonalDown),
        "diag-up" | "diagonal-up" => Ok(SweepDirection::DiagonalUp),
        other => Err(format!("unknown sweep direction: {other}")),
    }
}

fn parse_char(value: &str) -> Result<char, String> {
    let mut chars = value.chars();
    let ch = chars
        .next()
        .ok_or_else(|| "expected a character".to_string())?;
    if chars.next().is_some() {
        return Err("expected a single character".to_string());
    }
    Ok(ch)
}

fn validate_dots(value: &str) -> Result<(), String> {
    let count = value.chars().count();
    if count == 0 || count > 2 {
        return Err("dots must contain 1 or 2 characters".to_string());
    }
    Ok(())
}

fn parse_shadow(value: &str) -> Result<ShadowSpec, String> {
    let parts = parse_list(value);
    if parts.len() != 3 {
        return Err("`--shadow` expects dx,dy,alpha".to_string());
    }
    let dx = parts[0]
        .parse::<i32>()
        .map_err(|_| "shadow dx must be an integer".to_string())?;
    let dy = parts[1]
        .parse::<i32>()
        .map_err(|_| "shadow dy must be an integer".to_string())?;
    let alpha = parts[2]
        .parse::<f32>()
        .map_err(|_| "shadow alpha must be a float".to_string())?;
    Ok(ShadowSpec {
        offset: (dx, dy),
        alpha,
    })
}

fn parse_edge_shade(value: &str) -> Result<EdgeShadeSpec, String> {
    let parts = parse_list(value);
    if parts.len() != 2 {
        return Err("`--edge-shade` expects darken,char".to_string());
    }
    let darken = parts[0]
        .parse::<f32>()
        .map_err(|_| "edge shade darken must be a float".to_string())?;
    let ch = parse_char(&parts[1])?;
    Ok(EdgeShadeSpec { darken, ch })
}

fn parse_padding(value: &str) -> Result<tui_banner::Padding, String> {
    let parts = parse_list(value);
    match parts.len() {
        1 => Ok(tui_banner::Padding::from(
            parts[0]
                .parse::<usize>()
                .map_err(|_| "padding must be a number".to_string())?,
        )),
        4 => Ok(tui_banner::Padding::from((
            parse_usize(&parts[0], "--padding")?,
            parse_usize(&parts[1], "--padding")?,
            parse_usize(&parts[2], "--padding")?,
            parse_usize(&parts[3], "--padding")?,
        ))),
        _ => Err("`--padding` expects 1 or 4 comma-separated numbers".to_string()),
    }
}

fn parse_seed_threshold(value: &str, flag: &str) -> Result<(u32, u8), String> {
    let parts = parse_list(value);
    if parts.len() != 2 {
        return Err(format!("{flag} expects seed,threshold"));
    }
    let seed = parts[0]
        .parse::<u32>()
        .map_err(|_| format!("{flag} seed must be a number"))?;
    let threshold = parts[1]
        .parse::<u32>()
        .map_err(|_| format!("{flag} threshold must be a number"))?;
    if threshold > u8::MAX as u32 {
        return Err(format!("{flag} threshold must be 0..=255"));
    }
    Ok((seed, threshold as u8))
}

fn parse_color(value: &str) -> Result<Color, String> {
    if value.contains(',') {
        let parts = parse_list(value);
        if parts.len() != 3 {
            return Err("color expects r,g,b".to_string());
        }
        let r = parse_u8(&parts[0], "color")?;
        let g = parse_u8(&parts[1], "color")?;
        let b = parse_u8(&parts[2], "color")?;
        return Ok(Color::Rgb(r, g, b));
    }

    let hex = value.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return Err("color expects #RRGGBB or r,g,b".to_string());
    }
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "invalid hex".to_string())?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "invalid hex".to_string())?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "invalid hex".to_string())?;
    Ok(Color::Rgb(r, g, b))
}

fn parse_usize(value: &str, flag: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|_| format!("{flag} must be a number"))
}

fn parse_u8(value: &str, flag: &str) -> Result<u8, String> {
    let parsed = value
        .parse::<u32>()
        .map_err(|_| format!("{flag} must be a number"))?;
    if parsed > u8::MAX as u32 {
        return Err(format!("{flag} must be 0..=255"));
    }
    Ok(parsed as u8)
}

fn parse_u64(value: &str, flag: &str) -> Result<u64, String> {
    value
        .parse::<u64>()
        .map_err(|_| format!("{flag} must be a number"))
}

fn parse_f32(value: &str, flag: &str) -> Result<f32, String> {
    value
        .parse::<f32>()
        .map_err(|_| format!("{flag} must be a float"))
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('_', "-")
}

fn print_help() {
    println!(
        r#"tui-banner [TEXT] [options]

Options:
  --text <TEXT>                 Banner text (fallback to positional TEXT, default: "RUST CLI")
  --font <PATH>                 Figlet .flf font file
  --style <STYLE>               neon-cyber | arctic-tech | sunset-neon | forest-sky | chrome
                                crt-amber | ocean-flow | deep-space | fire-warning | warm-luxury
                                earth-tone | royal-purple | matrix | aurora-flux
  --gradient <DIR>              vertical | horizontal | diagonal (default: diagonal)
  --palette <HEXES>             Comma-separated hex colors (default: #00E5FF,#3A7BFF,#E6F6FF)
  --preset <PRESET>             Palette preset (same names as styles)
  --fill <FILL>                 keep | blocks | solid | pixel (default: keep)
  --fill-char <CHAR>            Character for solid/pixel fills
  --pixel-dither-checker <N>    Pixel dither checker period
  --pixel-dither-noise <S,T>    Pixel dither noise (seed,threshold)
  --pixel-dither-dots <DOTS>    Pixel dither dots (1-2 chars)
  --dither-checker <N>          Dot dither checker period
  --dither-noise <S,T>          Dot dither noise (seed,threshold)
  --dither-targets <STR>        Dither glyph targets (default: ░▒▓)
  --dither-dots <DOTS>          Dither dots (1-2 chars)
  --shadow <DX,DY,A>            Drop shadow (offset + alpha)
  --edge-shade <D,CH>           Edge shade (darken + char)
  --align <ALIGN>               left | center | right (default: center)
  --padding <P>                 1 or 4 comma-separated values (default: 1)
  --width <N>                   Force output width
  --max-width <N>               Clamp output width
  --kerning <N>                 Space between characters
  --line-gap <N>                Blank lines between text lines
  --color-mode <MODE>           auto | truecolor | ansi256 | no-color (default: truecolor)
  --light-sweep                 Enable static sweep
  --sweep-direction <DIR>       horizontal | vertical | diagonal-down | diagonal-up
  --sweep-center <F>            Sweep center (0..1)
  --sweep-width <F>             Sweep width (0..1)
  --sweep-intensity <F>         Sweep intensity (0..1)
  --sweep-softness <F>          Sweep softness (>=1)
  --animate-sweep <MS>          Animate sweep (frame delay in ms)
  --sweep-highlight <COLOR>     Highlight color (#RRGGBB or r,g,b, default: white)
  --help, -h                    Show this help
"#
    );
}
