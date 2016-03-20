
#[macro_use]
extern crate log;
#[macro_use]
extern crate logger;
extern crate time;
extern crate file_system;

#[cfg(test)]
extern crate settings;

use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

mod connection_settings;
use connection_settings::ConnectionSettings;

/// Режимы запуска в пакетном режиме

pub const MODE_DESIGNER: &'static str = "DESIGNER"; // Режим конфигуратора
pub const MODE_ENTERPRISE: &'static str = "ENTERPRISE"; // Режим предприятия
pub const MODE_CREATEINFOBASE: &'static str = "CREATEINFOBASE"; // Режим создания новой ИБ

/// Объект для работы с информационной базой 1С в пакетном режиме
pub struct InformationBaseV8 {
    connections_settings: ConnectionSettings,
}

impl InformationBaseV8 {
    pub fn new(connections_settings: &HashMap<String, String>) -> InformationBaseV8 {
        return InformationBaseV8 {
            connections_settings: ConnectionSettings::from_map(connections_settings),
        };
    }

    /// Загрузить конфигурацию в информационную базу
    pub fn load_cfg(&self, path_to_config_file: &String) -> Result<i32, String> {

        info!("Load cf: {}", path_to_config_file);

        if !file_system::exist(path_to_config_file) {
            error!("File not found.");
            panic!("File not found.");
        }

        let operation = String::from("/LoadCfg");
        let args = vec![&operation, path_to_config_file];
        let retval = self.run_application(MODE_DESIGNER, args);

        info!("-Load cf: {}", retval.is_ok());

        return retval;
    }

    /// Обновить конфигурацию информационной базы
    pub fn update_cfg(&self) -> Result<i32, String> {

        info!("Update information base.");

        let operation = String::from("/UpdateDBCfg");
        let args = vec![&operation];
        let retval = self.run_application(MODE_DESIGNER, args);

        info!("-Update information base: {}.", retval.is_ok());

        return retval;
    }

    /// Выгрузить конфигурацию информационной базы в файл
    pub fn dump_cfg(&self, path_to_file: &String) -> Result<i32, String> {

        info!("Unload in the file of the configuration: {}.", path_to_file);

        if file_system::exist(path_to_file) {
            match file_system::remove(path_to_file) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                    panic!("{}", e);
                }
            }
        }

        let operation = String::from("/DumpCfg");
        let args = vec![&operation, path_to_file];
        let retval = self.run_application(MODE_DESIGNER, args);

        info!("-Unload in the file of the configuration: {}.",
              retval.is_ok());

        return retval;
    }

    /// Создать новую информационную базу
    pub fn create(&self, path_to_template: &String) -> Result<i32, String> {

        info!("Create new information base. Template: {}",
              path_to_template);

        if !path_to_template.is_empty() && !file_system::exist(path_to_template) {
            error!("Template not found.");
            panic!("Template not found.");
        }

        if file_system::exist(self.connections_settings.path()) {
            match file_system::remove(self.connections_settings.path()) {
                Ok(_) => (),
                Err(e) => {
                    error!("{}", e);
                    panic!("{}", e);
                }
            }
        }

        // 1С предлагает для создания новой конфигурации на основе существующего
        // cf файла использование параметра /UseTemplate, но по какой-то причине
        // на некоторых версия платформы он работает не корректно.
        // Поэтому делаем в два этапа:
        // 1. создаем новую конфигурацию
        // 2. загружаем в нее нужный cf файл
        // 3. обновляем конфигурацию информационной базы

        let mut retval = self.run_application(MODE_CREATEINFOBASE, vec![]);

        if retval.is_ok() && !path_to_template.is_empty() {
            retval = self.load_cfg(path_to_template);
        }

        if retval.is_ok() {
            retval = self.update_cfg();
        }

        info!("-Create new information base: {}", retval.is_ok());

        return retval;
    }

    /// Запустить приложение в пакетном режиме
    fn run_application(&self, mode: &str, args: Vec<&String>) -> Result<i32, String> {

        let mut full_args: Vec<String> = vec![String::from(mode)];
        self.add_access_string(mode, &mut full_args);
        for a in &args {

            if !a.is_empty() {
                full_args.push((**a).clone());
            }
        }

        self.add_path_to_log(mode, &mut full_args);

        let output = Command::new(self.connections_settings.platform())
                         .args(&full_args)
                         .output()
                         .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        info!("command: {} {:?}",
              self.connections_settings.platform(),
              full_args);
        info!("status: {}", output.status);
        info!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        info!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        return match output.status.success() {
            true => {
                let code = match output.status.code() {
                    Some(v) => v,
                    None => 0,
                };
                return Ok(code);
            }
            false => {
                return Err(String::from_utf8_lossy(&output.stderr).into_owned());
            }
        };
    }

    /// Добавить в параметры вызова приложения 1cv8.exe аргументы с параметрами доступа к информационной базе в пакетном режиме
    fn add_access_string(&self, mode: &str, args: &mut Vec<String>) {

        if mode == MODE_DESIGNER || mode == MODE_ENTERPRISE {
            // Для серверного режима вместо '/F' нужно указывать '/S'
            args.push(String::from("/F"));
            args.push(self.connections_settings.path().clone());

            if !self.connections_settings.user_name().is_empty() {
                args.push(String::from("/N"));
                args.push(self.connections_settings.user_name().clone());
            }

            if !self.connections_settings.user_pwd().is_empty() {
                args.push(String::from("/P"));
                args.push(self.connections_settings.user_pwd().clone());
            }

        } else if mode == MODE_CREATEINFOBASE {
            args.push(format!(r"File='{}'", self.connections_settings.path()));
        }

        if mode == MODE_DESIGNER && !self.connections_settings.storage_path().is_empty() {
            args.push(String::from("/ConfigurationRepositoryF"));
            args.push(self.connections_settings.storage_path().clone());

            if !self.connections_settings.storage_user_name().is_empty() {
                args.push(String::from("/ConfigurationRepositoryN"));
                args.push(self.connections_settings.storage_user_name().clone());
            }

            if !self.connections_settings.storage_user_pwd().is_empty() {
                args.push(String::from("/ConfigurationRepositoryP"));
                args.push(self.connections_settings.storage_user_pwd().clone());
            }
        }
    }

    /// Добавить путь к файлу логирования в аргументы запуска приложения
    fn add_path_to_log(&self, mode: &str, args: &mut Vec<String>) {

        let mut operation_name = String::new();

        if mode == MODE_DESIGNER {
            for a in &*args {

                if a.starts_with("/") && a.len() > 2 {
                    // кроме праметров: "/F", "/P", "/N" и т.п.
                    let parts: Vec<&str> = a.split('/').collect();
                    operation_name = String::from(*parts.get(1).unwrap()); // "/DumpCfg" => "DumpCfg"
                    break;
                }
            }
        } else if mode == MODE_CREATEINFOBASE {
            operation_name = String::from("Create");
        } else if mode == MODE_ENTERPRISE {
            operation_name = String::from("enterprise");
        }

        let t = time::now();
        let file_name = format!("{} - {}.log",
                                time::strftime("%Y-%m-%d %H-%M-%S", &t).unwrap(),
                                operation_name);
        let path = Path::new(&logger::get_log_directory()).join(file_name);
        let path = file_system::path_to_str(&path.as_path());

        args.push(String::from("/Out"));
        args.push(path);
    }
}

#[cfg(test)]
mod tests {
    extern crate logger;
    extern crate settings;
    extern crate file_system;

    use super::InformationBaseV8;
    use std::path::Path;

    #[test]
    fn test_information_base() {
        let target_dir = get_target_dir();
        logger::init_log(&target_dir, Some(&String::from("trace")));
        clear_log_dir();

        let mut settings = create_settings().source_ib_connection_settings().unwrap();
        settings.insert(String::from("path"), get_path_to_new_ib());

        let ib = InformationBaseV8::new(&settings);
        ib.create(&get_path_to_cf_template());

        let path_to_temp_cf = Path::new(&get_target_dir()).join("temp.cf");
        let path_to_temp_cf = file_system::path_to_str(path_to_temp_cf.as_path());
        ib.dump_cfg(&path_to_temp_cf);

        assert!(file_system::exist(&path_to_temp_cf));
    }

    fn create_settings() -> settings::Settings {
        let path_to_pom1c_xml = Path::new(&get_test_data_dir()).join("pom1c.xml");
        let path_to_pom1c_xml = file_system::path_to_str(path_to_pom1c_xml.as_path());

        let pom1c_xml = match file_system::read_file(&path_to_pom1c_xml) {
            Ok(v) => String::from_utf8(v).unwrap(),
            Err(e) => panic!("{}", e),
        };

        return settings::Settings::new(&pom1c_xml);
    }

    fn get_target_dir() -> String {

        let path_to_current_dir = file_system::get_current_dir()
                                      .ok()
                                      .expect("Failed read current directory.");
        let target_dir = Path::new(&path_to_current_dir)
                             .join("target")
                             .join("debug");
        return file_system::path_to_str(target_dir.as_path());
    }

    fn get_test_data_dir() -> String {

        let path_to_current_dir = file_system::get_current_dir()
                                      .ok()
                                      .expect("Failed read current directory.");
        let path = Path::new(&path_to_current_dir)
                                    .parent().unwrap() // libs
                                    .parent().unwrap() // conf_robber
                                    .join("test_data");

        return file_system::path_to_str(path.as_path());
    }

    fn get_path_to_cf_template() -> String {

        let path = Path::new(&get_test_data_dir()).join("original.cf");
        return file_system::path_to_str(path.as_path());
    }

    fn get_path_to_new_ib() -> String {

        let path = Path::new(&get_target_dir()).join("ib");
        return file_system::path_to_str(path.as_path());
    }

    fn clear_log_dir() {
        let log_dir = logger::get_log_directory();
        for (file_name, path) in file_system::files_in_dir(&log_dir) {

            if !file_name.eq("main.log") {
                file_system::remove(&path);
            }
        }
    }
}
