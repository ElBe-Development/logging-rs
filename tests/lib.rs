// logging-rs tests
// Version: 1.1.0

// Copyright (c) 2023-present ElBe Development.

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

////////////////////////////////
// IMPORTS AND USE STATEMENTS //
////////////////////////////////

#[allow(unused_imports)]
use chrono;
#[allow(unused_imports)]
use logging_rs;


///////////
// TESTS //
///////////

#[cfg(test)]
mod tests {
    #[test]
    fn test_level_default() {
        assert_eq!(
            logging_rs::Level::default(),
            logging_rs::Level::DEBUG
        );
    }

    #[test]
    fn test_output_default() {
        assert_eq!(
            logging_rs::Output::default(),
            logging_rs::Output::STDOUT
        );
    }

    #[test]
    fn test_formatter_default() {
        assert_eq!(
            logging_rs::Formatter::default(),
            logging_rs::Formatter {
                color_format_string: "[{{color.bright_blue}}{{timestamp}}{{end}}] [{{level}}] {{path}}: {{message}}".to_owned(),
                format_string: "[{{timestamp}}] [{{level}}] {{path}}: {{message}}".to_owned(),
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_owned()
            }
        );
    }

    #[test]
    fn test_formatter_new() {
        assert_eq!(
            logging_rs::Formatter::new(
                "[{{color.bright_blue}}{{timestamp}}{{end}}] [{{level}}] {{path}}: {{message}}",
                "[{{timestamp}}] [{{level}}] {{path}}: {{message}}",
                "%Y-%m-%d %H:%M:%S"
            ),
            logging_rs::Formatter {
                color_format_string: "[{{color.bright_blue}}{{timestamp}}{{end}}] [{{level}}] {{path}}: {{message}}".to_owned(),
                format_string: "[{{timestamp}}] [{{level}}] {{path}}: {{message}}".to_owned(),
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_owned()
            }
        );
    }

    #[test]
    fn test_formatter_format() {
        let formatter: logging_rs::Formatter = logging_rs::Formatter::default();

        assert_eq!(
            formatter.format(logging_rs::Output::default(), logging_rs::Level::default(), "Test", vec![]),
            format!("[\x1b[94m{}\x1b[0m] [DEBUG] {{{{path}}}}: Test", chrono::Local::now().format(&logging_rs::Formatter::default().timestamp_format))
        );
    }

    #[test]
    fn test_logger_default() {
        assert_eq!(
            logging_rs::Logger::default(),
            logging_rs::Logger {
                formatter: logging_rs::Formatter::default(),
                writable_list: vec![logging_rs::Output::STDOUT]
            }
        );
    }

    #[test]
    fn test_logger_new() {
        assert_eq!(
            logging_rs::Logger::new(logging_rs::Formatter::default(), vec![logging_rs::Output::STDOUT]),
            logging_rs::Logger {
                formatter: logging_rs::Formatter::default(),
                writable_list: vec![logging_rs::Output::STDOUT]
            }
        );
    }
}
