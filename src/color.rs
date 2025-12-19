/// Supported color types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    /// 24-bit RGB color.
    Rgb(u8, u8, u8),
    /// ANSI 256-color palette index.
    Ansi256(u8),
}

/// Color output mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    /// Auto-detect from environment.
    Auto,
    /// 24-bit truecolor output.
    TrueColor,
    /// 256-color output.
    Ansi256,
    /// Disable color output.
    NoColor,
}

/// Palette of colors for gradients.
#[derive(Clone, Debug)]
pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    /// Create a palette from colors.
    pub fn new(colors: Vec<Color>) -> Self {
        Self { colors }
    }

    /// Create a palette from hex strings (invalid entries are ignored).
    pub fn from_hex(hexes: &[&str]) -> Self {
        let mut colors = Vec::with_capacity(hexes.len());
        for hex in hexes {
            if let Some(color) = parse_hex_color(hex) {
                colors.push(color);
            }
        }
        Self { colors }
    }

    /// Get palette colors.
    pub fn colors(&self) -> &[Color] {
        &self.colors
    }
}

impl Color {
    /// Linear interpolation between colors.
    pub fn lerp(self, other: Color, t: f32) -> Color {
        match (self, other) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => {
                let t = t.clamp(0.0, 1.0);
                let r = (r1 as f32 + (r2 as f32 - r1 as f32) * t).round() as u8;
                let g = (g1 as f32 + (g2 as f32 - g1 as f32) * t).round() as u8;
                let b = (b1 as f32 + (b2 as f32 - b1 as f32) * t).round() as u8;
                Color::Rgb(r, g, b)
            }
            (left, _) => left,
        }
    }
}

fn parse_hex_color(input: &str) -> Option<Color> {
    let hex = input.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}
