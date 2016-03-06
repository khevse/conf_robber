
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
        "-P" => unpack_to_dir(args.cf().unwrap(), args.target().unwrap(), args.log_level()),
        "-B" => {
            build_cf(args.dir().unwrap(),
                     args.target().unwrap(),
                     args.log_level())
        }
        _ => panic!("Failed parameters."),
    }
}

// Распаковать конфигурационный файл в каталог
fn unpack_to_dir(path_to_cf: &String, path_to_target_dir: &String, log_level: Option<&String>) {

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
            cf.deflate(&unpack_dir);
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
