/// Simple macro to either get the value from an Option type or return from the current function.
/// Example:
/// ```
/// use early_returns::some_or_return;
/// fn do_something_with_option(i: Option<i32>) {
///     let i = some_or_return!(i);
///     println!("{i}");
/// }
/// ```
#[macro_export]
macro_rules! some_or_return {
    ($from:expr) => {{
        if let Some(f) = $from {
            f
        } else {
            return;
        }
    }};
}

/// Simple macro to either get the value from an Option type or break out of a loop. If a loop
/// lifetime is specified, that loop will be exited, otherwise the immediate loop is exited.
/// Example:
/// ```
/// use early_returns::some_or_break;
/// fn do_something_with_option(vals: &Vec<Option<i32>>) {
///     for val in vals {
///         let val = some_or_break!(val);
///         println!("{}", val);
///     }
///
///     'l: for val in vals {
///         for i in 0..5 {
///             let val = some_or_break!(val, 'l);
///             println!("{}", val + i);
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! some_or_break {
    ($from:expr) => {{
        if let Some(f) = $from {
            f
        } else {
            break;
        }
    }};

    ($from:expr, $lt:lifetime) => {{
        if let Some(f) = $from {
            f
        } else {
            break $lt;
        }
    }};
}

/// Simple macro to either get the value from an Option type or continue in a loop. If a loop lifetime
/// is specified, that loop will be "continued", otherwise the immediate loop is "continued".
/// Example:
/// ```
/// use early_returns::some_or_continue;
/// fn do_something_with_option(vals: &Vec<Option<i32>>) {
///     for val in vals {
///         let val = some_or_continue!(val);
///         println!("{}", val);
///     }
///
///     'l: for val in vals {
///         for i in 0..5 {
///             let val = some_or_continue!(val, 'l);
///             println!("{}", val + i);
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! some_or_continue {
    ($from:expr) => {{
        if let Some(f) = $from {
            f
        } else {
            continue;
        }
    }};

    ($from:expr, $lt:lifetime) => {{
        if let Some(f) = $from {
            f
        } else {
            continue $lt;
        }
    }};
}

/// Simple macro to either get the value from a Result type or return from the current function.
/// Usage:
/// ```
/// use early_returns::{ok_or_return, some_or_return};
/// fn do_something_with_result(i: Result<i32, ()>) {
///     let i = ok_or_return!(i);
/// }
/// ```
#[macro_export]
macro_rules! ok_or_return {
    ($from:expr) => {{
        if let Ok(f) = $from {
            f
        } else {
            return;
        }
    }};
}

/// Simple macro to either get the Ok value from a Result type or break out of a loop. If a loop
/// lifetime is specified, that loop will be exited, otherwise the immediate loop is exited.
/// Example:
/// ```
/// use early_returns::ok_or_break;
/// fn do_something_with_option(vals: &Vec<Result<i32, ()>>) {
///     for val in vals {
///         let val = ok_or_break!(val);
///         println!("{}", val);
///     }
///
///     'l: for val in vals {
///         for i in 0..5 {
///             let val = ok_or_break!(val, 'l);
///             println!("{}", val + i);
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! ok_or_break {
    ($from:expr) => {{
        if let Ok(f) = $from {
            f
        } else {
            break;
        }
    }};
    ($from:expr, $lt:lifetime) => {{
        if let Ok(f) = $from {
            f
        } else {
            break $lt;
        }
    }};
}

/// Simple macro to either get the value from a Result type or continue in a loop. If a loop lifetime
/// is specified, that loop will be "continued", otherwise the immediate loop is "continued".
/// Example:
/// ```
/// use early_returns::ok_or_continue;
/// fn do_something_with_option(vals: &Vec<Result<i32, ()>>) {
///     for val in vals {
///         let val = ok_or_continue!(val);
///         println!("{}", val);
///     }
///
///     'l: for val in vals {
///         for i in 0..5 {
///             let val = ok_or_continue!(val, 'l);
///             println!("{}", val + i);
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! ok_or_continue {
    ($from:expr) => {{
        if let Ok(f) = $from {
            f
        } else {
            continue;
        }
    }};
    ($from:expr, $lt:lifetime) => {{
        if let Ok(f) = $from {
            f
        } else {
            continue $lt;
        }
    }};
}

#[cfg(test)]
mod test {
    struct Tester {
        value: i32,
    }

    impl Tester {
        fn new() -> Tester {
            Tester { value: 0 }
        }

        fn increment_with_optional(&mut self, value: Option<i32>) {
            let value = some_or_return!(value);
            self.value += value;
        }

        fn increment_with_result(&mut self, value: Result<i32, ()>) {
            let value = ok_or_return!(value);
            self.value += value;
        }

        fn increment_with_optional_with_ref(&mut self, value: Option<&i32>) {
            let value = some_or_return!(value);
            self.value += value;
        }

        fn increment_with_result_with_ref(&mut self, value: Result<&i32, ()>) {
            let value = ok_or_return!(value);
            self.value += value;
        }

        fn increment_with_ref_to_optional(&mut self, value: &Option<i32>) {
            let value = some_or_return!(value);
            self.value += value;
        }

        fn increment_with_ref_to_result(&mut self, value: &Result<i32, ()>) {
            let value = ok_or_return!(value);
            self.value += value;
        }

        fn increment_with_optional_with_break(&mut self, values: Vec<Option<i32>>) {
            for value in values {
                let value = some_or_break!(value);
                self.value += value;
            }
        }

        fn increment_with_optional_with_break_with_lifetime(&mut self, values: Vec<Option<i32>>) {
            'l: for value in values {
                self.value += 1;
                for _i in 0..1 {
                    let value = some_or_break!(value, 'l);
                    self.value += value;
                }
            }
        }

        fn increment_with_optional_with_continue(&mut self, values: Vec<Option<i32>>) {
            for value in values {
                let value = some_or_continue!(value);
                self.value += value;
            }
        }

        fn increment_with_optional_with_continue_with_lifetime(
            &mut self,
            values: Vec<Option<i32>>,
        ) {
            'l: for value in values {
                self.value += 1;
                for _i in 0..1 {
                    let value = some_or_continue!(value, 'l);
                    self.value += value;
                }
            }
        }

        fn increment_with_result_with_break(&mut self, values: Vec<Result<i32, ()>>) {
            for value in values {
                let value = ok_or_break!(value);
                self.value += value;
            }
        }

        fn increment_with_result_with_break_with_lifetime(&mut self, values: Vec<Result<i32, ()>>) {
            'l: for value in values {
                self.value += 1;
                for _i in 0..1 {
                    let value = ok_or_break!(value, 'l);
                    self.value += value;
                }
            }
        }

        fn increment_with_result_with_continue(&mut self, values: Vec<Result<i32, ()>>) {
            for value in values {
                let value = ok_or_continue!(value);
                self.value += value;
            }
        }

        fn increment_with_result_with_continue_with_lifetime(
            &mut self,
            values: Vec<Result<i32, ()>>,
        ) {
            'l: for value in values {
                self.value += 1;
                for _i in 0..1 {
                    let value = ok_or_continue!(value, 'l);
                    self.value += value;
                }
            }
        }

        fn increment_with_optional_by_ref(
            &mut self,
            values: Option<i32>
        ) {

            let i: &i32 = some_or_return!(values.as_ref());
            self.value += i;
        }

        fn increment_with_optional_from_fn_result<F: Fn() -> Option<i32>>(
            &mut self,
            value_getter: F
        ) {

            let i: i32 = some_or_return!(value_getter());
            self.value += i;
        }
    }

    #[test]
    fn should_return_early_with_unengaged_optional() {
        let mut tester = Tester::new();
        tester.increment_with_optional(None);
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_not_return_early_with_engaged_optional() {
        let mut tester = Tester::new();
        tester.increment_with_optional(Some(1));
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_return_early_with_err() {
        let mut tester = Tester::new();
        tester.increment_with_result(Err(()));
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_not_return_early_with_ok() {
        let mut tester = Tester::new();
        tester.increment_with_result(Ok(1));
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_return_early_with_unengaged_optional_with_ref() {
        let mut tester = Tester::new();
        tester.increment_with_optional_with_ref(None);
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_not_return_early_with_engaged_optional_with_ref() {
        let mut tester = Tester::new();
        tester.increment_with_optional_with_ref(Some(&1));
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_return_early_with_err_with_ref() {
        let mut tester = Tester::new();
        tester.increment_with_result_with_ref(Err(()));
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_not_return_early_with_ok_with_ref() {
        let mut tester = Tester::new();
        tester.increment_with_result_with_ref(Ok(&1));
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_return_early_with_ref_to_unengaged_optional() {
        let mut tester = Tester::new();
        tester.increment_with_ref_to_optional(&None);
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_not_return_early_with_ref_to_engaged_optional() {
        let mut tester = Tester::new();
        tester.increment_with_ref_to_optional(&Some(1));
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_return_early_with_ref_to_err() {
        let mut tester = Tester::new();
        tester.increment_with_ref_to_result(&Err(()));
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_not_return_early_with_ref_to_ok() {
        let mut tester = Tester::new();
        tester.increment_with_ref_to_result(&Ok(1));
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_break_with_unengaged_optional() {
        let mut tester = Tester::new();
        tester.increment_with_optional_with_break(vec![None, Some(1)]);
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_break_with_unengaged_optional_with_lifetime() {
        let mut tester = Tester::new();
        tester.increment_with_optional_with_break_with_lifetime(vec![None, Some(1)]);
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_continue_with_unengaged_optional() {
        let mut tester = Tester::new();
        tester.increment_with_optional_with_continue(vec![None, Some(1)]);
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_continue_with_unengaged_optional_with_lifetime() {
        let mut tester = Tester::new();
        tester.increment_with_optional_with_continue_with_lifetime(vec![None, Some(1)]);
        assert_eq!(tester.value, 3);
    }

    #[test]
    fn should_break_with_err_result() {
        let mut tester = Tester::new();
        tester.increment_with_result_with_break(vec![Err(()), Ok(1)]);
        assert_eq!(tester.value, 0);
    }

    #[test]
    fn should_break_with_err_result_with_lifetime() {
        let mut tester = Tester::new();
        tester.increment_with_result_with_break_with_lifetime(vec![Err(()), Ok(1)]);
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_continue_with_err_result() {
        let mut tester = Tester::new();
        tester.increment_with_result_with_continue(vec![Err(()), Ok(1)]);
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_continue_with_err_result_with_lifetime() {
        let mut tester = Tester::new();
        tester.increment_with_result_with_continue_with_lifetime(vec![Err(()), Ok(1)]);
        assert_eq!(tester.value, 3);
    }

    #[test]
    fn should_get_optional_from_reference() {
        let mut tester = Tester::new();
        tester.increment_with_optional_by_ref(Some(1));
        assert_eq!(tester.value, 1);
        tester.increment_with_optional_by_ref(None);
        assert_eq!(tester.value, 1);
    }

    #[test]
    fn should_get_optional_from_function_result() {
        let mut tester = Tester::new();
        tester.increment_with_optional_from_fn_result(|| Some(1));
        assert_eq!(tester.value, 1);
        tester.increment_with_optional_from_fn_result(|| None);
        assert_eq!(tester.value, 1);
        let a = 1;
        tester.increment_with_optional_from_fn_result(|| Some(a));
        assert_eq!(tester.value, 2);
    }

    fn print_if_all_available_nested(a: Option<i32>, b: Option<i32>, c: Result<i32, ()>) {
        if let Some(a) = a {
            if let Some(b) = b {
                if let Ok(c) = c {
                    println!("{a} + {b} + {c} = {}", a + b + c);
                }
            }
        }
    }

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

    fn print_if_all_available_macro(a: Option<i32>, b: Option<i32>, c: Result<i32, ()>) {
        let a = some_or_return!(a);
        let b = some_or_return!(b);
        let c = ok_or_return!(c);

        println!("{a} + {b} + {c} = {}", a + b + c);
    }

    fn something(_v: &i32) {}

    fn do_something_with_vec_of_optionals(values: &Vec<Option<i32>>) {
        for value in values {
            let value = some_or_continue!(value);
            something(value);
        }
    }

    #[test]
    fn doc_examples_compile() {
        print_if_all_available_nested(Some(1), Some(2), Ok(3));
        print_if_all_available_verbose(Some(1), Some(2), Ok(3));
        print_if_all_available_macro(Some(1), Some(2), Ok(3));

        print_if_all_available_nested(None, None, Err(()));
        print_if_all_available_verbose(None, None, Err(()));
        print_if_all_available_macro(None, None, Err(()));

        do_something_with_vec_of_optionals(&vec![Some(1), Some(2), None, Some(3)]);
    }
}
