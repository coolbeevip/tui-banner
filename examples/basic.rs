use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("RUST CLI") {
        Ok(banner) => banner
            .gradient(Gradient::vertical(Palette::from_hex(&[
                "#00E5FF", "#7B5CFF", "#FF5AD9",
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
