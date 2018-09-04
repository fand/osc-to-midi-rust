extern crate rosc;

use std::env;
use std::net::{UdpSocket, SocketAddrV4};
use std::str::FromStr;
use rosc::OscPacket;

fn main() {
    let args: Vec<String> = env::args().collect();
    let usage = format!("Usage {} IP:PORT", &args[0]);
    if args.len() < 2 {
        println!("{}", usage);
        ::std::process::exit(1);
    }
    let addr = match SocketAddrV4::from_str(&args[1]) {
        Ok(addr) => addr,
        Err(_) => panic!(usage),
    };
    let sock = UdpSocket::bind(addr).unwrap();
    println!("Listening to {}", addr);

    let mut buf = [0u8; rosc::decoder::MTU];

    loop {
        match sock.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("Received packet with size {} from {}", size, addr);
                let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                handle_packet(&packet);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn handle_packet(packet: &OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("Received OSC packet: {:?}", msg);

            if msg.addr != "/midi" {
                return;
            }

            match &msg.args {
                Some(args) => {
                    handle_midi_osc(args);
                }
                None => println!("No args")
            }
        }
        OscPacket::Bundle(bundle) => {
            println!("Received OSC bundle: {:?}", bundle);
            for x in &(bundle.content) {
                handle_packet(x);
            }
        }
    }
}

fn handle_midi_osc(args: &Vec<rosc::OscType>) {
    println!("midi: {:?}", args);
}
