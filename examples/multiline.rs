use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("MULTI\nLINE\nBANNER") {
        Ok(banner) => banner
            .gradient(Gradient::vertical(Palette::from_hex(&[
                "#00C2FF", "#00FFA3",
            ])))
            .fill(Fill::Keep)
            .line_gap(1)
            .max_width(60)
            .align(Align::Center)
            .padding((1, 2, 1, 2))
            .render(),
        Err(err) => {
            eprintln!("tui-banner: {err}");
            return;
        }
    };

    println!("{banner}");
}
