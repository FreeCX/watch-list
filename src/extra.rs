use parser;
use base;

use std::cmp;
use std::fmt;
use std::io::{self, Write};
use std::fs::File;

#[derive(Debug, Clone, Copy)]
pub enum ParamType {
    Status(base::Status),
    Progress(u16),
    Rate(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorStatus {
    IntParseError,
    EmptyFieldError,
    UnknownCommand,
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
    Error(ErrorStatus),
}

pub struct AnimeBase {
    pub list: Vec<base::Item>,
    pub name_len: usize,
    pub series_len: usize,
}

impl AnimeBase {
    pub fn new() -> AnimeBase {
        AnimeBase {
            list: Vec::new(),
            name_len: 0,
            series_len: 0,
        }
    }

    pub fn push(&mut self, item: base::Item) {
        self.name_len = cmp::max(item.name.len(), self.name_len);
        let cur_len = (f32::log10(cmp::max(item.maximum.get(), item.progress) as f32)).round() as
                      u16;
        self.series_len = cmp::max(self.series_len, cur_len as usize);
        self.list.push(item);
    }

    pub fn format(&self, item: &base::Item) -> String {
        let maximum = format!("{}", item.maximum);
        let status = format!("{}", item.status);
        format!("'{:>5$}', status: {:>8}, progress: {:>6$} / {:>6$}, rate: {:>2} / 10",
                item.name,
                status,
                item.progress,
                maximum,
                item.rate,
                self.name_len,
                self.series_len)
    }

    pub fn format_by_index(&self, index: usize) -> String {
        let item = self.list.get(index).unwrap();
        self.format(item)
    }

    pub fn write_to_file(&self, output: &mut File) -> Result<(), io::Error> {
        let mut result = String::new();
        for item in &(self.list) {
            result.push_str(&format!("\"{}\" {} progress {}/{} score {}\n",
                                     item.name,
                                     item.status,
                                     item.progress,
                                     item.maximum,
                                     item.rate));
        }
        write!(output, "{}", result)
    }
}

impl fmt::Display for AnimeBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for item in &(self.list) {
            let maximum = format!("{}", item.maximum);
            let f = format!("'{:>5$}', status: {}, progress: {:>6$} / {:>6$}, rate: {:>2} / 10\n",
                            item.name,
                            item.status,
                            item.progress,
                            maximum,
                            item.rate,
                            self.name_len,
                            self.series_len);
            result.push_str(&f);
        }
        write!(f, "{}", result)
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
                    match other.parse() {
                        Ok(value) => ExecCmd::Increment(value),
                        Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                    }
                } else {
                    ExecCmd::Increment(1)
                }
            }
            "-" => {
                if other.len() > 0 {
                    match other.parse() {
                        Ok(value) => ExecCmd::Decrement(value),
                        Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                    }
                } else {
                    ExecCmd::Decrement(1)
                }
            }
            "a" => {
                match iter.next() {
                    Some(new_name) => ExecCmd::Rename(new_name.to_owned()),
                    None => ExecCmd::Error(ErrorStatus::EmptyFieldError),
                }
            }
            "d" => ExecCmd::Delete,
            "i" => ExecCmd::Info,
            "f" => {
                if other.len() > 1 {
                    let (other, param) = other.split_at(1);
                    match other {
                        "s" => ExecCmd::FindParam(ParamType::Status(base::Status::from(param))),
                        "p" => {
                            match param.parse() {
                                Ok(value) => ExecCmd::FindParam(ParamType::Progress(value)),
                                Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                            }
                        }
                        "r" => {
                            match param.parse() {
                                Ok(value) => ExecCmd::FindParam(ParamType::Rate(value)),
                                Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                            }
                        }
                        _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
                    }
                } else {
                    match iter.next() {
                        Some(regex) => ExecCmd::Find(regex.to_owned()),
                        None => ExecCmd::Error(ErrorStatus::EmptyFieldError),
                    }
                }
            }
            "s" => {
                if other.len() >= 1 {
                    let (other, param) = other.split_at(1);
                    match other {
                        "m" => {
                            match param {
                                "?" => ExecCmd::Maximum(base::SeriesCounter::OnGoing),
                                _ => {
                                    match param.parse() {
                                        Ok(value) => {
                                            ExecCmd::Maximum(base::SeriesCounter::Value(value))
                                        }
                                        Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                                    }
                                }
                            }
                        }
                        "n" => {
                            match iter.next() {
                                Some(new_name) => ExecCmd::Rename(new_name.to_owned()),
                                None => ExecCmd::Error(ErrorStatus::EmptyFieldError),
                            }
                        }
                        "p" => {
                            match param.parse() {
                                Ok(value) => ExecCmd::Progress(value),
                                Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                            }
                        }
                        "r" => {
                            match param.parse() {
                                Ok(value) => ExecCmd::Rate(value),
                                Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                            }
                        }
                        "s" => ExecCmd::Status(base::Status::from(param)),
                        _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
                    }
                } else {
                    ExecCmd::Error(ErrorStatus::UnknownCommand)
                }
            }
            "w" => ExecCmd::Write,
            _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
        }
    }
}
