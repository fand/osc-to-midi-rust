extern crate rosc;
extern crate midir;

use std::env;
use std::net::{UdpSocket, SocketAddrV4};
use std::str::FromStr;
use std::error::Error;
use rosc::OscPacket;
use midir::{MidiOutput, MidiOutputConnection};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description())
    }
}

fn run() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage {} IP:PORT MIDI_PORT_INDEX", &args[0]);

    if args.len() < 3 {
        println!("{}", usage);
        ::std::process::exit(1);
    }

    let addr = match SocketAddrV4::from_str(&args[1]) {
        Ok(addr) => addr,
        Err(_) => panic!(usage),
    };
    let sock = UdpSocket::bind(addr).unwrap();

    let midi_out = MidiOutput::new("midir forwarding output")?;
    let out_port: usize = args[2].parse()?;
    let mut conn_out = midi_out.connect(out_port, "osc-to-midi")?;

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                handle_packet(&packet, &mut conn_out);
            }
            Err(_e) => {
                break;
            }
        }
    }

    Ok(())
}

fn handle_packet(packet: &OscPacket, conn_out: &mut MidiOutputConnection) {
    match packet {
        OscPacket::Message(msg) => {

            if msg.addr != "/midi" {
                return;
            }

            match &msg.args {
                Some(args) => {
                    handle_midi_osc(args, conn_out);
                }
                None => println!("No args")
            }
        }
        OscPacket::Bundle(bundle) => {
            // println!("Received OSC bundle: {:?}", bundle);
            for x in &(bundle.content) {
                handle_packet(x, conn_out);
            }
        }
    }
}

fn handle_midi_osc(args: &Vec<rosc::OscType>, conn_out: &mut MidiOutputConnection) {
    let mut message: Vec<u8> = Vec::new();
    for x in args {
        match x {
            rosc::OscType::Int(i) => message.push(*i as u8),
            rosc::OscType::Float(f) => message.push((*f * 127.0) as u8),
            _ => {}
        }
    }
    println!("Sending midi: {:?}", message);

    (*conn_out).send(&message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
}
