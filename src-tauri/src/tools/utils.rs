use log::{LevelFilter, info, error};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::str::FromStr;
use lazy_static::lazy_static;

pub fn init_logging_config() -> log4rs::Handle {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap()
}

lazy_static::lazy_static! {
    pub static ref LOGGER: log4rs::Handle = init_logging_config();
}

pub fn str2bool(v: &str) -> Result<bool, String> {
    match v.to_lowercase().as_str() {
        "yes" | "true" | "t" | "y" | "1" => Ok(true),
        "no" | "false" | "f" | "n" | "0" => Ok(false),
        _ => Err("Boolean value expected.".to_string()),
    }
}

// 使用示例
pub fn example_usage() {
    info!("This is an info message");
    error!("This is an error message");

    match str2bool("yes") {
        Ok(value) => println!("Converted to bool: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}
