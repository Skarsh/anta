use std::array::TryFromSliceError;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum ElfParseError {
    InvalidMagicBytes,
    BytesConversion(TryFromSliceError),
}

impl ElfParseError {
    fn as_str(&self) -> &'static str {
        match *self {
            ElfParseError::InvalidMagicBytes => "invalid magic bytes, this is not an elf file",
            ElfParseError::BytesConversion(_) => "failed to convert bytes to type or machine enum",
        }
    }
}

impl Display for ElfParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Error for ElfParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            Self::BytesConversion(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<TryFromSliceError> for ElfParseError {
    fn from(err: TryFromSliceError) -> Self {
        Self::BytesConversion(err)
    }
}
