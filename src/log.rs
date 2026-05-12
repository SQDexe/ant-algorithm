use {
    core::fmt::Arguments,
    console::{
        Style,
        StyledObject
        }
    };



/** **Technical part** - `Info` level style. */
const INFO_STYLE: Style = Style::new().green();
/** **Technical part** - `Warning` level style. */
const WARNING_STYLE: Style = Style::new().yellow();
/** **Technical part** - `Error` level style. */
const ERROR_STYLE: Style = Style::new().red();

/** **Technical part** - `Info` level tag. */
const INFO_MSG: &str = "INFO";
/** **Technical part** - `Warning` level tag. */
const WARNING_MSG: &str = "WARNING";
/** **Technical part** - `Error` level tag. */
const ERROR_MSG: &str = "ERROR";
/** **Technical part** - `Critical` level tag. */
const CRITICAL_MSG: &str = "CRITICAL";

/** **Technical part** - log level specifier. */
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    /** Information level. */
    Info,
    /** Warning level. */
    Warning,
    /** Error level. */
    Error,
    /** Critical error level. */
    Critical
    }

/** **Technical part** - trait implementation for converting log level into styled object. */
impl From<LogLevel> for StyledObject<&'static str> {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info =>
                INFO_STYLE.apply_to(INFO_MSG),
            LogLevel::Warning =>
                WARNING_STYLE.apply_to(WARNING_MSG),
            LogLevel::Error =>
                ERROR_STYLE.apply_to(ERROR_MSG),
            LogLevel::Critical =>
                ERROR_STYLE.apply_to(CRITICAL_MSG)
            }
        }
    }

/** **Technical part** - function for logging messages to stderr with specified level tag. */
pub fn log(level: LogLevel, msg: Arguments<'_>) {
    let error_tag = StyledObject::from(level);

    eprintln!("[{error_tag}]: {msg}");
    }

/** **Technical part** - shorthand macro for logging with `Info` level. */
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Info, ::core::format_args!($($arg)*))
        };
    }

/** **Technical part** - shorthand macro for logging with `Warning` level. */
#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Warning, ::core::format_args!($($arg)*))
        };
    }

/** **Technical part** - shorthand macro for logging with `Error` level. */
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Error, ::core::format_args!($($arg)*))
        };
    }

/** **Technical part** - shorthand macro for logging with `Critical` level. Used only for panics. */
#[macro_export]
macro_rules! critical {
    ($($arg:tt)*) => {
        $crate::log::log($crate::log::LogLevel::Critical, ::core::format_args!($($arg)*))
        };
    }