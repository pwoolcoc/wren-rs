# Wren for Rust

You can use this library to interpret
[Wren](https://github.com/munificent/wren) code in your
[Rust](https://www.rust-lang.org/) programs.

## Use

```rust
extern crate wren;

use std::default::Default;

use wren::{VM, Error};

fn main() {
    let source = r#"
class Unicorn {
  hasHorn {
    return true
  }
}
"#;
    let vm = VM::new(Default::default()); // loads the VM with the default VM config
    match vm.interpret("Test", source) {
      Err(Error::CompileError(msg)) => println!("Compile Error: {}", msg),
      Err(Error::RuntimeError(msg)) => println!("Runtime Error: {}", msg),
      Err(Error::UnknownError(msg)) => println!("Unknown Error: {}", msg),
      _ => println!("Successfully ran wren source"),
    }
}
```
