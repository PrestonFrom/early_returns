# early_returns
Macros to make early returns easier to work with in Rust

### Motivation
When working with optional values or result values, it can often be beneficial to code readability to use early returns to "bail out" if an option is not engaged (i.e. if it is `None`). However, if the function's return type is the unit type, the `?` operator cannot be used, which can lead to overly-verbose constructs like nested if-let blocks.

### What this crate provides
This crate hopes to make working with such types simpler by providing macros that will get the underlying type or return from the function immediately.

The macros are:
* `some_or_return`
  * Will "extract" a Some value if available *or* return from the current function.
* `ok_or_return`
  * Will "extract" an Ok value if available *or* return from the current function.

### Examples
The motivating example is something like this:
```rust
fn print_if_all_available_nested(a: Option<i32>, b: Option<i32>, c: Result<i32, ()>) {
    if let Some(a) = a {
        if let Some(b) = b {
            if let Ok(c) = c {
                println!("{a} + {b} + {c} = {}", a + b + c);
            }
        }
    }
}
```
As the nesting gets deeper or more complicated, it can be difficult for readers to follow. By returning early, readers can more easily follow the logic of the function, but there's no simple way to do this. For example, the following works and is easy to read, but includes a lot of boiler plate.
```rust
fn print_if_all_available_verbose(a: Option<i32>, b: Option<i32>, c: Result<i32, ()>) {
    let a = if let Some(a) = a {
        a
    } else {
        return;
    };
    
    let b = if let Some(b) = b {
        b
    } else {
        return;
    };
    
    let c = if let Ok(c) = c {
        c
    } else {
        return;
    };

    println!("{a} + {b} + {c} = {}", a + b + c);
}
```

This crate provides macros to reduce this boilerplate, so the above can be replaced with:
```rust
fn print_if_all_available_macro(a: Option<i32>, b: Option<i32>, c: Result<i32, ()>) {
    let a = some_or_return!(a);
    let b = some_or_return!(b);
    let c = ok_or_return!(c);

    println!("{a} + {b} + {c} = {}", a + b + c);
}
```
