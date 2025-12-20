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

use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let banner = Banner::new(">RUST CLI")?
        .gradient(Gradient::diagonal(Palette::from_hex(&[
            "#00E5FF", "#7B5CFF", "#FF5AD9",
        ])))
        .fill(Fill::Keep)
        .align(Align::Center)
        .padding(1);

    banner.animate_sweep(5, Some(tui_banner::Color::Rgb(255, 210, 120)))?;
    Ok(())
}
