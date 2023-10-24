<h1 align="center">
    logging-rs
</h1>
<h3 align="center">
    logging-rs helps you add logging to your projects using simple macros.
</h3>
<p align="center">
    <img src="https://img.shields.io/crates/v/logging-rs">
    <img src="https://www.codefactor.io/repository/github/ElBe-Development/logging-rs/badge">
    <img src="https://github.com/ElBe-Development/logging-rs/actions/workflows/megalinter.yml/badge.svg?branch=main&event=push">
    <img src="https://img.shields.io/badge/pre--commit-enabled-brightgreen?logo=pre-commit">
</p>

<img src="https://github.com/ElBe-Development/logging-rs/blob/main/.github/example.png?raw=true"/>

## About this project

logging-rs helps you add logging to your projects using simple macros.

## Installing

Run the following command to add the package to your dependencies:

```bash

$ cargo add logging-rs
...

```

### Git

To clone the repository locally using git run `git clone https://github.com/ElBe-Development/logging-rs.git`.

## Usage

To use logging-rs you need a configuration. It's best to keep it the same across multiple files. You then need to follow these steps:

1. Import the logging-rs crate:

    ```rust,ignore
    use logging_rs;
    ```

2. Create a new logger object:

    ```rust,ignore
    let logger = logging_rs::Logger::new(logging_rs::Formatter::default(), vec![logging_rs::Output::STDOUT]);
    ```

3. Log the messages you want to log:

    ```rust,ignore
    logging_rs::debug!(logger, "Debug message");
    logging_rs::info!(logger, "Info");
    logging_rs::warn!(logger, "Warning");
    logging_rs::error!(logger, "Error!");
    logging_rs::fatal!(logger, "Fatal error!");
    logging_rs::log!(logger, "Log message");
    ```

## Example

With the following rust code:

```rust,ignore
use logging_rs;

fn main() {
    let logger = logging_rs::Logger::default();

    logging_rs::debug!(logger, "Debug message");
}
```

You will get the following output:

```bash
[TIMESTAMP] [DEBUG] src\main.rs: Debug message
```

Where `TIMESTAMP` is the current timestamp.

## Contact

To contact us, get help or just chat with others, you can visit [our discord server](https://discord.gg/JVyyDukQqV).
