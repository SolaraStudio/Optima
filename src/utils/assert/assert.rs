pub fn assert(condition: bool, message: &str) {
    if !condition {
        panic!("{}", message);
    }
}

pub fn assert_eq<T: PartialEq + std::fmt::Debug>(a: &T, b: &T, msg: &str) {
    if a != b {
        panic!("Assertion failed: {:?} != {:?} - {}", a, b, msg);
    }
}

pub fn assert_ne<T: PartialEq + std::fmt::Debug>(a: &T, b: &T, msg: &str) {
    if a == b {
        panic!("Assertion failed: {:?} == {:?} - {}", a, b, msg);
    }
}

pub fn assert_ok<T, E: std::fmt::Debug>(result: Result<T, E>, msg: &str) -> T {
    match result {
        Ok(v) => v,
        Err(e) => panic!("Expected Ok, got Err: {:?} - {}", e, msg),
    }
}

pub fn assert_err<T: std::fmt::Debug, E: std::fmt::Debug>(result: Result<T, E>, msg: &str) -> E {
    match result {
        Ok(v) => panic!("Expected Err, got Ok: {:?} - {}", v, msg),
        Err(e) => e,
    }
}
