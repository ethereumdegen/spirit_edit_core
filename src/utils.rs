

pub trait StringUtilsExt {
    fn ensure_ends_with(&self, suffix: &str) -> String;
}

impl StringUtilsExt for String {
    fn ensure_ends_with(&self, suffix: &str) -> String {
        if self.ends_with(suffix) {
            self.to_string()
        } else {
            format!("{}{}", self, suffix)
        }
    }
}
