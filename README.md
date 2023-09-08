# REPO rust-begin
## First steps with the RUST programming language

### Install
Follow the steps at https://www.rust-lang.org/tools/install

### OSX Path Registration
Type in a terminal: `export PATH="$PATH:/Users/<your user>/.cargo/bin"`\
Test with `which rustc`\
Get version via `rustc --version`

### Hello World
In a file .rs type\
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