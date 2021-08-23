use chrono::prelude::{DateTime, Local, Timelike, Utc};

extern {
    fn foo_function();
    fn double_function(x: i32) -> i32;
    fn current_time_millis_func() -> i64;
}

pub fn call() {
    unsafe {
        foo_function();
        double_function(42);
    }
}

pub fn double(value: i32) -> i32 {
    unsafe {
        double_function(value)
    }
}

pub fn current_time_millis_cpp() -> i64 {
    unsafe {
        current_time_millis_func()
    }
}

pub fn current_time_millis_rust() -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() * 1000 + i64::from(utc.timestamp_subsec_millis())
}

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test -p fospring-cc -v -- --nocapture time_works
    #[test]
    fn time_works() {
        let current_time_ms_from_cpp = current_time_millis_cpp();
        println!("current_time_ms cpp:{:?}", current_time_ms_from_cpp);
        let current_time_millis_from_rust = current_time_millis_rust();
        println!("current_time_ms rust:{:?}", current_time_millis_from_rust);
    }

    // cargo test -p fospring-cc -v -- --nocapture double_works
    #[test]
    fn double_works() {
        let result = double(42);
        println!("got result:{:?}", result);
        assert_eq!(result, 84);
    }
}
