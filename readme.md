# Rust IPC Project Structure

## Description
Using Rust for the first time and wanted an example that ties together multiple 
components + pulls in external dependencies. So, this demo should:
* Have a library that defines a msg structure type / common functions
* Have two processes that communicate with IPC
* Have a top level that launches both processes and waits for their exit code
* Use some external Cargo crates along with local ones

## Working Notes
[My Notes from Rust's docs](./docs/rust_notes.md)
* [Rust Docs](https://doc.rust-lang.org/book/title-page.html)
  * [Bookmark Placement](https://doc.rust-lang.org/book/ch10-00-generics.html)
* [Rust API](https://doc.rust-lang.org/std/index.html)
* [IPC - Local sockets](https://docs.rs/interprocess/latest/interprocess/#modules)
  * Seems like best way for native, platform agnostic Rust IPC comms

## Todo's
### Milestones
* Finish noting Rust's docs
* Create example crates / modules to mock structure
### Later Research
Cross compiling rust for embedded?
Hosting own private registry? Use Docker to store / serve?
