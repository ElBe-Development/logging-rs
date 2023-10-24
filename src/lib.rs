#![doc = include_str!("../.github/README.md")]
// Logging-rs.
// Version: 1.0.0

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

// use chrono;


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


//////////////////
// HTTP METHODS //
//////////////////

// HTTP methods
/*#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HTTPMethod {
    /// GET request. The default value
    #[default]
    GET,
    /// POST request
    POST,
    /// PUT request
    PUT,
    /// PATCH request
    PATCH
}*/


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
    },
    // Web request (not currently implemented)
    /*REQUEST {
        method: HTTPMethod,
        url: String
    }*/
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
    ///     vec![("argument", "replaced value")]
    /// );
    /// ```
    ///
    /// # See also
    ///
    /// - [`Formatter`]
    /// - [`Output`]
    /// - [`Level`]
    pub fn format<'a>(&self, output: Output, level: Level, message: &'a str, mut arguments: Vec<(&str, &'a str)>) -> String {
        // TODO (ElBe): Make timestamps work. None of chrono, time or humantime have the things I want
        let timestamp: &str = "TIMESTAMP";

        // let time = chrono::Utc::now();
        //let _timestamp = time.format("%Y-%m-%d %H:%M:%S").to_string(); //chrono::format::DelayedFormat<chrono::format::StrftimeItems<'_>>
        //let parsed = chrono::NaiveDateTime::parse_from_str(&_timestamp.to_string(), "%Y-%m-%d %H:%M:%S").expect("Bad");

        //let borrowed = &_timestamp;

        //println!("{}", _timestamp);
        //println!("{}", parsed.to_string().as_str());

        // arguments.push(("timestamp", &time.format("%Y-%m-%d %H:%M:%S").to_string()));

        let mut colors: Vec<(&str, &str)> = vec![
            // Formatting codes
            ("end", "\x1b[0m"),
            ("bold", "\x1b[1m"),
            ("italic", "\x1b[3m"),
            ("underline", "\x1b[4m"),
            ("overline", "\x1b[53m"),

            // Foreground colors
            ("color.black", "\x1b[30m"),
            ("color.red", "\x1b[31m"),
            ("color.green", "\x1b[32m"),
            ("color.yellow", "\x1b[33m"),
            ("color.blue", "\x1b[34m"),
            ("color.magenta", "\x1b[35m"),
            ("color.cyan", "\x1b[36m"),
            ("color.white", "\x1b[37m"),

            // Bright foreground colors
            ("color.bright_black", "\x1b[90m"),
            ("color.bright_red", "\x1b[91m"),
            ("color.bright_green", "\x1b[92m"),
            ("color.bright_yellow", "\x1b[93m"),
            ("color.bright_blue", "\x1b[94m"),
            ("color.bright_magenta", "\x1b[95m"),
            ("color.bright_cyan", "\x1b[96m"),
            ("color.bright_white", "\x1b[97m"),

            // Background colors
            ("back.black", "\x1b[40m"),
            ("back.red", "\x1b[41m"),
            ("back.green", "\x1b[42m"),
            ("back.yellow", "\x1b[43m"),
            ("back.blue", "\x1b[44m"),
            ("back.magenta", "\x1b[45m"),
            ("back.cyan", "\x1b[46m"),
            ("back.white", "\x1b[47m"),

            // Bright background colors
            ("back.bright_black", "\x1b[100m"),
            ("back.bright_red", "\x1b[101m"),
            ("back.bright_green", "\x1b[102m"),
            ("back.bright_yellow", "\x1b[103m"),
            ("back.bright_blue", "\x1b[104m"),
            ("back.bright_magenta", "\x1b[105m"),
            ("back.bright_cyan", "\x1b[106m"),
            ("back.bright_white", "\x1b[107m"),
        ];

        let level_string: (&str, &str) = ("level", match level {
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARNING",
            Level::ERROR => "ERROR",
            Level::FATAL => "FATAL",
            Level::MESSAGE => "MESSAGE"
        });
        let colored_level_string: (&str, &str) = ("level", match level {
            Level::DEBUG => "DEBUG",
            Level::INFO => "{{color.blue}}INFO{{end}}",
            Level::WARN => "{{color.yellow}}WARNING{{end}}",
            Level::ERROR => "{{color.red}}ERROR{{end}}",
            Level::FATAL => "{{color.red}}FATAL{{end}}",
            Level::MESSAGE => "{{color.blue}}MESSAGE{{end}}"
        });

        arguments.push(("message", message));
        arguments.push(("timestamp", timestamp));
        //arguments.push(("timestamp", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string().to_owned().as_str()));

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
            result = result.replace(("{{".to_owned() + key + "}}").as_str(), value);
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
    ///     "src/lib.rs"
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
    pub fn log(&self, message: &str, level: Level, path: &str) {
        for writable in self.writable_list.clone() {
            let formatted: String = self.formatter.format(writable.clone(), level, message, vec![("path", path)]);

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
                },
                // TODO (ElBe): Add HTTP support
                /*Output::REQUEST { ref method, ref url } => {
                    match method {
                        HTTPMethod::GET => {},
                        HTTPMethod::POST => {},
                        HTTPMethod::PUT => {},
                        HTTPMethod::PATCH => {},
                    }
                }*/
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
            $logger.log($message, $crate::Level::DEBUG, std::panic::Location::caller().file());
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
            $logger.log($message, $crate::Level::INFO, std::panic::Location::caller().file());
        }
    };
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
            $logger.log($message, $crate::Level::WARN, std::panic::Location::caller().file());
        }
    };
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
            $logger.log($message, $crate::Level::ERROR, std::panic::Location::caller().file());
        }
    };
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
            $logger.log($message, $crate::Level::FATAL, std::panic::Location::caller().file());
        }
    };
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
            $logger.log($message, $crate::Level::MESSAGE, std::panic::Location::caller().file());
        }
    };
}
