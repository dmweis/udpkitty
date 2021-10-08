mod multicast;

use anyhow::Result;
use clap::Clap;
use multicast::MessageSender;
use std::net::SocketAddrV4;

#[derive(Clap)]
#[clap()]
struct Args {
    #[clap(short, long, default_value = "239.0.0.22:7071")]
    address: SocketAddrV4,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let sender = MessageSender::new(args.address)?;
    loop {
        if let Ok(message) = sender.receive() {
            println!("{}", message);
        }
    }
}
