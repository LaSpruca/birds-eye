use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

pub type ParsePacketResult<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Could not read the incoming packet, as it is too small")]
    TooSmallPacket,

    #[error("Could not find ID for packet")]
    NoId,

    #[error("Could not pass the length for a field on the incoming packet: {0}")]
    ErrorParsingLen(std::io::Error),

    #[error("Could not read the name for a field on the incoming packet: {0}")]
    ErrorReadingName(std::io::Error),

    #[error("Could not parse name for incoming packet: {0}")]
    ErrorParsingName(FromUtf8Error),

    #[error("Could not read the content for the a field in the incoming packet: {1}")]
    ErrorReadingContent(std::io::Error),

    #[error("Could not read the content for the field {0} in the incoming packet: {1}")]
    ErrorReadingContentNamed(String, std::io::Error),

    #[error("Could not read the content for the a field in the incoming packet: {1}")]
    InvalidString(FromUtf8Error),

    #[error("Could not read the content for the field {0} in the incoming packet: {1}")]
    InvalidStringNamed(String, FromUtf8Error),
}