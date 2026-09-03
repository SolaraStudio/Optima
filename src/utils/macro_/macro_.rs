#[macro_export]
macro_rules! try_opt {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => return None,
        }
    };
}

#[macro_export]
macro_rules! try_opt_ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(_) => return None,
        }
    };
}

#[macro_export]
macro_rules! try_result {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return Err(e.into()),
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return {
    ($expr:expr, $default:expr) => {
        match $expr {
            Some(val) => val,
            None => return $default,
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => continue,
        }
    };
}
