pub fn main() {
    println!("show_special_string_from_dp: {}", show_special_string_from_dp());
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

