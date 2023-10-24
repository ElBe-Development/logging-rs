use logging_rs;

fn main() {
    let logger = logging_rs::Logger::default();

    logging_rs::debug!(logger, "Debug message");
    logging_rs::info!(logger, "Info");
    logging_rs::warn!(logger, "Warning");
    logging_rs::error!(logger, "Error!");
    logging_rs::fatal!(logger, "Fatal error!");
    logging_rs::log!(logger, "Log message");
}
