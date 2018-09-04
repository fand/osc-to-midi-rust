extern crate rosc;
extern crate midir;

use std::env;
use std::net::{UdpSocket, SocketAddrV4};
use std::str::FromStr;
use std::error::Error;
use rosc::OscPacket;
use midir::MidiOutput;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let midi_out = MidiOutput::new("midir forwarding output")?;

    for i in 0..midi_out.port_count() {
        println!("{}: {}", i, midi_out.port_name(i)?);
    }

    Ok(())
}
