use crate::net::packets::error::{Error, ParsePacketResult};
use crate::net::packets::{error, FieldData};
use std::collections::HashMap;
use std::f64;
use std::io::{Cursor, Read};
use tracing::debug;

const SIZED_TYPES: [bool; 17] = [
    true, true, true, true, true, true, true, true, true, true, true, true, true, false, false,
    false, false,
];

/// Reads the next `n` bytes from `buf`
fn read_n(n: u32, buf: &mut Vec<u8>) -> ParsePacketResult<Vec<u8>> {
    // Read the next n bytes
    let mut bytes = buf
        .iter()
        .take(n as usize)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    // Check the correct number of bytes were read
    if bytes.len() != n as usize {
        return Err(Error::TooSmallPacket);
    }

    buf.drain(0..n as usize);

    Ok(bytes)
}

/// Reads the next byte of `buf` as a bool
#[inline(always)]
pub fn read_bool(buf: &mut Vec<u8>) -> ParsePacketResult<bool> {
    // Read next byte
    let bytes = read_n(1, buf)?;

    Ok(bytes[0] != 0)
}

/// Reads the next byte of `buf` as a u8
#[inline(always)]
pub fn read_u8(buf: &mut Vec<u8>) -> ParsePacketResult<u8> {
    // Read next byte
    let bytes = read_n(1, buf)?;

    Ok(bytes[0])
}

/// Reads the next 2 bytes of `buf` as a i8
#[inline(always)]
pub fn read_i8(buf: &mut Vec<u8>) -> ParsePacketResult<i8> {
    // Read next 2 bytes
    let bytes = read_n(1, buf)?;

    Ok(i8::from_le_bytes([bytes[0]]))
}

/// Reads the next 2 bytes of `buf` as a u16
#[inline(always)]
pub fn read_u16(buf: &mut Vec<u8>) -> ParsePacketResult<u16> {
    // Read next 2 bytes
    let bytes = read_n(2, buf)?;

    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

/// Reads the next 2 bytes of `buf` as a i16
#[inline(always)]
pub fn read_i16(buf: &mut Vec<u8>) -> ParsePacketResult<i16> {
    // Read next 2 bytes
    let bytes = read_n(2, buf)?;

    Ok(i16::from_le_bytes([bytes[0], bytes[1]]))
}

/// Reads the next 4 bytes of `buf` as a u32
#[inline(always)]
pub fn read_u32(buf: &mut Vec<u8>) -> ParsePacketResult<u32> {
    // Read next 4 bytes
    let bytes = read_n(4, buf)?;

    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// Reads the next 4 bytes of `buf` as a i32
#[inline(always)]
pub fn read_i32(buf: &mut Vec<u8>) -> ParsePacketResult<i32> {
    // Read next 4 bytes
    let bytes = read_n(4, buf)?;

    Ok(i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// Reads the next 8 bytes of `buf` as a u64
#[inline(always)]
pub fn read_u64(buf: &mut Vec<u8>) -> ParsePacketResult<u64> {
    // Read next 8 bytes
    let bytes = read_n(8, buf)?;

    Ok(u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

/// Reads the next 8 bytes of `buf` as a i64
#[inline(always)]
pub fn read_i64(buf: &mut Vec<u8>) -> ParsePacketResult<i64> {
    // Read next 8 bytes
    let bytes = read_n(8, buf)?;

    Ok(i64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

/// Reads the next 16 bytes of `buf` as a u128
#[inline(always)]
pub fn read_u128(buf: &mut Vec<u8>) -> ParsePacketResult<u128> {
    // Read next 16 bytes
    let bytes = read_n(16, buf)?;

    Ok(u128::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
        bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    ]))
}

/// Reads the next 16 bytes of `buf` as a i128
#[inline(always)]
pub fn read_i128(buf: &mut Vec<u8>) -> ParsePacketResult<i128> {
    // Read next 16 bytes
    let bytes = read_n(4, buf)?;

    Ok(i128::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], bytes[8],
        bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    ]))
}

/// Reads the next 4 bytes of `buf` as a u32
#[inline(always)]
pub fn read_f32(buf: &mut Vec<u8>) -> ParsePacketResult<f32> {
    // Read next 4 bytes
    let bytes = read_n(4, buf)?;

    Ok(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

/// Reads the next 8 bytes of `buf` as a u64
#[inline(always)]
pub fn read_f64(buf: &mut Vec<u8>) -> ParsePacketResult<f64> {
    // Read next 8 bytes
    let bytes = read_n(8, buf)?;

    Ok(f64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

/// Reads the next `n` bytes of `buf` as a str
#[inline(always)]
pub fn read_str(n: u32, buf: &mut Vec<u8>) -> ParsePacketResult<String> {
    Ok(String::from_utf8(read_n(n, buf)?)?)
}

/// Reads the next `n` bytes of `buf` as a vec of rgb values
pub fn read_rgb(n: u32, buf: &mut Vec<u8>) -> ParsePacketResult<Vec<(u8, u8, u8)>> {
    // Read next n bytes
    let mut bytes = read_n(n, buf)?;

    let mut cursor = Cursor::new(bytes);

    // Use flate2 to decompress the bytes
    let mut decompressor = flate2::read::GzDecoder::new(&mut cursor);

    let mut decompressed_bytes = Vec::new();

    // Read decompressed values into vec
    decompressor
        .read_to_end(&mut decompressed_bytes)
        .map_err(Error::UnableToDecompress)?;

    let mut values = vec![];

    // Convert bytes to vec of tupples
    while !decompressed_bytes.is_empty() {
        let bytes = read_n(3, &mut decompressed_bytes)?;
        values.push((bytes[0], bytes[1], bytes[2]));
    }

    Ok(values)
}

/// Read next dynamic array of bytes
pub fn read_arr_dyn(n: u32, buf: &mut Vec<u8>) -> ParsePacketResult<Vec<Vec<u8>>> {
    let mut bytes = read_n(n, buf)?;
    let mut array = vec![];

    while !bytes.is_empty() {
        let size = read_u32(&mut bytes)?;
        let data = read_n(size, &mut bytes)?;
        array.push(data);
    }

    Ok(array)
}

/// Read next `n` bytes of buffer as array with element size `n`
pub fn read_arr_fixed(n: u32, sz: u32, buf: &mut Vec<u8>) -> ParsePacketResult<Vec<Vec<u8>>> {
    let mut bytes = read_n(n, buf)?;
    let mut array = vec![];

    while !bytes.is_empty() {
        let data = read_n(sz, &mut bytes)?;
        array.push(data);
    }

    Ok(array)
}

pub fn read_kv_pair(n: u32, buf: &mut Vec<u8>) -> ParsePacketResult<HashMap<String, FieldData>> {
    let mut bytes = read_n(n, buf)?;
    while !bytes.is_empty() {
        let name = read_str(2, &mut bytes)?;

        let field_type = error::name(&name, read_u16(&mut bytes))?;
        let field_data = match field_type {
            0 => FieldData::Bool(error::name(&name, read_bool(&mut bytes))?),
            1 => FieldData::U8(error::name(&name, read_n(1, &mut bytes))?[0]),
            2 => FieldData::I8(error::name(&name, read_i8(&mut bytes))?),
            3 => FieldData::U16(error::name(&name, read_u16(&mut bytes))?),
            4 => FieldData::I16(error::name(&name, read_i16(&mut bytes))?),
            5 => FieldData::U32(error::name(&name, read_u32(&mut bytes))?),
            6 => FieldData::I32(error::name(&name, read_i32(&mut bytes))?),
            7 => FieldData::U64(error::name(&name, read_u64(&mut bytes))?),
            8 => FieldData::I64(error::name(&name, read_i64(&mut bytes))?),
            9 => FieldData::U128(error::name(&name, read_u128(&mut bytes))?),
            10 => FieldData::I128(error::name(&name, read_i128(&mut bytes))?),
            11 => FieldData::F32(error::name(&name, read_f32(&mut bytes))?),
            12 => FieldData::F64(error::name(&name, read_f64(&mut bytes))?),
            13 => {
                let size = error::name(&name, read_u32(&mut bytes))?;
                FieldData::String(error::name(&name, read_str(size, &mut bytes))?)
            }
            // - `14` - KV Pair
            // - `15` - Array
            // 16 => {
            //     let size = error::name(&name, read_u32(&mut bytes))?;
            //     FieldData::Rgb(error::name(&name, read_rgb(size, &mut bytes))?)
            // }
            _ => return Err(Error::InvalidFieldType(name, field_type)),
        };
    }

    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    #[test]
    fn test_read_n() {
        let mut buf = vec![0, 1, 2, 3, 4, 5];
        let bytes = read_n(3, &mut buf).unwrap();

        assert_eq!(vec![0, 1, 2], bytes);
        assert_eq!(vec![3, 4, 5], buf);
    }

    #[test]
    fn test_read_str() {
        let mut buf = String::from("Hello world").as_bytes().to_vec();
        let str = read_str(5, &mut buf).unwrap();
        assert_eq!(String::from("Hello"), str);
        assert_eq!(String::from(" world").as_bytes().to_vec(), buf);
    }

    #[test]
    fn test_read_bool() {
        let mut buf: Vec<u8> = vec![1, 0];
        let val_true = read_bool(&mut buf).unwrap();

        assert_eq!(vec![0], buf);
        assert_eq!(true, val_true);

        let val_false = read_bool(&mut buf).unwrap();
        assert_eq!(Vec::<u8>::new(), buf);
        assert_eq!(false, val_false);
    }

    #[test]
    fn test_read_dyn_arr() {
        let mut buf = vec![2, 0, 0, 0, 1, 2, 3, 0, 0, 0, 1, 2, 3, 6];
        let val = read_arr_dyn(13, &mut buf).unwrap();

        assert_eq!(vec![6], buf);
        assert_eq!(vec![vec![1, 2], vec![1, 2, 3]], val);
    }

    #[test]
    fn test_read_fixed_arr() {
        let mut buf = vec![12, 3, 6, 2];
        let val = read_arr_fixed(4, 2, &mut buf).unwrap();

        assert_eq!(Vec::<u8>::new(), buf);
        assert_eq!(vec![vec![12, 3], vec![6, 2]], val);
    }
}
