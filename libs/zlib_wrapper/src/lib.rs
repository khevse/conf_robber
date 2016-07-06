
#[macro_use]
extern crate log;
extern crate libc;
extern crate file_system;

use std::ptr;
use std::slice;

// Интерфейс C++ библиотеки
mod zlib {
    #[link(name = "zlibwrapper")]
    extern "C" {
        #[no_mangle]
        pub fn compress_data(source_data: *const u8,
                             source_data_size: u32,
                             data: &*mut u8,
                             size: *mut u32)
                             -> bool;
        #[no_mangle]
        pub fn decompress_data(source_data: *const u8,
                               source_data_size: u32,
                               data: &*mut u8,
                               size: *mut u32)
                               -> bool;
        #[no_mangle]
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
    let res: Vec<u8> = vec![0xCA, 0x48, 0xCD, 0xC9, 0xC9, 0x07];
    let test = compress(&data);

    assert_eq!(res, test);
}

#[test]
fn test_zlib_decompress() {

    let data: Vec<u8> = vec![0xCA, 0x48, 0xCD, 0xC9, 0xC9, 0x07];
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

#[test]
fn test_zlib_big_data() {
    use std::path::Path;

    let path_to_current_dir = file_system::get_current_dir()
        .ok()
        .expect("Failed read current directory.");
    let path_to_cf = Path::new(&path_to_current_dir)
                                .parent().unwrap() // libs
                                .parent().unwrap() // conf_robber
                                .join("test_data")
                                .join("original.cf");
    let path_to_cf = file_system::path_to_str(path_to_cf.as_path());

    let mut data = match file_system::read_file(&path_to_cf) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    while !data.is_empty() {

        let compress_data = compress(&data);
        let decompress_data = decompress(&compress_data);

        assert_eq!(decompress_data, data);

        let new_size: usize = data.len() -
                              match data.len() / 3 {
            0 => 1,
            _ => data.len() / 3,
        };
        println!("new_size: {}", new_size);
        data.truncate(new_size);
    }
}
