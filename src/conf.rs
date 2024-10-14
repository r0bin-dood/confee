use std::fmt::{Display, Formatter, Result};
use std::ops::Index;
use std::str::FromStr;
use std::{collections::HashMap, fs};

const DEFAULT_DELIM: char = ':';

#[derive(Debug)]
pub struct Conf {
    pairs: HashMap<String, String>,
    delim: Option<char>,
    empty_string: String,
}

impl Conf {
    pub fn from<const N: usize>(defaults: [(String, String); N]) -> Self {
        Self {
            pairs: HashMap::from(defaults),
            delim: None,
            empty_string: "".to_string(),
        }
    }

    pub fn with_delim(&mut self, delim: char) -> &mut Self {
        self.delim = Some(delim);
        self
    }

    pub fn update(&mut self, conf_file: &str) {
        let lines = Self::read_lines(conf_file);
        for line in lines {
            let i = line
                .find(self.delim.unwrap_or(DEFAULT_DELIM))
                .expect("Bad line in configuration file");
            let key = line[..i].trim();
            let value = line[i + 1..].trim();
            self.pairs
                .entry(key.to_string())
                .and_modify(|v| *v = value.to_string());
        }
    }
    fn read_lines(file_name: &str) -> Vec<String> {
        fs::read_to_string(file_name)
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    }

    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.pairs.get(key).and_then(|v| v.parse::<T>().ok())
    }
}

impl Index<&str> for Conf {
    type Output = String;

    fn index(&self, key: &str) -> &Self::Output {
        self.pairs.get(key).unwrap_or(&self.empty_string)
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (key, value) in &self.pairs {
            let formatted_value = if value.is_empty() {
                &self.empty_string
            } else {
                value
            };
            writeln!(
                f,
                "{}{} {}",
                key,
                self.delim.unwrap_or(DEFAULT_DELIM),
                formatted_value
            )?;
        }
        Ok(())
    }
}
