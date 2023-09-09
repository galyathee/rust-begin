# REPO rust-begin - First steps with the RUST programming language
First thing first: read https://en.wikipedia.org/wiki/Rust_(programming_language)

## A bit of history: what is RUST
Rust is a systems programming language that was first introduced in 2010 by Mozilla Research. It is designed to be a safe, concurrent, and practical language that emphasizes performance and memory safety. Rust has gained popularity for its ability to provide low-level control over system resources while preventing common programming errors like null pointer dereferences, buffer overflows, and data races.

## Key Features

- **Memory Safety:** Rust uses a unique ownership system that enforces strict rules about how memory is allocated and deallocated. This system eliminates common programming errors like null pointer dereferences and buffer overflows at compile time.

- **Concurrency:** Rust provides built-in support for concurrent programming through features like threads and asynchronous programming. The ownership system ensures that data races are prevented at compile time.

- **Zero-cost Abstractions:** Rust allows developers to write high-level code with abstractions without incurring any runtime performance penalties. The compiler optimizes code to run as efficiently as possible.

- **Systems Programming:** Rust is often used for system-level programming tasks, such as developing operating systems, device drivers, and embedded systems, due to its low-level capabilities and performance.

- **Cross-Platform:** Rust is designed to be cross-platform and can target a wide range of operating systems and hardware architectures.

- **Package Manager:** Rust has a package manager called Cargo, which simplifies dependency management, building, and publishing of Rust projects.

- **Ecosystem:** Rust has a growing ecosystem of libraries and frameworks that make it easier to develop various types of applications, from web services to game development.

Rust's focus on safety and performance has made it a popular choice for systems programming and other performance-critical applications. It has gained traction in industries like game development, web development (through frameworks like Rocket and Actix), and networking due to its ability to provide high-performance, concurrent code with strong guarantees of safety.

### RUST was influenced by many different programming languages:
#### Systems Programming Languages:
C++, C#, ERLANG, SWIFT
#### Functional Programming Languages:
Haskell, OCaml, Scheme, STANDARD ML
#### Web Development and High-Level Languages:
Ruby, Elm
#### Concurrent and Specialized Languages:
Alef, Cyclone, Limbo, Newsqueak

### Install
Follow the steps at https://www.rust-lang.org/tools/install \
If the page disapeared type this:\
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### OSX Path Registration
Type in a terminal: `export PATH="$PATH:/Users/<your user>/.cargo/bin"`\
Test with `which rustc`\
Get version via `rustc --version`

### Hello World
In a file .rs type
``` 
fn main() {
  println!("Hello, world!");
}
```
### Generate a binary into a dedicated folder from an rs file:
Call: `rustc main.rs --out-dir bin`\
After that command the bin folder was created to store the binary 'main' file

### Prefix unused variables with underscore '_'
In primitives.rs: `let _binary1: bool = true;`

### Import an external source
In main.rs: `mod primitives;`\
To call the main function of primitives use following notation `primitives::main();`

### Declare a function public
Use the `pub` word as in: `pub fn my_function() {...}`