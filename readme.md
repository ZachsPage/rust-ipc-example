# Rust IPC Project Structure

## Description
Using Rust for the first time and wanted an example that ties together multiple 
components + pulls in external dependencies. So, this demo should:
* Have a library that defines a msg structure type
* Have two processes that communicate with IPC
* Have a top level that launches both processes and waits for their exit code

## Working Notes
[My Notes from Rust's docs](./docs/rust_notes.md)
* [Rust Docs](https://doc.rust-lang.org/book/title-page.html)
* [Bookmark Placement](https://doc.rust-lang.org/book/ch10-00-generics.html)

## Todo's
### Milestones
* Finish noting Rust's docs
* Create example crates / modules to mock structure
### Later Research
Cross compiling rust for embedded?
Hosting own private registry? Use Docker to store / serve?
