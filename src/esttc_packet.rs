/*
Written by Devin Headrick

Define the ESTTC protocol packet strcuture for reading and writing with radio in ESTTC mode

Notes:
All commands are in ASCII format

When the uhf radio is put into pipe mode it emits a " +PIPE " to the UART

- Status Control Word (SCW) for pipe mode is bit # 5
    therefore to set pipe mode you set bit 5 in the SCW , 1=on , 0=off

What is the SCW used by ex2 to set pipe command?

SCW bits are :
0 RTFS - Inidicates if radio transceiver is config correctly
1 FRAM
2 SEC
3 CTS
4 Mode - 1=bootloader, 0=application
5 Pipe - 1=On, 0=off
6 BCN  - beacon message control
7 Echo
8, 9, 10 - RF mode
11 Rest - Writing this bit 1 resets the device
12, 13 UART Baud rate
14 HFXT
15 reserved

Thus to set pipe mode:
LSB -> MSB
    0000
    0(1)11 - app mode, pipe mode, enable beacon message control, enable echo
    1100 - default RF mode
    1100 - default UART baud (115200)

Resulting SCW: 
    0000011111001100

TODO - How is the CRC calculated for an ESTTC packet? 

TODO - Setup hardcoded esttc packet to put uhf into pipe mode as example

*/

use crate::helper_fxns::calc_crc32;

pub struct ESTTC_packet {
    header: [u8; 3],       //ES+
    command_type: u8,      //R , W
    address: [u8; 2],      //22, 33
    command_code: [u8; 2], //00 - FF
    command_data: Vec<u8>, // 0 - 110 bytes
    blank: u8,             //optional  0x20 HEX (space ascii)
    crc32: [u8; 8],        //optional
    cr: u8,                //Carriage return  0x0D HEX (CR ascii)
}

pub struct ESTTC_packet_no_CRC {
    header: [u8; 3],       //ES+
    command_type: u8,      //R , W
    address: [u8; 2],      //22, 33
    command_code: [u8; 2], //00 - FF
    command_data: Vec<u8>, // 0 - 110 bytes
    cr: u8,                //Carriage return  0x0D HEX (CR ascii)
}

// In order to set the uhf into pipe mode, the following 5th bit must be set HIGH
const SCW_set_pipe_mode: u16 = 0b0000011111001100;
//                                    ^

impl ESTTC_packet {

    //CRC polynomial: 0x04C11DB7
    fn calc_crc32(&mut self) {
        //use helper fxn crc calc
        //let crc_result = calc_crc32()
    }
    
}
