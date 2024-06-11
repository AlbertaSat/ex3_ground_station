/**
 * Written by Devin headrick
 * 2024 summer
 *
 * Simple CLI to write commands to the OBC via TCP. This establishes a TCP client and sends data as bytes
 */

use std::env;
use std::io::Write;
use std::net::Ipv4Addr;
use std::net::TcpStream;

fn main() {
    println!("Writing data to OBC FSW via TCP client socket connection");
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage: <obc_port> <subsystem_id> <subsystem_op_code> optional:<data> ...");
        return;
    }

    let port = args[1].parse::<u16>().unwrap();

    let subsystem_id = args[2].parse::<u8>().unwrap();

    let subsystem_op_code = args[3].parse::<u8>().unwrap();

    let mut data: Vec<u8> = Vec::new();

    if args.len() > 4 {
        //If there are more than 4 arguments, then we have data to send
        for i in 4..args.len() {
            data = args[i].clone().into_bytes();
        }
    }

    let mut stream = TcpStream::connect((Ipv4Addr::new(127, 0, 0, 1), port)).unwrap();
    let output_stream = &mut stream;

    let command_bytes = build_command_bytes(subsystem_id, subsystem_op_code, data);

    output_stream.write(&command_bytes).unwrap();
    output_stream.flush().unwrap();
}

fn build_command_bytes(subsystem_id: u8, subsystem_op_code: u8, data: Vec<u8>) -> Vec<u8> {
    let mut command: Vec<u8> = Vec::new();
    let len: u8 = data.len() as u8 + 3;
    command.push(len);
    command.push(subsystem_id);
    command.push(subsystem_op_code);

    for i in 0..data.len() {
        command.push(data[i]);
    }
    println!("Command Byte Values: {:?}", command);
    return command;
}
