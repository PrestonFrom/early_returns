/// Simple macro to either get the value from an Option type or return from the current function.
/// Usage:
/// ```
/// use early_returns::some_or_return;
/// fn do_something_with_option(i: Option<i32>) {
///     let i = some_or_return!(i);
/// }
/// ```
#[macro_export]
macro_rules! some_or_return {
    ($from:ident) => {{
        if let Some(f) = $from {
            f
        } else {
            return;
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
    ($from:ident) => {{
        if let Ok(f) = $from {
            f
        } else {
            return;
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

    #[test]
    fn doc_examples_compile() {
        print_if_all_available_nested(Some(1), Some(2), Ok(3));
        print_if_all_available_verbose(Some(1), Some(2), Ok(3));
        print_if_all_available_macro(Some(1), Some(2), Ok(3));

        print_if_all_available_nested(None, None, Err(()));
        print_if_all_available_verbose(None, None, Err(()));
        print_if_all_available_macro(None, None, Err(()));
    }
}
