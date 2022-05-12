use crate::net::packets::error::{Error, ParsePacketResult};
use std::f64;
use std::io::{Cursor, Read};

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

#[cfg(test)]
mod test {
    use super::*;
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
}
