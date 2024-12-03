use std::str::FromStr;

pub trait Choices: Sized + FromStr {
    fn get_all_choices() -> Vec<String>;
    fn from_str(s: &str) -> Result<Self, String> {
        <Self as std::str::FromStr>::from_str(s)
            .map_err(|_| format!("Invalid input: `{}`", s))
    }
}
