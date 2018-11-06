pub mod file_appender;
pub mod console_appender;

pub use self::file_appender::FileAppender;
pub use self::console_appender::ConsoleAppender;

pub trait Appender: Send {
    fn append(&mut self, log: &String, flush: bool);
}

