//! ```
//! extern crate logger;
//!
//! // 1. Create directory of logging in the directory.
//! // 2. Create the log with name "main.log"
//! let target_dir = String::from("c:\\temp"); // path to the directory where need create directory with logging files
//! let log_level  = String::from("debug"); //  Level logging: debug, trace, error, info, warn
//! logger::init_log(&target_dir, Some(&log_level));
//!
//! // After that you can write this is instructions
//! // error!("text"); // 2016-03-05 13:41:25,853 - ERROR - text
//! // info!("text");  // 2016-03-05 13:41:25,853 - INFO - text
//! // warn!("text");  // 2016-03-05 13:41:25,853 - WARN - text
//! // trace!("text"); // 2016-03-05 13:41:25,853 - TRACE - text
//! // debug!("text"); // 2016-03-05 13:41:25,853 - DEBUG - text
//!
//! ```

#[macro_use]
extern crate log;
#[macro_use]
extern crate fern;
extern crate time;
extern crate file_system;

use std::ptr;
use std::mem;
use std::path::Path;
use std::fs::File;

/// Settings of the logger
struct LogSettings {
    directory: String,
}

impl LogSettings {
    /// Create new object of the settings
    fn new(directory: &String) -> LogSettings {
        return LogSettings { directory: directory.clone() };
    }

    /// Return the directory with data of logging
    fn directory(&self) -> String {
        return self.directory.clone();
    }
}

static mut LOG_SETTINGS: *const LogSettings = 0 as *const LogSettings;

/// Set settings of logging
unsafe fn set_log_settings(directory: &String) {
    LOG_SETTINGS = mem::transmute(Box::new(LogSettings::new(directory)));
}

// Return settings of logging
unsafe fn get_log_settings<'a>() -> &'a mut LogSettings {
    if LOG_SETTINGS == ptr::null::<LogSettings>() {
        set_log_settings(&String::new());
    }
    return mem::transmute(LOG_SETTINGS);
}

/// Return the directory with data of logging
pub fn get_log_directory() -> String {
    return unsafe { get_log_settings().directory() };
}

/// Initialize the logger
pub fn init_log(target_dir: &String, log_level: Option<&String>) {

    let log_dir = file_system::path_to_str(&Path::new(&target_dir).join("log"));
    file_system::create_dir(&*log_dir);

    let log_file = Path::new(&log_dir).join("main.log");

    unsafe {
        set_log_settings(&log_dir);
    }

    // Cleaninig file if he exist
    match File::create(&*log_file) {
        Ok(v) => {
            match v.set_len(0) {
                Ok(_) => (),
                Err(e) => panic!("Failed cleaning: {}", e),
            }
        }
        Err(_) => (),
    }

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


#[cfg(test)]
mod tests {
    extern crate file_system;
    extern crate regex;

    use super::{init_log, get_log_directory};
    use std::path::Path;

    #[test]
    fn test_log() {

        let path_to_current_dir = file_system::get_current_dir()
                                      .ok()
                                      .expect("Failed read current directory.");
        let target_dir = Path::new(&path_to_current_dir)
                             .join("target")
                             .join("debug");
        let target_dir = file_system::path_to_str(target_dir.as_path());

        assert_eq!("", get_log_directory());

        init_log(&target_dir, Some(&String::from("trace")));

        let log_dir = Path::new(&target_dir).join("log");
        let log_dir = file_system::path_to_str(log_dir.as_path());

        assert_eq!(log_dir, get_log_directory());

        error!("text");
        trace!("text");
        debug!("text");
        warn!("text");
        info!("text");

        let files_in_target_dir = file_system::files_in_dir(&target_dir);
        let path_to_log_dir = files_in_target_dir.get(&String::from("log"));
        assert!(path_to_log_dir.is_some());

        let files_in_log_dir = file_system::files_in_dir(&path_to_log_dir.unwrap());
        let path_to_main_log = files_in_log_dir.get(&String::from("main.log"));
        assert!(path_to_main_log.is_some());

        let log_text = match file_system::read_file(path_to_main_log.unwrap()) {
            Err(e) => panic!("{}", e),
            Ok(v) => String::from_utf8(v).unwrap(),
        };

        assert!(has_text(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2}),(\d{3}) - ERROR - text", &log_text));
        assert!(has_text(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2}),(\d{3}) - TRACE - text", &log_text));
        assert!(has_text(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2}),(\d{3}) - DEBUG - text", &log_text));
        assert!(has_text(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2}),(\d{3}) - WARN - text",
                         &log_text));
        assert!(has_text(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2}),(\d{3}) - INFO - text",
                         &log_text));

    }

    fn has_text<'a>(regex_text: &'a str, text: &String) -> bool {

        let re = match regex::Regex::new(regex_text) {
            Ok(v) => v,
            Err(e) => {
                panic!(r"Failed regex string '{}': {}", regex_text, e);
            }
        };

        return re.is_match(text);
    }
}
