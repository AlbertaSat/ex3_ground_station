use std::fmt;
    
#[derive(Debug)]
pub struct Payload<'a> {
    pub id : usize,
    pub name: &'a str,
}

impl Payload<'_> {
    #[allow(dead_code)]
    pub fn from_str(cmd: &str) -> Result<&'static Payload<'static>, &'static str> {
        for p in PAYLOADS {
            if cmd == p.name {
                return Ok(p)
            }
        }
        Err("Unrecognized command name")
    }

    #[allow(dead_code)]
    pub fn from_int(id: u8) -> Result<&'static Payload<'static>, &'static str> {
        for p in PAYLOADS {
            if usize::from(id) == p.id {
                return Ok(p)
            }
        }
        Err("Unrecognized payload id")
    }
}

pub const PAYLOAD_EPS: &str = "eps";
pub const PAYLOAD_ADCS: &str = "adcs";
pub const PAYLOAD_DFGM: &str = "dfgm";

pub const PAYLOADS: &[Payload] = &[
    Payload { id : 0, name : "unknown" },
    Payload { id : 1, name : PAYLOAD_EPS },
    Payload { id : 2, name : PAYLOAD_DFGM },
    Payload { id : 3, name : PAYLOAD_ADCS },
];


impl fmt::Display for Payload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", self.name, self.id)
    }
}

// The positions of the fields of the message header
const MSG_LEN_IX : usize = 0;
const MSG_DST_IX : usize = 1;
const MSG_OP_IX : usize = 2;

pub const MSG_LEN : usize = 64;
pub const MSG_OPDATA_OFF : usize = 3;
pub const MSG_OPDATA_LEN : usize = MSG_LEN - MSG_OPDATA_OFF;

pub type Message = [u8; MSG_LEN];

#[derive(Debug)]
pub struct Command {
    pub payload: &'static Payload<'static>,
    pub opcode: u8,
    pub oplen: usize,
    pub opdata: [u8; MSG_OPDATA_LEN],
}

impl Command {
    #[allow(dead_code)]
    pub fn deserialize(msg: &Message) -> Command {
        let len: usize = usize::from(msg[MSG_LEN_IX]);
        let mut cmd = Command {
            oplen : usize::from(msg[MSG_LEN_IX]) - MSG_OPDATA_OFF,
            payload : match Payload::from_int(msg[MSG_DST_IX]) {
                Ok(p) => p,
                Err(_) => {
                    println!("unknown payload {}", msg[MSG_DST_IX]);
                    &PAYLOADS[0]
                },
            },
            opcode: msg[MSG_OP_IX],
            opdata: [0; MSG_OPDATA_LEN],
        };
        cmd.opdata[..cmd.oplen].copy_from_slice(&msg[MSG_OPDATA_OFF..len]);
        cmd
    }

    #[allow(dead_code)]
    pub fn serialize(&self) -> Message {
        let mut msg: Message = [0; MSG_LEN];
        msg[MSG_LEN_IX] = (self.oplen + MSG_OPDATA_OFF) as u8;
        msg[MSG_DST_IX] = self.payload.id as u8;
        msg[MSG_OP_IX] = self.opcode;
        if self.oplen > 0 {
            msg[MSG_OPDATA_OFF..].copy_from_slice(&self.opdata[0..self.oplen]);
        }
        msg
    }

    #[allow(dead_code)]
    pub fn status_msg(&self, status: u8) -> Message {
        let mut msg: Message = [0; MSG_LEN];
        msg[MSG_LEN_IX] = MSG_OPDATA_OFF as u8;
        msg[MSG_DST_IX] = self.payload.id as u8;
        msg[MSG_OP_IX] = self.opcode;
        msg[MSG_OPDATA_OFF] = status;
        msg
    }
}
