
#[macro_use]
extern crate log;

use std::error::Error;
use std::fs::{self, File, create_dir_all, metadata};
use std::io::{Read, BufWriter, Write};
use std::path::Path;
use std::env::{current_dir, split_paths};
use std::collections::HashMap;

// Прочитать данные файла в буфер
pub fn read_file(path: &str) -> Result<Vec<u8>, String> {

    // Open the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => {
            return Err(format!("Couldn't open: {}", Error::description(&why)));
        }
        Ok(file) => file,
    };

    let mut buffer: Vec<u8> = Vec::new();

    match file.read_to_end(&mut buffer) {
        Err(why) => Err(format!("Couldn't read: {}", Error::description(&why))),
        Ok(_) => Ok(buffer),
    }
}

// Записать данные файл. Если файл существует, то данные будут дописаны
pub fn write_file(path: &str, data: &Vec<u8>) -> Result<(), String> {

    let file = match File::create(path) {
        Err(e) => {
            return Err(format!("Failed create the file '{}' - {}.",
                               path,
                               Error::description(&e)))
        }
        Ok(v) => v,
    };

    let mut writer = BufWriter::new(&file);

    return match writer.write_all(&data[..]) {
        Err(e) => {
            Err(format!("Failed write to the file '{}': {}",
                        path,
                        Error::description(&e)))
        }
        Ok(_) => Ok(()),
    };
}

// Возвращает коллекцию с путями к файлам, которые находятся внутри каталога
pub fn files_in_dir(path_to_dir: &String) -> HashMap<String, String> {

    let mut retval: HashMap<String, String> = HashMap::new();

    if is_dir(path_to_dir) {
        let dir = match fs::read_dir(path_to_dir) {
            Ok(v) => v,
            Err(e) => {
                error!("Error read path: {}.", e);
                panic!("Error read path: {}.", e);
            }
        };

        for entry in dir {
            let entry = entry.ok().expect("Failed get entry.");
            let path = entry.path();

            let file_name: String = match path.file_name() {
                Some(v) => {
                    match v.to_str() {
                        Some(v) => String::from(v),
                        None => {
                            error!("Failed convertation name of the file to the string.");
                            panic!("Failed convertation name of the file to the string.");
                        }
                    }
                }
                None => {
                    error!("File name is empty.");
                    panic!("File name is empty.");
                }
            };

            let path = match entry.path().to_str() {
                None => {
                    error!("Error convert path to the string.");
                    panic!("Error convert path to the string.");
                }
                Some(v) => v.to_string(),
            };

            retval.insert(file_name, path);
        }
    }

    return retval;
}

// Выполняет проверку что путь указывает на каталог
pub fn is_dir(path: &String) -> bool {

    return match fs::metadata(path) {
        Err(v) => {
            error!("Error type of file check(is directory): {} - {}", path, v);
            panic!("Error type of file check(is directory): {} - {}", path, v);
        }
        Ok(v) => v.is_dir(),
    };
}

// Файл существует
pub fn exist(path: &String) -> bool {
    return Path::new(path).exists();
}

pub fn remove(path: &String) -> Result<(), String> {

    return match is_dir(path) {
        true => {
            match fs::remove_dir_all(path) {
                Ok(_) => Ok(()),
                Err(e) => {
                    Err(format!("Failed remove the file '{}' - {}.",
                                path,
                                Error::description(&e)))
                }
            }
        }
        false => {
            match fs::remove_file(path) {
                Ok(_) => Ok(()),
                Err(e) => {
                    Err(format!("Failed remove the file '{}' - {}.",
                                path,
                                Error::description(&e)))
                }
            }
        }

    };
}

// Возвращает имя файла
pub fn file_name(path: &String) -> String {

    return match Path::new(&path).file_name() {
        Some(v) => {
            match v.to_str() {
                Some(v) => v.to_string(),
                None => {
                    error!("Error converting name of the file to the string: {}", path);
                    panic!("Error converting name of the file to the string: {}", path);
                }
            }
        }
        None => {
            error!("Failed read name of the file: {}", path);
            panic!("Failed read name of the file: {}", path);
        }
    };
}

// Преобразует путь к строке
pub fn path_to_str(path: &Path) -> String {
    return match path.to_str() {
        None => {
            error!("Failed convert path file to the string.");
            panic!("Failed convert path file to the string.");
        }
        Some(path_str) => path_str.to_string(),
    };
}

// Создать каталог
pub fn create_dir(path: &str) {

    let path_to_dir = path.to_string();
    let exist = metadata(&path_to_dir).is_ok();

    if exist == false {
        match create_dir_all(&path_to_dir) {
            Err(e) => {
                error!("Failed create target directory: {}.", e);
                panic!("Failed create target directory: {}.", e);
            }
            Ok(_) => (),
        };
    }
}

// Возвращает путь к каталогу в котором находится исполняемый файл
pub fn get_current_dir() -> Result<String, String> {

    match current_dir() {
        Ok(exe_path) => {
            for path in split_paths(&exe_path) {
                return Ok(path_to_str(path.as_path()));
            }
        }
        Err(e) => {
            return Err(format!("Failed to get current directory: {}", e));
        }
    };

    return Err(String::from("Failed to get current directory"));
}

#[cfg(test)]
mod tests {
    use {read_file, path_to_str, get_current_dir, exist};
    use std::path::Path;

    #[test]
    fn test_read_file() {
        let path_to_current_dir = get_current_dir().ok().expect("Failed read current directory.");
        let path_to_original_cf = Path::new(&path_to_current_dir)
                                    .parent().unwrap() // libs
                                    .parent().unwrap() // conf_robber
                                    .join("test_data")
                                    .join("original.cf");

        let path_to_original_cf = path_to_str(path_to_original_cf.as_path());

        assert_eq!(true, exist(&path_to_original_cf));

        let buffer = match read_file(&path_to_original_cf) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(36851, buffer.len());
    }
}
