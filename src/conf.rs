use std::fmt::{self, Display, Formatter};
use std::ops::Index;
use std::str::FromStr;
use std::{collections::HashMap, fs};

const DEFAULT_DELIM: char = ':';

/// Conf is more or less a wrapper around `HashMap`<String, String>, and it controls access to (key, value) pairs, which
/// represent configuration properties for an application and their respective values. It offers methods
/// to ergonomically and safely parse a configuration file and update the defaults previously set by the user.
///

#[derive(Debug)]
pub struct Conf {
    pairs: HashMap<String, String>,
    delim: Option<char>,
    conf_file_name: String,
    updated: bool,
    empty_string: String,
}

impl Conf {
    /// Creates a Conf, given user defaults
    ///
    /// # Examples
    ///
    /// ```
    /// let mut conf = Conf::from([
    ///     ("foo".to_string(), "bar".to_string()),
    ///     ("yee".to_string(), "haw".to_string()),
    /// ]);
    /// ```
    #[must_use]
    pub fn from<const N: usize>(defaults: [(String, String); N]) -> Self {
        Self {
            pairs: HashMap::from(defaults),
            delim: None,
            conf_file_name: String::new(),
            empty_string: String::new(),
            updated: false,
        }
    }

    /// Sets the delimiter for this Conf
    pub fn with_delim(&mut self, delim: char) -> &mut Self {
        self.delim = Some(delim);
        self
    }
    pub fn and_delim(&mut self, delim: char) -> &mut Self {
        self.with_delim(delim)
    }
    /// Gets the delimiter set for this Conf
    #[must_use]
    pub fn delim(&self) -> char {
        self.delim.unwrap_or(DEFAULT_DELIM)
    }

    /// Sets the configuration file name for this Conf
    pub fn with_file(&mut self, conf_file_name: &str) -> &mut Self {
        self.conf_file_name = conf_file_name.to_string();
        self
    }
    pub fn and_file(&mut self, conf_file_name: &str) -> &mut Self {
        self.with_file(conf_file_name)
    }
    /// Gets the configuration file name set for this Conf
    #[must_use]
    pub fn file(&self) -> &String {
        &self.conf_file_name
    }

    /// Updates Conf with new values, given the file name has been set
    ///
    /// # Errors
    ///     This function could return errors if no lines are read from
    ///     file, or if the delimeter is not found.
    /// # Examples
    ///
    /// ```
    /// let mut conf = Conf::from([
    ///     ("foo".to_string(), "bar".to_string()),
    ///     ("yee".to_string(), "haw".to_string()),
    /// ]);
    /// match conf.with_file(conf_file_name).update() {
    ///     Ok(_) => println!("Successfully updated configuration!"),
    ///     Err(e) => panic!("Error updating configuration: {}", e),
    /// }
    /// ```
    pub fn update(&mut self) -> Result<(), String> {
        let lines = self.read_lines()?;
        for line in lines {
            let i = line
                .find(self.delim.unwrap_or(DEFAULT_DELIM))
                .ok_or_else(|| format!("No delimiter found in line: {line}"))?;
            let key = line[..i].trim();
            let value = line[i + 1..].trim();
            self.pairs
                .entry(key.to_string())
                .and_modify(|v| *v = value.to_string());
        }
        self.updated = true;
        Ok(())
    }
    fn read_lines(&self) -> Result<Vec<String>, String> {
        let contents = fs::read_to_string(&self.conf_file_name).map_err(|e| e.to_string())?;
        Ok(contents.lines().map(String::from).collect())
    }

    /// Gets the update status for this Conf
    #[must_use]
    pub fn is_updated(&self) -> bool {
        self.updated
    }

    /// Function to index into Conf, and attempt type conversion.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut conf = Conf::from([
    ///     ("addr".to_string(), "127.0.0.1".to_string()),
    ///     ("port".to_string(), "8080".to_string()),
    ///     ("is_valid".to_string(), "true".to_string()),
    /// ]);
    /// let addr: IpAddr = conf.get("addr").unwrap();
    /// let port = conf.get::<u16>("port").unwrap();
    /// let is_valid: bool = conf.get("is_valid").unwrap();
    /// ```
    #[must_use]
    pub fn get<T: FromStr>(&self, key: &str) -> Option<T> {
        self.pairs.get(key).and_then(|v| v.parse::<T>().ok())
    }
}

/// Allows for the use of [ ]. Occasionally useful
///
/// # Examples
///
/// ```
/// let mut conf = Conf::from([
///     ("foo".to_string(), "bar".to_string()),
///     ("yee".to_string(), "haw".to_string()),
/// ]);
/// println!("{}", conf["foo"]);
/// ```
impl Index<&str> for Conf {
    type Output = String;

    fn index(&self, key: &str) -> &Self::Output {
        self.pairs.get(key).unwrap_or(&self.empty_string)
    }
}

/// Displays the config file as confee would expect to read it
///
/// # Examples
///
/// ```
/// let mut conf = Conf::from([
///     ("foo".to_string(), "bar".to_string()),
///     ("yee".to_string(), "haw".to_string()),
/// ]);
/// println!("{}", conf);
/// ```
impl Display for Conf {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (key, value) in &self.pairs {
            let formatted_value = if value.is_empty() {
                &self.empty_string
            } else {
                value
            };
            writeln!(f, "{}{} {}", key, self.delim(), formatted_value)?;
        }
        Ok(())
    }
}
