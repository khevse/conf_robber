
use log;
use fern;
use time;

use std::path::Path;
use utils;

// Инициализировать объект логирования
pub fn init_log(target_dir: &String, log_level: Option<&String>) {

    let log_dir = utils::fs::path_to_str(&Path::new(&target_dir).join("log"));
    utils::fs::create_dir(&*log_dir);

    let log_file = Path::new(&log_dir).join("main.log");

    // Create a basic logger configuration
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg, level, _location| {
            let t = time::now();
            format!("{},{:03} - {} - {}",
                    time::strftime("%Y-%m-%d %H:%M:%S", &t).unwrap(),
                    t.tm_nsec / 1000_000,
                    level,
                    msg)
        }),

        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file(&log_file)],

        level: match log_level {
            None => log::LogLevelFilter::Info,
            Some(v) => {
                match &**v {
                    "debug" => log::LogLevelFilter::Debug,
                    "info" => log::LogLevelFilter::Info,
                    "trace" => log::LogLevelFilter::Trace,
                    "warn" => log::LogLevelFilter::Warn,
                    "error" => log::LogLevelFilter::Error,
                    _ => {
                        panic!("Bad value of the logging level. True variants: debug, info, \
                                trace, warn, error.")
                    }
                }
            }
        },
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}.", e);
    }
}
