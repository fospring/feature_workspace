#[cfg(feature = "fospring_vc")]
pub fn get_special_string() -> String {
    "feature fospring_vc on".to_string()
}

#[cfg(feature = "neo")]
pub fn get_special_string() -> String {
    "feature neo on".to_string()
}
