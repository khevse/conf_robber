
#[macro_use]
extern crate log;
extern crate logger;
extern crate conf_v8;
extern crate file_system;

mod utils;

use std::env;
use std::path::Path;

fn main() {

    let args = utils::args::Args::new(env::args().collect());
    println!("Operation type={}", args.operation());

    match &*args.operation() {
        "-P" => {
            unpack_to_dir(args.cf().unwrap(),
                          args.target().unwrap(),
                          args.log_level(),
                          args.settings())
        }
        "-B" => {
            build_cf(args.dir().unwrap(),
                     args.target().unwrap(),
                     args.log_level())
        }
        "-F" => {
            format_text(args.dir().unwrap(),
                        args.target().unwrap(),
                        args.log_level())
        }
        _ => panic!("Failed parameters."),
    }
}

// Распаковать конфигурационный файл в каталог
fn unpack_to_dir(path_to_cf: &String,
                 path_to_target_dir: &String,
                 log_level: Option<&String>,
                 settings: Option<&String>) {

    logger::init_log(&path_to_target_dir, log_level);

    info!("Path to the configuration file:{}", path_to_cf);
    info!("Begin");

    let unpack_dir = file_system::path_to_str(&Path::new(&path_to_target_dir).join("unpack"));
    file_system::create_dir(&*unpack_dir);

    match file_system::read_file(&*path_to_cf) {
        Err(v) => {
            error!("{}", v);
            panic!("{}", v)
        }
        Ok(data) => {
            let mut cf = conf_v8::CF::from_cf(&data);
            if settings.is_some() {
                let settings = settings.unwrap();
                let settings = match file_system::read_file(&settings) {
                    Ok(v) => String::from_utf8(v).unwrap(),
                    Err(e) => panic!("{}", e),
                };

                cf.filter(&settings);
            }

            cf.deflate_to_files(&unpack_dir);
        }
    }

    info!("End");
}

// Упаковать данные каталога в конфигурационный файл
fn build_cf(path_to_dir: &String, path_to_target_dir: &String, log_level: Option<&String>) {

    logger::init_log(&path_to_target_dir, log_level);

    info!("Path to the directory:{}", path_to_dir);
    info!("Begin");

    let file_name = file_system::path_to_str(&Path::new(&path_to_target_dir)
        .join("configuration.cf"));

    let cf = conf_v8::CF::from_file(&path_to_dir);
    match file_system::write_file(&*file_name, &cf.for_cf()) {
        Ok(_) => (),
        Err(e) => {
            error!("Error writing file of the result: {}", e);
            panic!("Error writing file of the result: {}", e);
        }
    }

    info!("End");
}

// Выполнить форматирование текста распакованных блоков конфигурации
fn format_text(path_to_dir: &String, path_to_target_dir: &String, log_level: Option<&String>) {

    logger::init_log(&path_to_target_dir, log_level);

    info!("Path to the directory:{}", path_to_dir);
    info!("Begin");

    dir_iterator(path_to_dir);

    info!("End");
}

fn dir_iterator(dir: &String) {

    let add_tabs = |count: &usize, vec: &mut Vec<u8>, pos: &mut usize| {
        let count_spaces = *count * 4;
        for _ in 0..count_spaces {
            vec.insert(*pos, b' ');
        }
        *pos += count_spaces;
    };

    for (_, path) in &file_system::files_in_dir(dir) {

        if file_system::is_dir(path) {
            dir_iterator(path);
        } else {
            let mut text = match file_system::read_file(path) {
                Ok(v) => v,
                Err(e) => {
                    error!("{}", e);
                    panic!("{}", e);
                }
            };

            let mut pos = 0;
            let mut prev: u8 = 0;
            let mut tabs: usize = 0;

            while pos < text.len() {
                let s = *text.get(pos).unwrap();
                match s {
                    b'{' => {
                        tabs += 1;
                        if prev.ne(&b'\n') {
                            text.insert(pos, b'\n');
                            pos += 1;
                        }
                        add_tabs(&tabs, &mut text, &mut pos);
                        prev = b'{';
                    }
                    b'}' => {
                        let next = match text.get(pos + 1) {
                            None => 0x00,
                            Some(v) => *v,
                        };
                        if next.ne(&b'\r') {
                            pos += 1;
                            text.insert(pos, b'\n');
                            prev = b'\n';
                            pos += 1;
                        }
                        tabs -= 1;
                    }
                    _ => (),
                }

                if prev.eq(&b'\n') {
                    let tmp_tabs = tabs + 1;
                    add_tabs(&tmp_tabs, &mut text, &mut pos);
                }

                prev = s;
                pos += 1;
            }

            let _ = file_system::write_file(path, &text);
        }
    }
}
