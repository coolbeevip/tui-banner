use tui_banner::{Banner, Style};

fn main() -> Result<(), tui_banner::BannerError> {
    println!();
    let banner = Banner::new("RUST CLI")? // text
        .style(Style::RoyalPurple) // style
        .render();

    println!("{banner}");
    Ok(())
}
