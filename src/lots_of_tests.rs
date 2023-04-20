mod use_conditional_compilation {
    #[test]
    fn always_pass() {
        assert!(true)
    }
}

#[cfg(test)]
mod tests {
    use std::num::{IntErrorKind, ParseIntError};

    #[test]
    fn normal_test() {
        let value = Some(5);
        assert!(value.is_some())
    }

    #[test]
    fn a_failing_test() {
        let value = None;
        assert_eq!(Some(5), value);
    }

    #[test]
    fn returning_result() -> Result<(), ParseIntError> {
        let number_5 = "5".parse::<u64>()?;
        assert_eq!(5, number_5);
        Ok(())
    }

    #[test]
    fn results_lose_context() -> Result<(), ParseIntError> {
        let bad_number = "five".parse::<u64>()?;
        let also_bad_number = "six".parse::<u64>()?;
        assert_eq!(5, bad_number);
        assert_eq!(6, also_bad_number);
        Ok(())
    }

    #[test]
    fn unwrapping_and_expecting_gives_backtrace() {
        let bad_number = "five".parse::<u64>().unwrap();
        let also_bad_number = "six".parse::<u64>().unwrap();
        assert_eq!(5, bad_number);
        assert_eq!(6, also_bad_number);
    }

    #[test]
    fn of_course_checking_for_errors_is_fine() {
        let parse_result = "five".parse::<u64>();
        assert!(parse_result.is_err());
        // Or directly unwrap
        let err = parse_result.unwrap_err();
        assert_eq!(&IntErrorKind::InvalidDigit, err.kind());
    }

    #[test]
    #[should_panic]
    fn testing_for_expected_panics() {
        let _ = "5".parse::<u64>().unwrap_err();
    }

    #[test]
    #[should_panic = "invariant violated. Roses should be red"]
    fn for_panics_with_a_descriptive_message() {
        panic!("invariant violated. Roses should be red")
    }
}
