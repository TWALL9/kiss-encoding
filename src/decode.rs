use crate::constants::{FEND, FESC, TFEND, TFESC};
use crate::error::DecodeError;

#[derive(Default, Clone, Copy)]
enum DecodeState {
    #[default]
    WaitForStartFend,
    GetPort,
    Decoding,
    Escape,
    InError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodedVal {
    StartFend,
    DataPort(u8),
    Data(u8),
    Escape,
    EndFend,
}

#[derive(Default)]
pub struct DataFrame {
    state: DecodeState,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            state: DecodeState::WaitForStartFend,
        }
    }

    pub fn decode_byte(&mut self, b: u8) -> Result<Option<DecodedVal>, DecodeError> {
        match (self.state, b) {
            (DecodeState::WaitForStartFend, FEND) => {
                self.state = DecodeState::GetPort;
                Ok(Some(DecodedVal::StartFend))
            }
            (DecodeState::WaitForStartFend, _) => Ok(None),
            (DecodeState::GetPort, FEND | FESC | TFEND | TFESC) => {
                self.state = DecodeState::InError;
                Err(DecodeError::InputTooShort)
            }
            (DecodeState::GetPort, b) => {
                self.state = DecodeState::Decoding;
                let port = (b & 0xF0) >> 4;
                Ok(Some(DecodedVal::DataPort(port)))
            }
            (DecodeState::Decoding, FESC) => {
                self.state = DecodeState::Escape;
                Ok(Some(DecodedVal::Escape))
            }
            (DecodeState::Decoding, FEND) => {
                self.state = DecodeState::WaitForStartFend;
                Ok(Some(DecodedVal::EndFend))
            }
            (DecodeState::Decoding, b) if b == TFEND | TFESC => {
                self.state = DecodeState::InError;
                Err(DecodeError::UnexpectedEscapeVal(b))
            }
            (DecodeState::Decoding, b) => Ok(Some(DecodedVal::Data(b))),
            (DecodeState::Escape, b) => {
                if b == TFEND {
                    self.state = DecodeState::Decoding;
                    Ok(Some(DecodedVal::Data(FEND)))
                } else if b == TFESC {
                    self.state = DecodeState::Decoding;
                    Ok(Some(DecodedVal::Data(FESC)))
                } else {
                    self.state = DecodeState::InError;
                    Err(DecodeError::IncorrectValAfterEscape(b))
                }
            }
            (DecodeState::InError, b) => {
                if b == FEND {
                    self.state = DecodeState::WaitForStartFend;
                    Ok(Some(DecodedVal::EndFend))
                } else {
                    Ok(None)
                }
            }
        }
    }
}
