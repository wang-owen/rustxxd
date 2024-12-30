use crate::util;

pub struct XXDOptions {
    pub bits: bool,
    pub cols: Option<usize>,
    pub groupsize: Option<usize>,
    pub include: bool,
    pub len: Option<usize>,
    pub postscript: bool,
    pub revert: bool,
    pub uppercase: bool,
}

impl Default for XXDOptions {
    fn default() -> XXDOptions {
        Self {
            bits: false,
            cols: Some(16),
            groupsize: None,
            include: false,
            len: None,
            postscript: false,
            revert: false,
            uppercase: false,
        }
    }
}

pub fn match_option<T>(args: &[String], index: usize) -> Option<T>
where
    T: std::str::FromStr,
{
    match args.get(index) {
        Some(value) => match value.parse::<T>() {
            Ok(parsed_value) => Some(parsed_value),
            Err(_) => {
                util::help(1);
                None
            }
        },
        None => {
            util::help(1);
            None
        }
    }
}
