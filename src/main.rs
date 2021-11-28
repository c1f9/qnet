use std::net::UdpSocket;

use quiche;
use smol::Async;

pub struct QuicSocket {
    config: quiche::Config,
    socket: Async<UdpSocket>,
}

impl QuicSocket {
    pub fn new() -> Result<Self> {
        let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION)?;
        config.set_cc_algorithm_name("reno")?;

        let mut socket = Async::<UdpSocket>::bind("")?;

        Self { config, socket }
    }
}

fn main() {
    println!("Hello, world!");
}
