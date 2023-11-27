use logging_rs;

fn main() {
    let logger = logging_rs::Logger::default();

    logging_rs::debug!(logger, "Debug message");
    logging_rs::info!(logger, "Info");
    logging_rs::warn!(logger, "Warning");
    logging_rs::error!(logger, "Error!");
    logging_rs::fatal!(logger, "Fatal error!");
    logging_rs::log!(logger, "Log message");

    logging_rs::debug!(logger, "Debug message with {{more_info}}", "more_info" = "additional information");
    logging_rs::info!(logger, "Info and {{details}}", "details" = "more stuff");
    logging_rs::warn!(logger, "Warning: {{name}} is bad", "name" = "War");
    logging_rs::error!(logger, "Error! {{stuff}} went wrong", "stuff" = "Everything");
    logging_rs::fatal!(logger, "Fatal error! Code {{code}}", "code" = "404");
    logging_rs::log!(logger, "Log message and {{more}}", "more" = "more");
}
