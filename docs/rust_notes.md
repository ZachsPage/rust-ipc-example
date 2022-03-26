# Rust Docs Notes

## Project Creation / Compiling
`cargo new <name>`
`cargo build` (Just to see if compiles without gening a bin - `cargo check`)
  * Much better than compiling with rustc...
  * Can also add `build` steps with a `build.rs` with a main
    * Can use `[build-dependencies]` to specify crates only used when building
Then can do `cargo run`

## Rust Packaging
Registry - crates.io - crate host
* Can use also use git repo as dep source
Workspace - can house multiple crates
* `[[workspace]] members = ["crate_folder_one", "crate_folder_two"]`
  * Each with their own subfolder & Cargo.toml
Packages - build / test / share crates
Crates - tree of modules
* Add via deps - `<dep name> = "<version>"`
* cargo.locks to same minor vers, update with `cargo update`
* Link with `use <crate>::<module>` & `use <name> as <new name>`
  * Was previously  `extern crate <name>`, but prefer "use" now
  * Multi `use` - `use <crate>::{<mod_1>, <mod_2>}` or `use <crate>::*`
* Can only produce one library, but multiple binaries
  * Compiles each `src/bin/<name>.rs` to their own binary, or specify with 
     `[[bin]]` and a `name:/path:<main.rs>` attribute
Modules - like namespace `mod {`
* Declare like `<mod> <name>;` to say either `<name>/mod.rs` or same dir `<name>.rs`
Pub - export a pub / mod / fn / use by prepending with `pub`
Super - super:: to go up a relative level

## Syntax
let /+ mut - constant - ex. `let x: i32 = 324;` - or infer type
`let mut s = String::from("hello")` - mutable string instead of constant
* `String` is `Vec<u8>` stored on the heap, growable, not null term'd
* `&str` is a slice `&[u8]` - reference view into a `String`
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
No `->` - Rust has auto dereference when calling fcns
Switch - `match <var> { <value> => <action>, ... }` 
* Catch with `other` or `_` (if no need to use value in caught case)
  * Single case - `if let <cond> = <var to match> {}` - do if true - can pair with `else`
Ex. Match with Optional:
```rust
fn plus_one(x: Optional<i8>) -> Optional<i8> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  } // Note return since no ;
}
```
### Structs
`struct User { <name>: <type>, etc }` in place with `let user = User {...`
No ctors - use `fn build_user(params) -> User { User {...} }`
Struct Update Syntax - `let user2 = User { email: String::from(), ..user1 }`
* Inits email to something new, then takes rest of user1
Tuple Structs - type only members - `struct Point(i32,i32,i32)`
* `let origin = Point(0,0,0);` - use `origin.<idx>` to access
* Can be passed to fcns as parameter packs
Unit Like Struct - no members. Useful to impl. a trait on some type but dont 
 have have any data to store with the type itself. See section `Traits`
### Methods / Traits
`impl <struct name> { fn ... }` - Associated Functions
* Typeless keyword param `&self` if needed - like rhs / lhs `(&self, other: <Type>)`
  * This makes them `Methods` instead of Associated Functions
Printable struct - `#[derive(Debug)]` above struct.
* println! formats - One line {:?} - multi {:#?}
* dbg! instead - prints [<file>:<line>] resolved code
### Enums
`enum IpAddrKind { V4, V6 }` - `let four = IpAddrKind::V4`
Enum Variant - stores data like `V4(String), V6(String)` - tuples are also valid
* Std lib ex: `Option<T>{ None, Some(T) }` - Either None or stores data
  * [Option docs](https://doc.rust-lang.org/std/option/enum.Option.html)
* Example with enum & vector:
```
enum SpreadsheetCell { Int(i32), Float(f64), Text(String) }
let row = vec![ SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12), ];
```
### Errors
No exceptions, just `Result<T,E>` & macro `panic!` to stop execution
* Prefer to panic! if shouldnt continue else return result
* Control behavior with `RUST_BACKTRACE=1 cargo run` or `[profile] panic="<opt>"`
* `.unwrap()` - like a match expression - returns whats in `Ok`, or panic!
* `.expect(String)` - if fails to unwrap, prints string
* Return Err on error with `?` at end of statement
  * Shortcut to match `Err(e) => return Err(e)`
Example of handling with `unwrap_or_else`
```rust
let f = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
});
```

### Ownership
* Defaults to move instead of a a copy - see `Copy` trait
* Can only have ONE mutable reference (&mut) in scope at a time
* Error if have &, the &mut also. But many & is okay