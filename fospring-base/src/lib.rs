pub mod error_helper;
mod errors;
use errors::*;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("error:{:?}", foo());
    }
    fn foo() -> Result<()> {
        Err("foo error!".into())
    }
}
