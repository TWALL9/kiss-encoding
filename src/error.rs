#[derive(Debug)]
pub enum EncodeError {
    WriteError,
}

#[derive(Debug)]
pub enum DecodeError {
    FramingError,
    InputTooShort,
    UnexpectedEscapeVal(u8),
    IncorrectValAfterEscape(u8),
}
