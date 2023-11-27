# Formatting codes

When formatting a message you can use arguments in the form of `{{name}}`. Following arguments are available by default:

| Name        | Description                                                                                                                                                           | Example               |
| :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :-------------------- |
| `message`   | Log message                                                                                                                                                           | `My message`          |
| `level`     | Uppercase level name. Will have colors attached to it if the output is stdout or stderr                                                                               | `ERROR`               |
| `timestamp` | UTC timestamp the log function was called (Technically the time the format function was called). Can be formatted using the `timestamp_format` field of the formatter | `2023-11-27 20:49:47` |
| `path`      | Relative path to the caller of the log macro                                                                                                                          | `src\main.rs`         |

Users can also specify custom arguments by either supplying a `Vec<(&str, String)>` of key-value pairs of the argument name and value or using the fields in the macros:

```rust,ignore
logging_rs::log!(logger, "My message with {{arg}}", "arg" = "my arguments")
```

## ASCII format characters

Most terminals support special ASCII characters.

| Name                   | Description                                         |
| :--------------------- | :-------------------------------------------------- |
| `end`                  | Escapes any previously started escape sequences     |
| `bold`                 | Makes the text bold                                 |
| `italic`               | Makes the text italic                               |
| `underline`            | Underlines the text                                 |
| `overline`             | Overlines the text (Not supported by all terminals) |
| `color.black`          | Makes the text black                                |
| `color.red`            | Makes the text red                                  |
| `color.green`          | Makes the text green                                |
| `color.yellow`         | Makes the text yellow                               |
| `color.blue`           | Makes the text blue                                 |
| `color.magenta`        | Makes the text magenta                              |
| `color.cyan`           | Makes the text cyan                                 |
| `color.white`          | Makes the text white                                |
| `color.bright_black`   | Makes the text bright black                         |
| `color.bright_red`     | Makes the text bright red                           |
| `color.bright_green`   | Makes the text bright green                         |
| `color.bright_yellow`  | Makes the text bright yellow                        |
| `color.bright_blue`    | Makes the text bright blue                          |
| `color.bright_magenta` | Makes the text bright magenta                       |
| `color.bright_cyan`    | Makes the text bright cyan                          |
| `color.bright_white`   | Makes the text bright white                         |
| `back.black`           | Makes the text background black                     |
| `back.red`             | Makes the text background red                       |
| `back.green`           | Makes the text background green                     |
| `back.yellow`          | Makes the text background yellow                    |
| `back.blue`            | Makes the text background blue                      |
| `back.magenta`         | Makes the text background magenta                   |
| `back.cyan`            | Makes the text background cyan                      |
| `back.white`           | Makes the text background white                     |
| `back.bright_black`    | Makes the text background bright black              |
| `back.bright_red`      | Makes the text background bright red                |
| `back.bright_green`    | Makes the text background bright green              |
| `back.bright_yellow`   | Makes the text background bright yellow             |
| `back.bright_blue`     | Makes the text background bright blue               |
| `back.bright_magenta`  | Makes the text background bright magenta            |
| `back.bright_cyan`     | Makes the text background bright cyan               |
| `back.bright_white`    | Makes the text background bright white              |
