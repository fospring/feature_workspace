use anyhow::Error as AnyErr;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct FospringErr {
    code: i32,
    msg: String,
}

impl Display for FospringErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FospringErr{{code:{:?}, msg:{:?}}}", self.code, self.msg)
    }
}

impl std::error::Error for FospringErr {}

fn is_fospring_err(err: &AnyErr) -> bool {
    for cause in err.chain() {
        if let Some(fospring_err) = cause.downcast_ref::<FospringErr>() {
            println!("code: {:?}, msg: {:?}", fospring_err.code, fospring_err.msg);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_downcast_ref_error() {
        let err = FospringErr{code: 10, msg: "hello!".to_string()};
        let any_err= anyhow::anyhow!(err);
        let is_fospring = is_fospring_err(&any_err);
        assert!(is_fospring);
    }
}
