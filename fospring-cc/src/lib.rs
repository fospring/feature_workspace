extern {
    fn foo_function();
    fn double_function(x: i32) -> i32;
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

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test -p fospring-cc -v -- --nocapture double_works
    #[test]
    fn double_works() {
        let result = double(42);
        println!("got result:{:?}", result);
        assert_eq!(result, 84);
    }
}
