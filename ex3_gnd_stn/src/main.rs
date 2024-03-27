use std::{
    io::{self, Write},
    net::TcpStream,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut data: [u8; 64] = [0; 64];

    // The first three bytes are header stuff, the rest will be payload
    // We are just trying to send to ADCS right now

    data[0] = 64;
    data[1] = 3;
    data[2] = 69;

    let x = stream.write(&data)?;
    println!("{x}");
    Ok(())
}
