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

use tui_banner::{Align, Banner, ColorMode, Fill, Gradient, Palette};

fn main() -> Result<(), tui_banner::BannerError> {
    println!();
    let banner = Banner::new("RUST CLI")? // text
        .color_mode(ColorMode::TrueColor)
        .gradient(Gradient::horizontal(Palette::from_hex(&[
            "#FFB347", // orange
            "#FF6B6B", // coral
            "#D645FF", // purple
        ])))
        .fill(Fill::Keep)
        .dither()
        .targets("░▒▓")
        .dots("o:")
        .checker(3)
        .align(Align::Center)
        .padding(1)
        .render();

    println!("{banner}");
    Ok(())
}
