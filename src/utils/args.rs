
use std::collections::HashMap;

const PACK: &'static str = "-P"; // Разобрать конфигурационный файл на блоки и записать их в файлы
const BUILD: &'static str = "-B"; // Создать файл конфигурации на основании раннее распакованной в файлы конфигурации
const CF: &'static str = "--cf"; // Путь к конфигурационному файлу
const DIR: &'static str = "--dir"; // Каталог
const TARGET: &'static str = "--target"; // Путь к каталогу в который будет помещен результат
const LOG_LEVEL: &'static str = "--log-level"; // Уровень логирования при выполнении операции

// Аргументы переданные в программу
pub struct Args {
    operation: String,
    params: HashMap<String, String>,
}

impl Args {
    // Инициализировать объекта на основании коллекции аргументов переданных в программу
    pub fn new(args: Vec<String>) -> Args {

        let mut params = HashMap::new();
        let mut operation = String::new();

        for arv in args {
            let values: Vec<&str> = arv.split('=').collect();

            match values.len() {
                1 => {
                    let key = String::from(*values.get(0).unwrap());

                    operation = match key.eq(PACK) || key.eq(BUILD) {
                        true => key,
                        _ => String::new(),
                    };
                }
                2 => {
                    let key = String::from(*values.get(0).unwrap());
                    let val = String::from(*values.get(1).unwrap());

                    if val.len() > 0 &&
                       (key.eq(CF) || key.eq(DIR) || key.eq(TARGET) || key.eq(LOG_LEVEL)) {
                        params.insert(key, val);
                    }
                }
                _ => {
                    error!("The value should not include the symbol '=' or be empty.");
                    panic!("The value should not include the symbol '=' or be empty.");
                }
            }
        }

        let retval = Args {
            operation: operation,
            params: params,
        };

        if retval.operation().eq(PACK) {
            if retval.cf() == None || retval.target() == None {
                panic!("{}", Args::desc_unpuck_params());
            }

        } else if retval.operation().eq(BUILD) {
            if retval.dir() == None || retval.target() == None {
                panic!("{}", Args::desc_build_params());
            }

        } else {
            let mut desc = String::new();
            desc.push_str(&*Args::desc_unpuck_params());
            desc.push_str("\n\n");
            desc.push_str(&*Args::desc_build_params());

            panic!("{}", desc);
        }

        return retval;
    }

    // Возвращает тип выполняемой операции
    pub fn operation(&self) -> String {
        return self.operation.clone();
    }

    // Возвращает путь к конфигурационному файлу
    pub fn cf(&self) -> Option<&String> {
        return self.params.get(CF);
    }

    // Возвращает каталог
    pub fn dir(&self) -> Option<&String> {
        return self.params.get(DIR);
    }

    // Возвращает путь к каталогу в котором будет результат выполнения
    pub fn target(&self) -> Option<&String> {
        return self.params.get(TARGET);
    }

    // Возвращает уровень логирования
    pub fn log_level(&self) -> Option<&String> {
        return self.params.get(LOG_LEVEL);
    }

    // Возвращает справку для выполнения операции по распаковке конфигурационного файла
    fn desc_unpuck_params() -> String {

        let mut desc = String::new();

        desc.push_str("Operation type: unpack the configuration file (*.cf)\n");
        desc.push_str("Options:\n");
        desc.push_str(PACK);
        desc.push_str(" - operation type\n");
        desc.push_str(CF);
        desc.push_str("=Path to the source file *.cf\n");
        desc.push_str(TARGET);
        desc.push_str("=Path to the target directory\n");
        desc.push_str(LOG_LEVEL);
        desc.push_str("=Log level (optional)\n");

        return desc;
    }

    // Возвращает справку для выполнения операции по сборке конфигурационного файла
    fn desc_build_params() -> String {

        let mut desc = String::new();

        desc.push_str("Operation type: build the configuration file (*.cf)\n");
        desc.push_str("Options:\n");
        desc.push_str(BUILD);
        desc.push_str(" - operation type\n");
        desc.push_str(DIR);
        desc.push_str("=Path to the directory with the source files\n");
        desc.push_str(TARGET);
        desc.push_str("=Path to the configuration file (*.cf)\n");
        desc.push_str(LOG_LEVEL);
        desc.push_str("=Log level (optional)\n");

        return desc;
    }
}
