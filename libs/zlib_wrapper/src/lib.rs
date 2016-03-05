
#[macro_use]
extern crate log;
extern crate libc;

use std::ptr;
use std::slice;

// Интерфейс C++ библиотеки
mod zlib {
    #[link(name = "zlibwrapper")]
    extern "C" {
        pub fn compress_data(source_data: *const u8,
                             source_data_size: u32,
                             data: &*mut u8,
                             size: *mut u32)
                             -> bool;
        pub fn decompress_data(source_data: *const u8,
                               source_data_size: u32,
                               data: &*mut u8,
                               size: *mut u32)
                               -> bool;
        pub fn free_data(data: &*mut u8);
    }
}

// Сжать данные
pub fn compress(source_data: &Vec<u8>) -> Vec<u8> {

    let src: &[u8] = &source_data[..];
    let mut retval: Vec<u8> = Vec::new();

    unsafe {
        let data: *mut u8 = ptr::null_mut();
        let mut size: u32 = 0;

        if zlib::compress_data(src.as_ptr(), src.len() as u32, &data, &mut size) == true {
            retval.extend_from_slice(slice::from_raw_parts(data, size as usize));
            zlib::free_data(&data);
        } else {
            error!("Unable to compress the data: \n{:?}.", source_data);
            panic!("Unable to compress the data.");
        }
    }

    return retval;
}

// Распаковать данные
pub fn decompress(source_data: &Vec<u8>) -> Vec<u8> {

    let src: &[u8] = &source_data[..];
    let mut retval: Vec<u8> = Vec::new();

    unsafe {
        let data: *mut u8 = ptr::null_mut();
        let mut size: u32 = 0;

        if zlib::decompress_data(src.as_ptr(), src.len() as u32, &data, &mut size) == true {
            retval.extend_from_slice(slice::from_raw_parts(data, size as usize));
            zlib::free_data(&data);
        } else {
            error!("Unable to decompress the data: \n{:?}.", source_data);
            panic!("Unable to decompress the data.");
        }
    }

    return retval;
}

#[test]
fn test_zlib_compress() {

    let data: Vec<u8> = vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]; // hello
    let res: Vec<u8> = vec![0xCB, 0x48, 0xCD, 0xC9, 0xC9, 0x07, 0x00];
    let test = compress(&data);

    assert_eq!(res.len(), test.len());
    assert_eq!(res, test);
}

#[test]
fn test_zlib_decompress() {

    let data: Vec<u8> = vec![0xCB, 0x48, 0xCD, 0xC9, 0xC9, 0x07, 0x00];
    let res: Vec<u8> = vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]; // hello
    let test = decompress(&data);

    assert_eq!(res.len(), test.len());
    assert_eq!(res, test);
}

#[test]
fn test_zlib_compress_decompress() {

    let data: Vec<u8> = vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]; // hello
    let compress_data = compress(&data);
    let decompress_data = decompress(&compress_data);

    assert_eq!(data, decompress_data);
}
