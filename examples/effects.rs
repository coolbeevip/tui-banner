use tui_banner::{Align, Banner, Fill, Gradient, Palette};

fn main() {
    let banner = match Banner::new("EFFECTS") {
        Ok(banner) => banner
            .gradient(Gradient::diagonal(Palette::from_hex(&[
                "#FFB86C", "#FF5E5E",
            ])))
            .fill(Fill::Blocks)
            .edge_shade(0.4, '#')
            .shadow((2, 1), 0.35)
            .dither()
            .targets("#")
            .dots(".:")
            .checker(3)
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
