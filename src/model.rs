#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// viewed
    Complete,
    /// dropped from viewing
    Drop,
    #[default]
    /// plan to watch
    Plan,
    /// now watching
    Watch,
    /// viewing on hold
    Hold,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    /// title
    pub name: String,
    /// viewing status
    pub status: Status,
    /// number of episodes watched
    pub watched: u16,
    /// total number of episodes
    pub episodes: u16,
    /// title rating
    pub rate: u8,
}

// TODO: try_from
impl From<&str> for Status {
    fn from(value: &str) -> Status {
        match value {
            "complete" | "c" => Status::Complete,
            "drop" | "d" => Status::Drop,
            "plan" | "p" => Status::Plan,
            "watch" | "w" => Status::Watch,
            "hold" | "h" => Status::Hold,
            _ => Status::Plan,
        }
    }
}
