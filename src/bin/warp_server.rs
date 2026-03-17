use std::io;
use clap::Parser;
use warp::server;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long,default_value = "7000")]
    port: u16
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    server::run(args.port);
    Ok(())
}
