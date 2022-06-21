mod desereialize;
mod error;

use byteorder::{LittleEndian, ReadBytesExt};
use error::{Error, ParsePacketResult};
use std::{
    collections::HashMap,
    io::{self, BufRead, Cursor, Read},
};

type PC = Cursor<Vec<u8>>;

/// A packet to be sent from either the client or the server
///
/// # Packet structure (KV Pair)
/// Each packet uses the following layout to encode information
/// - `key`: \[u8; 2\] - The two letter key of the field, i.e. ID.
/// - `type`: u16 - The type id of the data contained in the structure
/// - `size`: u32 - The number of bytes in the value section, only used for dynamic sized types
/// - `value`: \[u8; `size`\] - The content of that field
///
/// # Reserved keys
/// There are a few reserved names
/// - `ID`: the ID of the packet that is being send to the client (u32)
///
/// # Packet field types
/// ## Static size type
/// - `0` - bool
/// - `1` - u8
/// - `2` - i8
/// - `3` - u16
/// - `4` - i16
/// - `5` - u32
/// - `6` - i32
/// - `7` - u64
/// - `8` - i64
/// - `9` - u128
/// - `10` - i128
/// - `11` - f32
/// - `12` - f64
/// ## Dynamic sized types
/// - `13` - String
/// - `14` - KV Pair
/// - `15` - Array
/// - `16` - GZipped Array of groups of 3 bytes (r,g,b)
pub struct Packet {
    /// The id of the packet
    id: u32,
    /// Any fields and their data
    fields: HashMap<String, Vec<u8>>,
}

impl Packet {
    pub fn from_bytes(bytes: Vec<u8>) -> ParsePacketResult<Self> {
        // let mut bytes = bytes.clone();
        //
        // // Check that there is actually some data in the packet (minimum size of id field)
        // if bytes.len() < 10 {
        //     return Err(Error::TooSmallPacket);
        // }
        //
        // let mut fields = HashMap::new();
        //
        // // Loop until all da bytes are read
        // while !bytes.is_empty() {
        //     // Read the name of the field
        //     let name = read_str(2, &mut bytes);
        //
        //     // Read the field type
        //     let field_type = match cursor.read_u32::<LittleEndian>() {
        //         Ok(val) => val,
        //         Err(err) => return Err(Error::ErrorReadingName(err)),
        //     } as usize;
        //
        //     let size = match cursor.read_u32::<LittleEndian>() {
        //         Ok(val) => val,
        //         Err(err) => return Err(Error::ErrorReadingName(err)),
        //     } as usize;
        //
        //     let data = {
        //         let mut temp = Vec::with_capacity(size);
        //         match cursor.read_exact(temp.as_mut_slice()) {
        //             Ok(_) => {}
        //             Err(err) => return Err(Error::ErrorReadingContentNamed(name, err)),
        //         };
        //         temp
        //     };
        //
        //     fields.insert(name, data);
        // }

        todo!()
    }
}

pub enum FieldData {
    String(String),
    Array(Vec<FieldData>),
    KVPair(HashMap<String, FieldData>),
    Rgb([u8; 3]),
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    F32(f32),
    F64(f64),
}
