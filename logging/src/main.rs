/*

Using env_logger crate

use env_logger::Env;
use log::{debug, error, info, trace, warn};
fn main() {
    dotenv::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    trace!("Trace log");
    debug!("Debug log");
    info!("Info log");
    warn!("Warn log");
    error!("Error log");
}
*/

/*
    Using log4rs crate

*/

use log::{debug, error, info, trace, warn};
use log::{LevelFilter, SetLoggerError};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

fn main() -> Result<(), SetLoggerError> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}\n")))
        .build("log/output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Trace),
        )
        .unwrap();

    log4rs::init_config(config)?;
    trace!("Trace log");
    debug!("Debug log");
    info!("Info log");
    warn!("Warn log");
    error!("Error log");

    Ok(())
}
