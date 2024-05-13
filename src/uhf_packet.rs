/*
Written by Devin Headrick

This defines an UHF packet, for communicating with Endurosat UHF transceiver


//TODO - build a larger data file like an image from a series of incoming packets


Provided packet structure for Endurosat UHF:
+-----------+---------+-------------+---------------------+
| Field     | Length  | Description | Byte/Bit Order      |
+-----------+---------+-------------+---------------------+
| Preamble  | 5 bytes | Value:      | LSB first           |
|           |         | 0xAAAAAAAAAA|                     |
+-----------+---------+-------------+---------------------+
| Sync Word | 1 byte  | Value:      | LSB first           |
|           |         | 0x7E        |                     |
+-----------+---------+-------------+---------------------+
| Data Field| 1 byte  | Length of   | MSB first           |
| 1         |         | "Data Field |                     |
|           |         | 2"          |                     |
+-----------+---------+-------------+---------------------+
| Data Field| 20-128  | Payload     | MSB first           |
| 2         |  bytes  | (variable   |                     |
|           |         | number of   |                     |
|           |         | bytes)      |                     |
+-----------+---------+-------------+---------------------+
| CRC       | 2 bytes | CRC-16/     |                     |
|           |         | CCITT-FALSE | MSB first. The CRC  |
|           |         | Polynomial: | is applied to Data  |
|           |         | x16 + x12 +| Field 1 + Data     |
|           |         | x5 + 1      | Field 2*.           |
|           |         | Seed        |                     |
|           |         | (initial    |                     |
|           |         | value): all |                     |
|           |         | 1’s         |                     |
+-----------+---------+-------------+---------------------+


*/

pub struct UHF_packet {
    preamble: [u8; 5], // 0xAAAAAAAAAA
    sync_word: u8,     // 0x7E
    payload_length: u8,
    payload_data: Vec<u8>, //variable 1 - 128 bytes
    crc16: [u8; 2],
}

impl UHF_packet {
    //TODO create an UHF packet from input data
    fn new(data: [u8; 128]) {
        //TODO - this

        self.calc_crc();
    }

    fn calc_crc(&mut self) {
        self.crc16 = 10;
    }
}
