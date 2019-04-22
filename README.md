# genlog
Test log generator

## Building from source

1. Install the Rust toolchain (stable) using `rustup` see [Installation](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
    * build has been tested with at least Rust 1.32

2. Run `cargo run -- --h` for help.

## Example

Create logs with 2 rotations (three files), each containing a maximum of 100 lines, if the end is not reached.
Begin and end are set set, the maximum gap between log entires is set to one second.

    >cargo run -- -o /tmp -l 100 -r 2 -b (date -d "22-Sep-2019 11:00:35.012" +"%s") -e (date -d "22-Sep-2019 12:32:35.012" +"%s") -g 1000
