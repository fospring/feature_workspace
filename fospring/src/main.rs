pub fn main() {
    fospring_log::simple_logger::init().map_err(|err|{println!("log init error"); err}).ok();
    println!("show_special_string_from_dp: {}", show_special_string_from_dp());
    fospring_log::using_logging();
}

pub fn show_special_string_from_dp() -> String {
    fospring_view::get_special_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_show_string() {
        let s = show_special_string_from_dp();
        println!("s={:?}", s);
    }
}

