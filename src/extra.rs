use parser;
use base;

use std::cmp;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ParamType {
    Status(base::Status),
    Progress(u16),
    Rate(u8)
}

#[derive(Debug, Clone)]
pub enum ExecCmd {
    Increment(u16),
    Decrement(u16),
    Append(String),
    Delete,
    Info,
    Find(String),
    FindParam(ParamType),
    Maximum(base::SeriesCounter),
    Rename(String),
    Progress(u16),
    Status(base::Status),
    Rate(u8),
    Write,
    Error
}

pub struct AnimeBase {
    pub list: Vec<base::Item>,
    pub name_len: usize,
    pub series_len: usize,
}

impl AnimeBase {
    pub fn new() -> AnimeBase {
        AnimeBase { list: Vec::new(), name_len: 0, series_len: 0 }
    }
    pub fn push(&mut self, item: base::Item) {
        self.name_len = cmp::max(item.name.len(), self.name_len);
        let cur_len = (f32::log10(cmp::max(item.maximum.get(), item.progress) as f32)).round() as u16;
        self.series_len = cmp::max(self.series_len, cur_len as usize);
        self.list.push(item);
    }
}

impl fmt::Display for AnimeBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for item in &(self.list) {
            let maximum = format!("{}", item.maximum);
            result.push_str(&format!("'{:>5$}', status: {}, progress: {:>6$} / {:>6$}, rate: {:>2} / 10\n",
               item.name, item.status, item.progress, maximum, item.rate, self.name_len, self.series_len));
        }
        write!(f, "{}", result.trim())
    }
}

// TODO:
// - rewrite this code
// FIX:
// - (...).parse().unwrap()
// - (...).unwrap()
impl ExecCmd {
    pub fn get(cmd: &str, iter: &mut parser::Splitter) -> ExecCmd {
        let (cmd, other) = cmd.split_at(1);
        match cmd {
            "+" => {
                if other.len() > 0 {
                    ExecCmd::Increment(other.parse().unwrap())
                } else {
                    ExecCmd::Increment(1)
                }
            },
            "-" => {
                if other.len() > 0 {
                    ExecCmd::Decrement(other.parse().unwrap())
                } else {
                    ExecCmd::Decrement(1)
                }
            },
            "a" => {
                let new_name = iter.next().unwrap();
                ExecCmd::Rename(new_name.to_owned())
            },
            "d" => ExecCmd::Delete,
            "i" => ExecCmd::Info,
            "f" => {
                let regex = iter.next().unwrap();
                ExecCmd::Find(regex.to_owned())
            },
            "g" => {
                let (other, param) = other.split_at(1);
                match other {
                    "s" => ExecCmd::FindParam(ParamType::Status(base::Status::from(param))),
                    "p" => ExecCmd::FindParam(ParamType::Progress(param.parse().unwrap())),
                    "r" => ExecCmd::FindParam(ParamType::Rate(param.parse().unwrap())),
                    _ => ExecCmd::Error
                }
            },
            "m" => {
                match other {
                    "?" => ExecCmd::Maximum(base::SeriesCounter::OnGoing),
                    _ => ExecCmd::Maximum(base::SeriesCounter::Value(other.parse().unwrap()))
                }
            },
            "n" => {
                let new_name = iter.next().unwrap();
                ExecCmd::Rename(new_name.to_owned())
            },
            "p" => {
                let value = other.parse().unwrap();
                ExecCmd::Progress(value)
            },
            "s" => ExecCmd::Status(base::Status::from(other)),
            "r" => {
                let value = other.parse().unwrap();
                ExecCmd::Rate(value)
            },
            "w" => ExecCmd::Write,
            _ => ExecCmd::Error
        }
    }
}