mod cli;
pub mod device;
pub mod textbox;
pub mod timespan;
mod tui;
pub mod wattage;

use clap::Parser;
use cli::Args;
use std::io::Result;
use tui::App;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut terminal = ratatui::init();
    let mut app = App::new(!args.no_color_devices);
    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}
