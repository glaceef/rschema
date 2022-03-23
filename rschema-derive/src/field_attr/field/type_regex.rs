use once_cell::sync::Lazy;
use regex::Regex;

macro_rules! regex {
    ($var:ident = $re:literal) => {
        pub static $var: Lazy<Regex> = Lazy::new(||{
            Regex::new(&format!("^({0}|Box<{0}>)$", $re)).unwrap()
        });
    }
}

regex!(STRING_TYPE_REGEX = "String|str");

regex!(NUMBER_TYPE_REGEX = "[iu]((8|16|32|64|128)|size)");

// vector / slice / boxed-slice
regex!(ARRAY_TYPE_REGEX = r"(Vec<.+>|\[.+;\d+\]|Box<\[.+\]>)");

regex!(BOOLEAN_TYPE_REGEX = "bool");