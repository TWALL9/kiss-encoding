// In-frame delimiters
pub const FEND: u8 = 0xC0;
pub const FESC: u8 = 0xDB;
pub const TFEND: u8 = 0xDC;
pub const TFESC: u8 = 0xDD;

// Command codes
pub const DATA_FRAME: u8 = 0x00;
pub const TX_DELAY: u8 = 0x01;
pub const PERISTANCE: u8 = 0x02;
pub const SLOT_TIME: u8 = 0x03;
pub const TX_TAIL: u8 = 0x04;
pub const FULL_DUPLEX: u8 = 0x05;
pub const SET_HARDWARE: u8 = 0x06;
pub const RETURN: u8 = 0xFF;
