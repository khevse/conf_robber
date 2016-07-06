
#[macro_use]
extern crate log;
extern crate time;
extern crate regex;
extern crate aho_corasick;
#[macro_use]
extern crate lazy_static;

extern crate conv;
extern crate settings;
extern crate file_system;
extern crate zlib_wrapper;

#[macro_use]
mod meta_data;
mod structure;
mod configuration;

pub use configuration::CF;

pub static DEFAULT_BLOCK_SIZE: i32 = 512; // Размер блока данных по умолчанию
pub static GROUP_BLOKS_FLAG: [u8; 4] = [0xFF, 0xFF, 0xFF, 0x7F]; // маркер группы &conv::int32_to_bytes(i32::max_value())
