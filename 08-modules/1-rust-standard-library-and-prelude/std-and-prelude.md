# Rust Standard Library

## Includes
- Core data types
- Functions
- Macros
- and more ...
> For detail, see https://doc.rust-lang.org/std

## `use` statement:
- Bring a module into scope
- Usually included at the top
> Ex
> ```
> use std::thread;
> 
> thread::spawn(move || {
>     //some work here
> })
> ```

## `std`
- Available to all Rust programs by default

# Prelude
- List of things automatically imported into every Rust program
- Does not include the entire Rust Standard library
> For detail, see https://doc.rust-lang.org/std/prelude/index.html
