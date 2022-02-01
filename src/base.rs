use crate::extra::ErrorStatus;
use crate::parser;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Complete,
    Drop,
    Plan,
    Watch,
    Hold,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeriesCounter {
    Value(u16),
    OnGoing,
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub status: Status,
    pub progress: u16,
    pub maximum: SeriesCounter,
    pub rate: u8,
}

impl<'a> From<&'a str> for Status {
    fn from(s: &'a str) -> Status {
        match s {
            "complete" | "c" => Status::Complete,
            "drop" | "d" => Status::Drop,
            "plan" | "p" => Status::Plan,
            "watch" | "w" => Status::Watch,
            "hold" | "h" => Status::Hold,
            _ => Status::Error,
        }
    }
}

impl FromStr for SeriesCounter {
    type Err = ErrorStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(SeriesCounter::OnGoing),
            _ => match s.parse() {
                Ok(value) => Ok(SeriesCounter::Value(value)),
                Err(_) => Err(ErrorStatus::IntParseError),
            },
        }
    }
}

impl SeriesCounter {
    pub fn get(self) -> u16 {
        match self {
            SeriesCounter::Value(value) => value,
            SeriesCounter::OnGoing => 0,
        }
    }
}

impl Item {
    // TODO:
    // - rewrite this code
    // - fix (...).parse().unwrap()
    pub fn new(text: &str) -> Item {
        let raw: Vec<_> = parser::Splitter::new(text, parser::SplitFormat::Anime).collect();
        let progress: Vec<_> = raw[3].split('/').collect();
        let maximum = match progress[1] {
            "?" => SeriesCounter::OnGoing,
            value => SeriesCounter::Value(value.parse().unwrap()),
        };
        Item {
            name: raw[0].to_owned(),
            status: Status::from(raw[1]),
            progress: progress[0].parse().unwrap(),
            maximum,
            rate: raw[5].parse().unwrap(),
        }
    }

    pub fn empty(name: &str) -> Item {
        Item { name: name.to_owned(), status: Status::Plan, progress: 0, maximum: SeriesCounter::OnGoing, rate: 0 }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "'{:>16}', status: {:?}, progress: {:>2} / {:?}, rate: {}",
            self.name, self.status, self.progress, self.maximum, self.rate
        )
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Status::Complete => "complete",
                Status::Drop => "drop",
                Status::Plan => "plan",
                Status::Watch => "watch",
                Status::Hold => "hold",
                Status::Error => "<error>",
            }
        )
    }
}

impl fmt::Display for SeriesCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SeriesCounter::Value(value) => write!(f, "{}", value),
            SeriesCounter::OnGoing => write!(f, "?"),
        }
    }
}
