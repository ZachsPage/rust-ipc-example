# Rust IPC Project Structure
## Description
Using Rust for the first time and wanted an example that ties together multiple 
components + pulls in external dependencies. So, this demo should:
* Have a library that defines a msg structure type
* Have two processes that communicate with IPC
* Have a top level that launches both processes and waits for their exit code

## Working Notes
https://doc.rust-lang.org/book/title-page.html

### Project Creation / Compiling
`cargo new <name>`
`cargo build` (Just to see if compiles without gening a bin - `cargo check`)
  * Much better than compiling with rustc...
Then can do `cargo run`

### Crates
Add via deps, cargo.locks to same minor vers, update with `cargo update`

### Syntax
let /+ mut - constant - ex. `let x: i32 = 324;` - or infer type
`let mut s = String::from("hello")` - mutable string instead of constant
* If `let s2 = s` makes s invalid (move), unless using .clone() (deep copy)
  * Same with passing strings to functions - unless using ref & (see Ownership)
  * Doesnt apply to i8,16,etc since using stack (`Copy trait`) not heap
tuple () - can use destructuring with let
array [] -  [1,2,3] - or [3; 5] sets up 5 3's - slice &a[0..2] &a[..2]
expressions - `let x = { <multiple expressions> }` - lambda like
* CAN'T end in a semicolon - becomes a `statement` and wont return a value
`fn <name>(<param>:<type>>) -> <return>`
* Line that doesnt end in `;` is return
`println!("Var: {}", x)`
`if <cond> {<val>} else {<val>}` - not parenths
loop - can use like `let x = loop {...}`
`for element in a` - `for num in (1..4).rev() {` - output 4 3 2 1
`for (i, &item) in bytes.iter().enumerate() {..` - like pythons 

### Ownership
* Defaults to move instead of a a copy - see `Copy` trait
* Can only have ONE mutable reference (&mut) in scope at a time
* Error if have &, the &mut also. But many & is okay

#### Structs


### TODO
#### Research
Cross compiling rust for embedded?

