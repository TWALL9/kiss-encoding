use core::{fmt::Write, result::Result};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::{vec, vec::Vec};

use crate::constants::{FEND, FESC, TFEND, TFESC};
use crate::error::EncodeError;

pub fn encode_write<W: Write>(w: &mut W, port: u8, msg: &[u8]) -> Result<(), EncodeError> {
    let port_nib = (port & 0x0F) << 4;
    w.write_char(FEND as char)
        .map_err(|_| EncodeError::WriteError)?;
    w.write_char(port_nib as char)
        .map_err(|_| EncodeError::WriteError)?;

    for b in msg.iter() {
        if *b == FEND {
            w.write_char(FESC as char)
                .map_err(|_| EncodeError::WriteError)?;
            w.write_char(TFEND as char)
                .map_err(|_| EncodeError::WriteError)?;
        } else if *b == FESC {
            w.write_char(FESC as char)
                .map_err(|_| EncodeError::WriteError)?;
            w.write_char(TFESC as char)
                .map_err(|_| EncodeError::WriteError)?;
        } else {
            w.write_char(*b as char)
                .map_err(|_| EncodeError::WriteError)?;
        }
    }

    w.write_char(FEND as char)
        .map_err(|_| EncodeError::WriteError)?;

    Result::Ok(())
}

#[cfg(feature = "alloc")]
pub fn encode(port: u8, msg: &[u8]) -> Result<Vec<u8>, EncodeError> {
    let port_nib = (port & 0x0F) << 4;
    let mut v = vec![FEND, port_nib];

    for b in msg.iter() {
        if *b == FEND {
            v.push(FESC);
            v.push(TFEND);
        } else if *b == FESC {
            v.push(FESC);
            v.push(TFESC);
        } else {
            v.push(*b);
        }
    }

    v.push(FEND);

    Result::Ok(v)
}