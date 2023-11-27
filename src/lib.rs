#![doc = include_str!("../.github/README.md")]
// Logging-rs
// Version: 1.1.0

// Copyright (c) 2023-present I Language Development.

// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the 'Software'),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

/////////////
// EXPORTS //
/////////////

pub mod errors;


/////////////
// IMPORTS //
/////////////

use std;
use std::io::Write;

use chrono;


////////////////
// LOG LEVELS //
////////////////

/// Log levels
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Level {
    /// Debug log level. The default value
    #[default]
    DEBUG,
    /// Info log level
    INFO,
    /// Warn log level
    WARN,
    /// Error log level
    ERROR,
    /// Fatal log level
    FATAL,
    /// Message log level
    MESSAGE
}


/////////////////
// OUTPUT TYPE //
/////////////////

/// Output types
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Output {
    /// Stdout. The default value
    #[default]
    STDOUT,
    /// Stderr
    STDERR,
    /// File
    FILE {
        /// File path
        path: String
    }
}


///////////////
// FORMATTER //
///////////////

/// Logging formatter object.
///
/// Use [`Formatter::new()`] to create formatter objects instead of using this struct.
///
/// # Parameters
///
/// - `color_format_string`: Format string supporting special ASCII control characters
/// - `format_string`: Format string *NOT* supporting special ASCII control characters
/// - `timestamp_format`: Timestamp format string in strftime format
///
/// # Returns
///
/// A new `Formatter` object with the specified format strings.
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// logging_rs::Formatter {
///     color_format_string: "format string with color support".to_owned(),
///     format_string: "format string".to_owned(),
///     timestamp_format: "timestamp format".to_owned()
/// };
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Formatter {
    /// Format string supporting special ASCII control characters
    pub color_format_string: String,
    /// Format string *NOT* supporting special ASCII control characters
    pub format_string: String,
    /// Timestamp format string in strftime format
    pub timestamp_format: String,
}

impl Default for Formatter {
    fn default() -> Formatter {
        return Formatter::new("[{{color.bright_blue}}{{timestamp}}{{end}}] [{{level}}] {{path}}: {{message}}", "[{{timestamp}}] [{{level}}] {{path}}: {{message}}", "%Y-%m-%d %H:%M:%S");
    }
}

impl Formatter {
    /// Creates a new formatter object.
    ///
    /// # Parameters
    ///
    /// - `color_format_string`: Format string supporting special ASCII control characters
    /// - `format_string`: Format string *NOT* supporting special ASCII control characters
    /// - `timestamp_format`: Timestamp format string in strftime format
    ///
    /// # Returns
    ///
    /// A new `Formatter` object with the specified format strings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use logging_rs;
    /// logging_rs::Formatter::new(
    ///     "[{{color.bright_blue}}{{timestamp}}{{end}}] [{{level}}] {{path}}: {{message}}",
    ///     "[{{timestamp}}] [{{level}}] {{path}}: {{message}}",
    ///     "%Y-%m-%d %H:%M:%S"
    /// );
    /// ```
    ///
    /// # See also
    ///
    /// - [`Formatter`]
    pub fn new(color_format_string: &str, format_string: &str, timestamp_format: &str) -> Formatter {
        Formatter {
            color_format_string: color_format_string.to_owned(),
            format_string: format_string.to_owned(),
            timestamp_format: timestamp_format.to_owned()
        }
    }

    /// Formats the given message.
    ///
    /// # Parameters
    ///
    /// - `self`: The formatter object
    /// - `output`: The [`Output`] to write to
    /// - `level`: The log [`Level`] to use for formatting
    /// - `message`: The message to log
    /// - `arguments`: A vector of additional formatting arguments
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use logging_rs;
    /// # let formatter: logging_rs::Formatter = logging_rs::Formatter::default();
    /// formatter.format(
    ///     logging_rs::Output::default(),
    ///     logging_rs::Level::default(),
    ///     "Some message with an {{argument}}",
    ///     vec![("argument", "replaced value".to_string())]
    /// );
    /// ```
    ///
    /// # See also
    ///
    /// - [`Formatter`]
    /// - [`Output`]
    /// - [`Level`]
    pub fn format<'a>(&self, output: Output, level: Level, message: &'a str, mut extra_arguments: Vec<(&str, String)>) -> String {
        let mut arguments: Vec<(&str, String)> = vec![];
        let mut colors: Vec<(&str, String)> = vec![
            // Formatting codes
            ("end", "\x1b[0m".to_string()),
            ("bold", "\x1b[1m".to_string()),
            ("italic", "\x1b[3m".to_string()),
            ("underline", "\x1b[4m".to_string()),
            ("overline", "\x1b[53m".to_string()),

            // Foreground colors
            ("color.black", "\x1b[30m".to_string()),
            ("color.red", "\x1b[31m".to_string()),
            ("color.green", "\x1b[32m".to_string()),
            ("color.yellow", "\x1b[33m".to_string()),
            ("color.blue", "\x1b[34m".to_string()),
            ("color.magenta", "\x1b[35m".to_string()),
            ("color.cyan", "\x1b[36m".to_string()),
            ("color.white", "\x1b[37m".to_string()),

            // Bright foreground colors
            ("color.bright_black", "\x1b[90m".to_string()),
            ("color.bright_red", "\x1b[91m".to_string()),
            ("color.bright_green", "\x1b[92m".to_string()),
            ("color.bright_yellow", "\x1b[93m".to_string()),
            ("color.bright_blue", "\x1b[94m".to_string()),
            ("color.bright_magenta", "\x1b[95m".to_string()),
            ("color.bright_cyan", "\x1b[96m".to_string()),
            ("color.bright_white", "\x1b[97m".to_string()),

            // Background colors
            ("back.black", "\x1b[40m".to_string()),
            ("back.red", "\x1b[41m".to_string()),
            ("back.green", "\x1b[42m".to_string()),
            ("back.yellow", "\x1b[43m".to_string()),
            ("back.blue", "\x1b[44m".to_string()),
            ("back.magenta", "\x1b[45m".to_string()),
            ("back.cyan", "\x1b[46m".to_string()),
            ("back.white", "\x1b[47m".to_string()),

            // Bright background colors
            ("back.bright_black", "\x1b[100m".to_string()),
            ("back.bright_red", "\x1b[101m".to_string()),
            ("back.bright_green", "\x1b[102m".to_string()),
            ("back.bright_yellow", "\x1b[103m".to_string()),
            ("back.bright_blue", "\x1b[104m".to_string()),
            ("back.bright_magenta", "\x1b[105m".to_string()),
            ("back.bright_cyan", "\x1b[106m".to_string()),
            ("back.bright_white", "\x1b[107m".to_string()),
        ];

        let level_string: (&str, String) = ("level", match level {
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARNING",
            Level::ERROR => "ERROR",
            Level::FATAL => "FATAL",
            Level::MESSAGE => "MESSAGE"
        }.to_string());
        let colored_level_string: (&str, String) = ("level", match level {
            Level::DEBUG => "DEBUG",
            Level::INFO => "{{color.blue}}INFO{{end}}",
            Level::WARN => "{{color.yellow}}WARNING{{end}}",
            Level::ERROR => "{{color.red}}ERROR{{end}}",
            Level::FATAL => "{{color.red}}FATAL{{end}}",
            Level::MESSAGE => "{{color.blue}}MESSAGE{{end}}"
        }.to_string());

        arguments.push(("message", message.to_string()));
        arguments.push(("timestamp", chrono::Local::now().format(&self.timestamp_format).to_string()));
        arguments.append(&mut extra_arguments);

        let mut result: String = match output {
            Output::STDOUT | Output::STDERR => {
                arguments.push(colored_level_string);
                self.color_format_string.to_owned()
            },
            _ => {
                arguments.push(level_string);
                self.format_string.to_owned()
            }
        };

        arguments.append(&mut colors);

        for (key, value) in arguments {
            result = result.replace(("{{".to_owned() + key + "}}").as_str(), &value);
        }

        return result.clone();
    }
}


///////////////////
// LOGGER STRUCT //
///////////////////

/// Logger object.
///
/// Use [`Logger::new()`] to create logger objects instead of using this struct.
///
/// # Parameters
///
/// - `formatter`: The [`Formatter`] to use for formatting messages
/// - `writable_list`: A vector of [`Output`]s to write to
///
/// # Returns
///
/// A new `Logger` object with the specified formatter and writables.
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// logging_rs::Logger {
///     formatter: logging_rs::Formatter::default(),
///     writable_list: vec![logging_rs::Output::default()]
/// };
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Logger {
    pub formatter: Formatter,
    pub writable_list: Vec<Output>
}

impl Default for Logger {
    fn default() -> Logger {
        return Logger::new(Formatter::default(), vec![Output::STDOUT]);
    }
}

impl Logger {
    /// Creates a new logger object.
    ///
    /// # Parameters
    ///
    /// - `formatter`: The [`Formatter`] to use for formatting messages
    /// - `writable_list`: A vector of [`Output`]s to write to
    ///
    /// # Returns
    ///
    /// A new `Logger` object with the specified formatter and writables.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use logging_rs;
    /// logging_rs::Logger::new(logging_rs::Formatter::default(), vec![logging_rs::Output::default()]);
    /// ```
    ///
    /// # See also
    ///
    /// - [`Logger`]
    pub fn new(formatter: Formatter, writable_list: Vec<Output>) -> Logger {
        Logger {
            formatter: formatter,
            writable_list: writable_list
        }
    }

    /// Logs the given message.
    ///
    /// # Parameters
    ///
    /// - `self`: The logger object
    /// - `message`: The message to log
    /// - `level`: The log [`Level`] to use for logging
    /// - `path`: The path of the calling file
    /// - `arguments`: A list of arguments to use when formatting the message
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use logging_rs;
    /// # let logger: logging_rs::Logger = logging_rs::Logger::default();
    /// logger.log(
    ///     "Some message",
    ///     logging_rs::Level::default(),
    ///     "src/lib.rs",
    ///     vec![]
    /// );
    /// ```
    ///
    /// # See also
    ///
    /// - [`debug!()`]
    /// - [`info!()`]
    /// - [`warn!()`]
    /// - [`error!()`]
    /// - [`fatal!()`]
    /// - [`log!()`]
    /// - [`Logger`]
    /// - [`Level`]
    pub fn log(&self, message: &str, level: Level, path: &str, mut arguments: Vec<(&str, String)>) {
        arguments.push(("path", path.to_string()));
        for writable in self.writable_list.clone() {
            let formatted: String = self.formatter.format(writable.clone(), level, message, arguments.clone());

            match writable {
                Output::STDOUT => println!("{}", formatted),
                Output::STDERR => eprintln!("{}", formatted),
                Output::FILE { ref path } => {
                    let file: Result<std::fs::File, std::io::Error> = std::fs::OpenOptions::new().create(true).append(true).write(true).open(path);
                    let write: Result<_, std::io::Error> = write!(file.as_ref().unwrap(), "{}", formatted);

                    if let Err(error) = file {
                        errors::Error::new("File error", "The file could not be opened", 1).raise(format!("Path: {}\nError: {}", path, error).as_str());
                    }

                    if let Err(error) = write {
                        errors::Error::new("Writing error", "The file could not be edited", 2).raise(format!("File: {}\nText: {}\nError: {}", path, formatted, error).as_str());
                    }
                }
            }
        }
    }
}


////////////
// MACROS //
////////////

/// Logs the given message with logging level [`Level::DEBUG`].
///
/// # Parameters
///
/// - `logger`: The logger object to log with
/// - `message`: The message to log
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// # let logger: logging_rs::Logger = logging_rs::Logger::default();
/// logging_rs::debug!(logger, "A message");
/// logging_rs::debug!(logger, "A message with more {{details}}", "details" = "stuff");
/// ```
///
/// # See also
///
/// - [`info!()`]
/// - [`warn!()`]
/// - [`error!()`]
/// - [`fatal!()`]
/// - [`log!()`]
/// - [`Logger`]
#[macro_export]
macro_rules! debug {
    ($logger:expr, $message:expr) => {
        {
            $logger.log($message, $crate::Level::DEBUG, std::panic::Location::caller().file(), vec![]);
        }
    };

    ($logger:expr, $message:expr, $($argument_name:literal = $argument_value:literal),* $(,)?) => {
        {
            let mut arguments: Vec<(&str, String)> = vec![];

            $(
                arguments.push(($argument_name, $argument_value.to_string()));
            )*

            $logger.log($message, $crate::Level::DEBUG, std::panic::Location::caller().file(), arguments);
        }
    };
}

/// Logs the given message with logging level [`Level::INFO`].
///
/// # Parameters
///
/// - `logger`: The logger object to log with
/// - `message`: The message to log
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// # let logger: logging_rs::Logger = logging_rs::Logger::default();
/// logging_rs::info!(logger, "A message");
/// logging_rs::info!(logger, "A message with more {{details}}", "details" = "stuff");
/// ```
///
/// # See also
///
/// - [`debug!()`]
/// - [`warn!()`]
/// - [`error!()`]
/// - [`fatal!()`]
/// - [`log!()`]
/// - [`Logger`]
#[macro_export]
macro_rules! info {
    ($logger:expr, $message:expr) => {
        {
            $logger.log($message, $crate::Level::INFO, std::panic::Location::caller().file(), vec![]);
        }
    };

    ($logger:expr, $message:expr, $($argument_name:literal = $argument_value:literal),* $(,)?) => {
        {
            let mut arguments: Vec<(&str, String)> = vec![];

            $(
                arguments.push(($argument_name, $argument_value.to_string()));
            )*

            $logger.log($message, $crate::Level::INFO, std::panic::Location::caller().file(), arguments);
        }
    }
}

/// Logs the given message with logging level [`Level::WARN`].
///
/// # Parameters
///
/// - `logger`: The logger object to log with
/// - `message`: The message to log
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// # let logger: logging_rs::Logger = logging_rs::Logger::default();
/// logging_rs::warn!(logger, "A message");
/// logging_rs::warn!(logger, "A message with more {{details}}", "details" = "stuff");
/// ```
///
/// # See also
///
/// - [`debug!()`]
/// - [`info!()`]
/// - [`error!()`]
/// - [`fatal!()`]
/// - [`log!()`]
/// - [`Logger`]
#[macro_export]
macro_rules! warn {
    ($logger:expr, $message:expr) => {
        {
            $logger.log($message, $crate::Level::WARN, std::panic::Location::caller().file(), vec![]);
        }
    };

    ($logger:expr, $message:expr, $($argument_name:literal = $argument_value:literal),* $(,)?) => {
        {
            let mut arguments: Vec<(&str, String)> = vec![];

            $(
                arguments.push(($argument_name, $argument_value.to_string()));
            )*

            $logger.log($message, $crate::Level::WARN, std::panic::Location::caller().file(), arguments);
        }
    }
}

/// Logs the given message with logging level [`Level::ERROR`].
///
/// # Parameters
///
/// - `logger`: The logger object to log with
/// - `message`: The message to log
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// # let logger: logging_rs::Logger = logging_rs::Logger::default();
/// logging_rs::error!(logger, "A message");
/// logging_rs::error!(logger, "A message with more {{details}}", "details" = "stuff");
/// ```
///
/// # See also
///
/// - [`debug!()`]
/// - [`info!()`]
/// - [`warn!()`]
/// - [`fatal!()`]
/// - [`log!()`]
/// - [`Logger`]
#[macro_export]
macro_rules! error {
    ($logger:expr, $message:expr) => {
        {
            $logger.log($message, $crate::Level::ERROR, std::panic::Location::caller().file(), vec![]);
        }
    };

    ($logger:expr, $message:expr, $($argument_name:literal = $argument_value:literal),* $(,)?) => {
        {
            let mut arguments: Vec<(&str, String)> = vec![];

            $(
                arguments.push(($argument_name, $argument_value.to_string()));
            )*

            $logger.log($message, $crate::Level::ERROR, std::panic::Location::caller().file(), arguments);
        }
    }
}

/// Logs the given message with logging level [`Level::FATAL`].
///
/// # Parameters
///
/// - `logger`: The logger object to log with
/// - `message`: The message to log
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// # let logger: logging_rs::Logger = logging_rs::Logger::default();
/// logging_rs::fatal!(logger, "A message");
/// logging_rs::fatal!(logger, "A message with more {{details}}", "details" = "stuff");
/// ```
///
/// # See also
///
/// - [`debug!()`]
/// - [`info!()`]
/// - [`warn!()`]
/// - [`error!()`]
/// - [`log!()`]
/// - [`Logger`]
#[macro_export]
macro_rules! fatal {
    ($logger:expr, $message:expr) => {
        {
            $logger.log($message, $crate::Level::FATAL, std::panic::Location::caller().file(), vec![]);
        }
    };

    ($logger:expr, $message:expr, $($argument_name:literal = $argument_value:literal),* $(,)?) => {
        {
            let mut arguments: Vec<(&str, String)> = vec![];

            $(
                arguments.push(($argument_name, $argument_value.to_string()));
            )*

            $logger.log($message, $crate::Level::FATAL, std::panic::Location::caller().file(), arguments);
        }
    }
}

/// Logs the given message with logging level [`Level::MESSAGE`].
///
/// # Parameters
///
/// - `logger`: The logger object to log with
/// - `message`: The message to log
///
/// # Examples
///
/// ```rust
/// # use logging_rs;
/// # let logger: logging_rs::Logger = logging_rs::Logger::default();
/// logging_rs::log!(logger, "A message");
/// logging_rs::log!(logger, "A message with more {{details}}", "details" = "stuff");
/// ```
///
/// # See also
///
/// - [`debug!()`]
/// - [`info!()`]
/// - [`warn!()`]
/// - [`error!()`]
/// - [`fatal!()`]
/// - [`Logger`]
#[macro_export]
macro_rules! log {
    ($logger:expr, $message:expr) => {
        {
            $logger.log($message, $crate::Level::MESSAGE, std::panic::Location::caller().file(), vec![]);
        }
    };

    ($logger:expr, $message:expr, $($argument_name:literal = $argument_value:literal),* $(,)?) => {
        {
            let mut arguments: Vec<(&str, String)> = vec![];

            $(
                arguments.push(($argument_name, $argument_value.to_string()));
            )*

            $logger.log($message, $crate::Level::MESSAGE, std::panic::Location::caller().file(), arguments);
        }
    }
}
