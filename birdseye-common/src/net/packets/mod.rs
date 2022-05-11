use crate::net::packets::Error::ErrorReadingName;
use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::{BufRead, Cursor, Read};
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

    #[error("Could not read the content for the field {0} on the incoming packet: {1}")]
    ErrorReadingContent(String, std::io::Error),
}

/// A packet to be sent from either the client or the server
///
/// # Packet structure
/// Each packet uses the following layout to encode information
/// - `name`: \[u8, u8\] The two letter name of the field, i.e. ID.
/// - `size`: u32 (LittleEndian): The number of bytes that that field takes up
/// - `data`: \[u8; `size`\]: The content of that field
///
/// # Reserved names
/// There are a few reserved names
/// - `ID`: the ID of the packet that is being send to the client
pub struct Packet {
    /// The id of the packet
    id: u32,
    /// Any fields and their data
    fields: HashMap<String, Vec<u8>>,
}

impl Packet {
    pub fn from_bytes(bytes: Vec<u8>) -> ParsePacketResult<Self> {
        // Get the length of the bytes
        let byte_len = bytes.len() as u64;

        // Check that there is actually some data in the packet (minimum size of id field)
        if byte_len < 10 {
            return Err(Error::TooSmallPacket);
        }

        let mut fields = HashMap::new();

        // Get a cursor to read the data
        let mut cursor = Cursor::new(bytes);

        // Loop until all da bytes are read
        while cursor.position() != byte_len as u64 {
            // Read the name of the field
            let name = {
                let mut temp = [0; 2];
                match cursor.read_exact(&mut temp) {
                    Ok(_) => {}
                    Err(err) => return Err(Error::ErrorReadingName(err)),
                };

                match String::from_utf8(temp.to_vec()) {
                    Ok(val) => val.to_uppercase(),
                    Err(err) => return Err(Error::ErrorParsingName(err)),
                }
            };

            let size = match cursor.read_u32::<LittleEndian>() {
                Ok(val) => val,
                Err(err) => return Err(ErrorReadingName(err)),
            } as usize;

            let data = {
                let mut temp = Vec::with_capacity(size);
                match cursor.read_exact(temp.as_mut_slice()) {
                    Ok(_) => {}
                    Err(err) => return Err(Error::ErrorReadingContent(name, err)),
                };
                temp
            };

            fields.insert(name, data);
        }

        match fields.get("ID") {
            None => Err(Error::NoId),
            Some(id) => {
                if id.len() != 4 {
                    Err(Error::NoId)
                } else {
                    let bytes = [id[0], id[1], id[2], id[3]];
                    fields.remove("id");

                    Ok(Self {
                        id: u32::from_le_bytes(bytes),
                        fields,
                    })
                }
            }
        }
    }
}
