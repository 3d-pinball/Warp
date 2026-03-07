use ratatui::prelude::Stylize;
use ratatui::widgets::Widget;
use std::io;

mod client;
mod server;

fn main() -> io::Result<()> {
    client::run()
}
