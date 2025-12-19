use std::collections::HashMap;

use super::{Font, Glyph};

/// Errors when parsing Figlet fonts.
#[derive(Debug)]
pub enum FigletError {
    /// Header line is missing or invalid.
    InvalidHeader,
    /// Not enough data for glyphs.
    MissingData,
    /// Numeric field parse error.
    InvalidNumber,
}

/// Parse a Figlet `.flf` string into a font.
pub fn parse(data: &str) -> Result<Font, FigletError> {
    let mut lines = data.lines();
    let header = lines.next().ok_or(FigletError::InvalidHeader)?;
    let (hardblank, height, comment_lines) = parse_header(header)?;

    for _ in 0..comment_lines {
        lines.next().ok_or(FigletError::MissingData)?;
    }

    let mut glyphs: HashMap<char, Glyph> = HashMap::new();
    let mut endmark: Option<char> = None;

    for code in 32u8..=126u8 {
        let mut rows: Vec<Vec<char>> = Vec::with_capacity(height);
        for _ in 0..height {
            let line = lines.next().ok_or(FigletError::MissingData)?;
            let marker = endmark.get_or_insert_with(|| line.chars().last().unwrap_or('@'));
            let cleaned = clean_line(line, *marker, hardblank);
            rows.push(cleaned.chars().collect());
        }
        glyphs.insert(code as char, Glyph { rows });
    }

    let fallback = glyphs.get(&'?').cloned().unwrap_or_else(|| Glyph {
        rows: vec![vec!['?'; 1]; height],
    });

    Ok(Font {
        height,
        glyphs,
        fallback,
    })
}

fn parse_header(line: &str) -> Result<(char, usize, usize), FigletError> {
    if !line.starts_with("flf2a") || line.len() < 6 {
        return Err(FigletError::InvalidHeader);
    }
    let hardblank = line.chars().nth(5).ok_or(FigletError::InvalidHeader)?;
    let mut parts = line.split_whitespace();
    parts.next();
    let height = parse_usize(parts.next())?;
    let _baseline = parse_usize(parts.next())?;
    let _max_len = parse_usize(parts.next())?;
    let _old_layout = parse_i32(parts.next())?;
    let comment_lines = parse_usize(parts.next())?;
    Ok((hardblank, height, comment_lines))
}

fn parse_usize(part: Option<&str>) -> Result<usize, FigletError> {
    part.ok_or(FigletError::InvalidHeader)?
        .parse::<usize>()
        .map_err(|_| FigletError::InvalidNumber)
}

fn parse_i32(part: Option<&str>) -> Result<i32, FigletError> {
    part.ok_or(FigletError::InvalidHeader)?
        .parse::<i32>()
        .map_err(|_| FigletError::InvalidNumber)
}

fn clean_line(line: &str, endmark: char, hardblank: char) -> String {
    let mut trimmed = line.trim_end_matches(endmark).to_string();
    trimmed = trimmed.replace(hardblank, " ");
    trimmed
}
