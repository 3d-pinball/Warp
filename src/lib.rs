use rust_i18n::i18n;

pub mod client;
pub mod shared;
pub mod server;

i18n!("locales", fallback = "en-US");
