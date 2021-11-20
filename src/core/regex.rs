use regex::{Captures, Regex};

pub struct RegEx {
    value: Option<Regex>
}

pub struct RegExMatch<'a> {
    captures: Captures<'a>,
}

impl RegEx {
    pub fn new(pattern: &str) -> Self {
        let value = Regex::new(pattern).ok();

        RegEx {
            value: value
        }
    }

    pub fn search<'a>(&self, text: &'a str) -> Option<RegExMatch<'a>> {
        let value = self.value.as_ref()?;
        let captures = value.captures(&text)?;
        let regex_match: RegExMatch = RegExMatch { captures };

        Some(regex_match)
    }

    pub fn split<'a>(&self, text: &'a str) -> Option<Vec<&'a str>> {
        let value = self.value.as_ref()?;

        Some(value.split(text).collect())
    }
}

impl RegExMatch<'_> {
    pub fn get_string(&self, group: usize) -> String {
        return self.captures[group].to_string();
    }

    pub fn get_i32(&self, group: usize) -> Option<i32> {
        self.get_string(group).parse::<i32>().ok()
    }

    pub fn get_u8(&self, group: usize) -> Option<u8> {
        self.get_string(group).parse::<u8>().ok()
    }

    pub fn get_usize(&self, group: usize) -> Option<usize> {
        self.get_string(group).parse::<usize>().ok()
    }
}
