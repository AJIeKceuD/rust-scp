use log::{Record, Level, Metadata};

pub struct SimpleLogger;

// mod db_log_object;
// use db_log_object::DBLogObject as DBLogObject;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            match record.level() {
                // Level::Info => {
                //     println!("!!!{} - {}", record.level(), record.args());
                //
                //     // deserialize
                //     let s: &str = &record.args().to_string()[..];
                //     let log: DBLogObject = match serde_json::from_str(s) {
                //         Ok(log) => {
                //             log
                //         }
                //         Err(err) => { // TODO is it okay? return without anything?
                //             println!("Info log error: {}", s);
                //             return;
                //         }
                //     };
                //
                //     // and save to db
                //     println!("{:?}", log);
                // }
                _ => {
                    println!("{} - {}", record.level(), record.args());
                }
            }
        }
    }

    fn flush(&self) {}
}