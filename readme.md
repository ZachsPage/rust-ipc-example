# Rust IPC Project Structure

## Description
Using Rust for the first time and wanted an example that ties together multiple 
components + pulls in external dependencies. So, this demo should:
* Have a library that defines a msg structure type / common functions
* Have two processes that communicate with IPC
* Have a top level that launches both processes and waits for their exit code
* Use some external Cargo crates along with local ones

## Directory Descriptions
* `docs` - Repo working notes
  * [My Notes from Rust's docs](./docs/rust_notes.md)
* `src` - Holds `main.rs` & other demo code
  * `bin` - Rust creates a separate binary for each file in this directory
  * `ipc_comms` - My local library created for this demo
* `submods` - submodules
  * [ipc_channel](https://doc.servo.org/ipc_channel/index.html)
    * Seems like best way for native, platform agnostic Rust IPC comms
    * Wanted to link as a submodule as reference 

## Other Links
* [Rust Docs](https://doc.rust-lang.org/book/title-page.html)
  * [Bookmark Placement](https://doc.rust-lang.org/book/ch10-00-generics.html)
* [Rust API](https://doc.rust-lang.org/std/index.html)
