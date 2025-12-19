use tui_banner::{Align, Banner, ColorMode, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("NO COLOR") {
        Ok(banner) => banner
            .color_mode(ColorMode::NoColor)
            .gradient(Gradient::vertical(Palette::from_hex(&[
                "#444444", "#AAAAAA",
            ])))
            .fill(Fill::Blocks)
            .align(Align::Left)
            .padding(1)
            .render(),
        Err(err) => {
            eprintln!("tui-banner: {err}");
            return;
        }
    };

    println!("{banner}");
}
