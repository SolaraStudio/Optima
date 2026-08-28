#[macro_export]
macro_rules! assert_ok {
    ($expr:expr) => {
        assert!($expr.is_ok(), "Expected Ok, got Err: {:?}", $expr.err())
    };
}

#[macro_export]
macro_rules! assert_err {
    ($expr:expr) => {
        assert!($expr.is_err(), "Expected Err, got Ok: {:?}", $expr.ok())
    };
}

pub fn assert_eq<T: PartialEq + std::fmt::Debug>(a: &T, b: &T, msg: &str) {
    assert_eq!(a, b, "{}", msg);
}

pub fn assert_ne<T: PartialEq + std::fmt::Debug>(a: &T, b: &T, msg: &str) {
    assert_ne!(a, b, "{}", msg);
}
