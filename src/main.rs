use std::io;
use rust_i18n::i18n;

mod client;
mod server;

i18n!("locales", fallback = "en-US");
fn main() -> io::Result<()> {
    client::run()
}
