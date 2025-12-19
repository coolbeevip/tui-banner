use tui_banner::{Align, Banner, Dither, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("PIXEL") {
        Ok(banner) => banner
            .gradient(Gradient::diagonal(Palette::from_hex(&[
                "#FFD86E", "#FF6B6B",
            ])))
            .fill(Fill::pixel_with_dither('#', Dither::checker(3, ".:")))
            .align(Align::Center)
            .padding(1)
            .render(),
        Err(err) => {
            eprintln!("tui-banner: {err}");
            return;
        }
    };

    println!("{banner}");
}
