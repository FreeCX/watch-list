use std::fmt;

#[derive(Clone)]
pub enum ListStatus {
    Error,
    Complete,
    Drop,
    Plan,
    Watch,
    Hold,
}

#[derive(Clone)]
pub struct Database {
    pub name: String,
    pub status: ListStatus,
    pub current: u32,
    pub maximum: u32,
    pub score: u8,
}

impl fmt::Display for ListStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListStatus::Complete => f.write_str("complete"),
            ListStatus::Drop => f.write_str("drop"),
            ListStatus::Plan => f.write_str("plan"),
            ListStatus::Watch => f.write_str("watch"),
            ListStatus::Hold => f.write_str("hold"),
            ListStatus::Error => f.write_str("<error>"),
        }
    }
}

impl fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "'{:>16}', status: {:>8}, progress: {:>2} / {:>2}, score: {}",
               self.name,
               self.status.to_string(),
               self.current,
               self.maximum,
               self.score)
    }
}
