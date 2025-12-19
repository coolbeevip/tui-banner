use tui_banner::{Align, Banner, Fill, Font, Gradient, Palette};

fn main() {
    let banner = Banner::new("RUST CLI")
        .font(Font::dos_rebel())
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
}
