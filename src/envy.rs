use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const TEST_FILE: &str = ".env.test";
const DEBUG_FILE: &str = ".env.debug";
const RELEASE_FILE: &str = ".env";

#[derive(Debug, Clone)]
pub struct Envy {
    map: HashMap<String, String>,
}

impl Envy {
    pub fn debug() -> Result<Envy> {
        Ok(Envy {
            map: init_hashmap(DEBUG_FILE)?,
        })
    }
    pub fn release() -> Result<Envy> {
        Ok(Envy {
            map: init_hashmap(RELEASE_FILE)?,
        })
    }
    pub fn test() -> Result<Envy> {
        Ok(Envy {
            map: init_hashmap(TEST_FILE)?,
        })
    }
    pub fn detect() -> Result<Envy> {
        let filename = if cfg!(debug_assertions) {
            TEST_FILE
        } else if cfg!(test) {
            DEBUG_FILE
        } else {
            RELEASE_FILE
        };

        Ok(Envy {
            map: init_hashmap(filename)?,
        })
    }
    pub fn get(&self, key: &str) -> String {
        self.map.get(key).map_or_else(String::new, String::from)
    }
    pub fn amount(&self) -> usize {
        self.map.keys().len()
    }
    pub fn print_debug(&self) {
        for key in self.map.keys() {
            println!(
                "{} = {}",
                key,
                self.map.get(key).unwrap_or(&String::from(""))
            );
        }
    }
}

impl TryFrom<&'static str> for Envy {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &'static str) -> Result<Self> {
        Ok(Envy {
            map: init_hashmap(value)?,
        })
    }
}

fn init_hashmap(path: &str) -> Result<HashMap<String, String>> {
    let items = read_items(Path::new(path))?;

    let mut hashmap: HashMap<String, String> = HashMap::new();

    for item in items {
        hashmap.insert(item.0, item.1);
    }

    Ok(hashmap)
}

fn read_items<P>(path: P) -> Result<Vec<(String, String)>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.flatten().filter_map(parse_line).collect())
}

fn parse_line(mut line: String) -> Option<(String, String)> {
    if line.starts_with('#') {
        return None;
    }

    if !line.contains('=') {
        return None;
    }

    if line.contains(" #") {
        line = line.split_once(" #").unwrap().0.to_string();
    }

    line.split_once("=").and_then(|parts| {
        if parts.0.is_empty() || parts.1.is_empty() {
            return None;
        }
        Some((parts.0.trim().to_string(), parts.1.trim().to_string()))
    })
}
