#![cfg_attr(not(test), no_std)]

pub mod constants;
pub mod decode;
pub mod encode;
pub mod error;

#[cfg(test)]
mod test {
    use super::*;

    use decode::{DataFrame, DecodedVal};
    use constants::{FEND, FESC, TFEND, TFESC};
    use error::DecodeError;

    #[test]
    fn decode_basics() {
        let test: [u8; 7]= [0xC0, 0x00, 0x54, 0x45, 0x53, 0x54, 0xC0];

        let mut decoder = DataFrame::default();
        let start_fend = decoder.decode_byte(test[0]).unwrap().unwrap();
        assert_eq!(start_fend, DecodedVal::StartFend);

        let port = decoder.decode_byte(test[1]).unwrap().unwrap();
        assert_eq!(port, DecodedVal::DataPort(0));

        for b in 2..6 {
            let d = decoder.decode_byte(test[b]).unwrap().unwrap();
            assert_eq!(d, DecodedVal::Data(test[b]));
        }

        let d = decoder.decode_byte(*test.last().unwrap()).unwrap();
        assert_eq!(d, Some(DecodedVal::EndFend));
    }

    #[test]
    fn decode_escape_character() {
        let test = [FESC, TFEND, FESC, TFESC];
        let mut decoder = DataFrame::default();

        let _start = decoder.decode_byte(FEND).unwrap().unwrap();
        let _port = decoder.decode_byte(0).unwrap().unwrap();
        
        for t in test.iter() {
            assert!(decoder.decode_byte(*t).is_ok())
        }
    }

    #[test]
    fn detect_bogus_escape_character() {
        let bogus = [FEND, 0, FESC, 0, 0, FEND];
        let mut decoder = DataFrame::default();

        let _start = decoder.decode_byte(bogus[0]).unwrap().unwrap();
        let _port = decoder.decode_byte(bogus[1]).unwrap().unwrap();

        let fesc = decoder.decode_byte(bogus[2]).unwrap();
        assert_eq!(fesc, Some(DecodedVal::Escape));

        let bad_escape_val = decoder.decode_byte(bogus[3]);
        assert!(matches!(bad_escape_val, Err(DecodeError::IncorrectValAfterEscape(0))));

        let ignored = decoder.decode_byte(bogus[4]).unwrap();
        assert!(ignored.is_none());

        let fend = decoder.decode_byte(bogus[5]).unwrap();
        assert_eq!(fend, Some(DecodedVal::EndFend));
    }
}
