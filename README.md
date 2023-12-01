# Server example

This is example code showing how to set up a simple server that listens for
incoming connections on port 8765 and writes the data received to stdout.

- `src/bin/server.rs` contains the code that uses the `libc` crate to make
  system calls by calling the C wrapper functions.
- `src/bin/simpleserver.rs` contains the code that uses the higher-level Rust networking API to perform the same task.

## Running the servers

Run `$ cargo run --bin server` or `$ cargo run --bin simpleserver` to run the
servers. (Only one can be run at a time because they use the same port.)

## Connecting to the servers using `nc`

You can use `$ nc 127.0.0.1 8765` in a terminal to make a connection to your
server. Anything you type in the terminal window will be printed by the
server.
