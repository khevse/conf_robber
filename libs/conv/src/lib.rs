
#[macro_use]
extern crate log;
extern crate byteorder;
extern crate encoding;

use byteorder::{ByteOrder, BigEndian, LittleEndian};
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::UTF_16LE;

pub fn utf16_to_utf8(value: &[u8]) -> Vec<u8> {

    let mut buf = String::new();
    match UTF_16LE.decode_to(value, DecoderTrap::Ignore, &mut buf) {
        Err(e) => {
            error!("Decode utf16 to utf8: {}", e);
            panic!("Decode utf16 to utf8: {}", e);
        }
        Ok(_) => (),
    }

    return buf.into_bytes();
}

pub fn utf8_to_utf16(value: &[u8]) -> Vec<u8> {

    let text = match String::from_utf8(value.to_vec()) {
        Ok(v) => v,
        Err(e) => {
            error!("utf8 to string: {}", e);
            panic!("utf8 to string: {}", e);
        }
    };

    let mut buf = Vec::new();
    match UTF_16LE.encode_to(&text as &str, EncoderTrap::Ignore, &mut buf) {
        Err(e) => {
            error!("Encode utf8 to utf16: {}", e);
            panic!("Encode utf8 to utf16: {}", e);
        }
        Ok(_) => (),
    }

    return buf;
}

pub fn int64_to_bytes(value: u64) -> [u8; 8] {
    let mut buf = [0u8; 8];
    LittleEndian::write_u64(&mut buf, value);
    return buf;
}

pub fn int32_to_bytes(value: i32) -> [u8; 4] {
    let mut buf = [0u8; 4];
    LittleEndian::write_i32(&mut buf, value);
    return buf;
}

pub fn int32_to_hex_bytes(value: i32) -> [u8; 8] {

    let mut temp_buf = [0u8; 4];
    BigEndian::write_i32(&mut temp_buf, value);

    let hex_str: Vec<String> = temp_buf.iter()
                                       .map(|b| format!("{:02x}", b))
                                       .collect();
    let mut buf: [u8; 8] = [0u8; 8];

    for i in 0..hex_str.len() {
        let bytes = hex_str[i].as_bytes();

        buf[i * 2] = bytes[0];
        buf[i * 2 + 1] = bytes[1];
    }

    return buf;
}

pub fn hex_to_int(bytes: &[u8]) -> i32 {

    let mut result: i32 = 0;

    if bytes.len() == 0 {
        return result;
    }

    for symbol in bytes {
        if *symbol >= b'0' && *symbol <= b'9' {
            result <<= 4;
            result += (*symbol - b'0') as i32;
        } else if *symbol >= b'a' && *symbol <= b'f' {
            result <<= 4;
            result += (*symbol - b'a' + 10) as i32;
        } else {
            break;
        }
    }

    return result;
}

pub fn bytes_to_int64(bytes: &[u8]) -> u64 {
    return LittleEndian::read_u64(&bytes);
}

pub fn bytes_to_int32(bytes: &[u8]) -> i32 {
    return LittleEndian::read_i32(&bytes);
}


#[cfg(test)]
mod tests {
    use {utf16_to_utf8, utf8_to_utf16, hex_to_int, int32_to_hex_bytes, int64_to_bytes,
         bytes_to_int64, bytes_to_int32};

    #[test]
    fn test_utf16_utf8() {
        let data_utf16 = [b'n', 0x00, b'a', 0x00, b'm', 0x00, b'e', 0x00];
        let data_utf8 = [b'n', b'a', b'm', b'e'];
        let test_utf8 = utf16_to_utf8(&data_utf16);
        let test_utf16 = utf8_to_utf16(&data_utf8);

        assert_eq!(&data_utf8, &test_utf8[..]);
        assert_eq!(&data_utf16, &test_utf16[..]);
    }


    #[test]
    fn test_hex_to_int() {
        let v: Vec<u8> = vec![b'0', b'0', b'0', b'0', b'0', b'0', b'a', b'c'];
        assert_eq!(172, hex_to_int(&v));
    }

    #[test]
    fn test_int32_to_bytes() {

        let data: Vec<u8> = vec![b'0', b'0', b'0', b'0', b'0', b'0', b'a', b'c'];
        let test = int32_to_hex_bytes(172);

        assert_eq!(data, test);
        assert_eq!(hex_to_int(&data), hex_to_int(&test));
    }

    #[test]
    fn test_int64_to_bytes() {
        assert_eq!([0xE4, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                   int64_to_bytes(740));
    }

    #[test]
    fn test_bytes_to_int64() {
        let data: Vec<u8> = vec![0x00, 0x93, 0x8E, 0x30, 0x23, 0x42, 0x02, 0x00];
        assert_eq!(635668859360000, bytes_to_int64(&data[0..8]));
    }

    #[test]
    fn test_bytes_to_int32() {
        let data: Vec<u8> = vec![0xE4, 0x02, 0x00, 0x00];
        assert_eq!(740, bytes_to_int32(&data[0..4]));
    }
}
