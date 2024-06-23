use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

use crate::message::{self, Message};

pub struct ObcClient {
    addr: String,
    port: u32,
    stream: Option<TcpStream>,
}

impl ObcClient {
    pub fn new(addr: String, port: u32) -> Self {
        ObcClient { addr, port, stream: None }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        if self.stream.is_none() {
            match TcpStream::connect(format!("{}:{}", self.addr, self.port)).await {
                Ok(s) => self.stream = Some(s),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    pub async fn send_cmd(&mut self, input: [&str; 3]) -> Result<u8, Box<dyn Error>> {
        let mut data = parse_command(input)?;
        let stream: &mut tokio::net::TcpStream = match &mut self.stream {
            Some(s) => s,
            None => {
                return Err("not connected".into());
            }
        };
        stream.write(&data).await?;

        match stream.read(&mut data).await {
            Ok(len) => if len == message::MSG_LEN {
                Ok(handle_response(&data))
            }
            else {
                println!("short read: {len} bytes");
                Err("short response".into())
            },
            Err(e) => {
                println!("Failed to receive data: {}", e);
                Err(e.into())
            }
        }
    }
}

fn parse_command(input: [&str; 3]) -> Result<Message, Box<dyn Error>> {
    let cmd = message::Command {
        payload: match message::Payload::from_str(input[0]) {
            Ok(p) => p,
            Err(e) => return Err(e.into()),
        },
        opcode: match input[1].parse::<u8>() {
            Ok(op) => op,
            Err(_) => return Err("opcode must be numerical".into()),
        },
        oplen: 0,
        opdata: [0; message::MSG_OPDATA_LEN],
    };

    println!("serializing: {:?}", cmd);
    Ok(cmd.serialize())
}

fn handle_response(msg: &Message) -> u8 {
    let reply = message::Command::deserialize(msg);

    println!("Response status: {}", reply.opcode);
    reply.opcode
}
