# early_returns
Macros to make early returns and loop breaks/continues easier to work with in Rust

### Motivation

When working with Option or Result values, it can often be beneficial to code readability to use early returns to "bail out" if an option is not engaged (e.g. if it is `None` or an error). However, if the function's return type is the unit type, the `?` operator cannot be used, which can lead to overly-verbose constructs like nested if-let blocks.

Additionally, the `?` operator cannot be used in loops, where it might be useful to break or continue if an option is `None` or a result is `Err`.

#### Are early returns actually good?

In my opinion, yes, but there certainly seems to be a lot of debate around this!

Very briefly, I like early returns because:
1. It makes the requirements/preconditions obvious to readers up front.
   1. This seems like it should reduce cognitive overhead and makes it easier for readers to follow the core logic.
   2. Ideally, this helps with "chunking", so readers can look at the flow in discrete parts.
2. It reduces excessive indentation.
   1. This is probably a purely personal preference, but as code indentations grow larger, I find it more difficult to follow.

#### Should you actually use this?

Potentially this won't be very useful for much longer -- the [`let-else` RFC](https://github.com/rust-lang/rust/issues/87335) looks like it will implement a language feature to accomplish everything the macros here provide. Until then, this might be useful for you!

Personally, I would like the `?` operator to work for the unit type.

#### Why shouldn't you use this?

It's potentially very confusing the first time someone sees one of these macros -- you either need to trust the name or look at the actual macros to see that the return/break/continue. When introducing them to an established code base, you run the risk of confusing others, so if you are thinking about adding them, definitely talk to your team first.

### What this crate provides
This crate hopes to make working with such types simpler by providing macros that will get the underlying type or return from the function or break from/continue in loops immediately.

The macros for Option are:
* `some_or_return`
  * Will "extract" a `Some` value if available *or* return from the current function. (Can also return a default value.)
* `some_or_break`
  * Will "extract" a `Some` value if available *or* break from either the current loop (if no loop lifetime is specified) or the specified loop (if a loop lifetime is specified).
* `some_or_continue`
  * Will "extract" a `Some` value if available *or* continue either the current loop (if no loop lifetime is specified) or the specified loop (if a loop lifetime is specified).

The macros for Result are:
* `ok_or_return`
  * Will "extract" an `Ok` value if available *or* return from the current function. (Can also return a default value.)
* `ok_or_break`
  * Will "extract" an `Ok` value if available *or* break from either the current loop (if no loop lifetime is specified) or the specified loop (if a loop lifetime is specified).
* `ok_or_continue`
  * Will "extract" an `Ok` value if available *or* continue either the current loop (if no loop lifetime is specified) or the specified loop (if a loop lifetime is specified).

### Examples
#### Early return from a function 
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
use early_returns::{some_or_return, ok_or_return};
fn print_if_all_available_macro(a: Option<i32>, b: Option<i32>, c: Result<i32, ()>) {
    let a = some_or_return!(a);
    let b = some_or_return!(b);
    let c = ok_or_return!(c);

    println!("{a} + {b} + {c} = {}", a + b + c);
}
```

#### Continuing in a loop
Similarly, there are macros that can be used to break or continue in loops. For example, this:
```rust
fn something(_v: &i32) {}

fn do_something_with_vec_of_optionals(values: &Vec<Option<i32>>) {
  for value in values.iter() {
    let value = if let Some(value) = value {
      value
    } else {
      continue;
    };
    something(value);
  }
}
```

can be reduced to this:

```rust
use early_returns::some_or_continue;

fn something(_v: &i32) {}

fn do_something_with_vec_of_optionals(values: &Vec<Option<i32>>) {
  for value in values.iter() {
    let value = some_or_continue!(value);
    something(value);
  }
}
```
