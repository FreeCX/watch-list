use crate::base;
use crate::parser;

use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum ParamType {
    Status(base::Status),
    Progress(u16),
    Maximum(base::SeriesCounter),
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
    FilterParam(ParamType),
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
        AnimeBase { list: Vec::new(), name_len: 0, series_len: 0 }
    }

    pub fn push(&mut self, item: base::Item) {
        self.name_len = cmp::max(item.name.len(), self.name_len);
        let curr = (f32::log10(cmp::max(item.maximum.get(), item.progress) as f32)).round() as u16;
        self.series_len = cmp::max(self.series_len, curr as usize);
        self.list.push(item);
    }

    pub fn append(&mut self, name: &str) -> usize {
        self.push(base::Item::empty(name));
        self.list.len() - 1
    }

    pub fn format(&self, item: &base::Item) -> String {
        format!(
            "'{:>5$}', status: {:>8}, progress: {:>6$} / {:>6$}, rate: {:>2} / 10",
            item.name, item.status, item.progress, item.maximum, item.rate, self.name_len, self.series_len
        )
    }

    pub fn format_by_index(&self, index: usize) -> String {
        let item = self.list.get(index).unwrap();
        self.format(item)
    }

    pub fn write_to_file(&self, output: &mut File) -> Result<(), io::Error> {
        let mut result = String::new();
        for item in &(self.list) {
            result.push_str(&format!(
                "\"{}\" {} progress {}/{} score {}\n",
                item.name, item.status, item.progress, item.maximum, item.rate
            ));
        }
        write!(output, "{}", result)
    }

    pub fn get_item(&self, index: usize) -> Option<&base::Item> {
        self.list.get(index)
    }

    fn set_item<'a, F>(&'a mut self, index: usize, cond: F) -> Option<()>
    where
        F: FnOnce(&'a mut base::Item) -> Option<()>,
    {
        self.list.get_mut(index).and_then(cond)
    }

    pub fn set_maximum(&mut self, index: usize, value: base::SeriesCounter) -> Option<()> {
        self.set_item(index, |f| {
            f.maximum = value;
            if let base::SeriesCounter::Value(max) = f.maximum {
                if f.progress >= max {
                    f.status = base::Status::Complete;
                } else {
                    f.status = base::Status::Watch;
                }
            }
            Some(())
        })
    }

    fn update_progress_status(f: &'_ mut base::Item) {
        if let base::SeriesCounter::Value(max) = f.maximum {
            if f.progress >= max {
                f.progress = max;
                f.status = base::Status::Complete;
            } else {
                f.status = base::Status::Watch;
            }
        }
    }

    pub fn set_progress(&mut self, index: usize, value: u16) -> Option<()> {
        self.set_item(index, |f| {
            f.progress = value;
            AnimeBase::update_progress_status(f);
            Some(())
        })
    }

    pub fn set_status(&mut self, index: usize, status: base::Status) -> Option<()> {
        self.set_item(index, |f| {
            f.status = status;
            if status == base::Status::Complete {
                if let base::SeriesCounter::Value(max) = f.maximum {
                    f.progress = max
                }
            }
            Some(())
        })
    }

    pub fn set_rate(&mut self, index: usize, value: u8) -> Option<()> {
        self.set_item(index, |f| {
            f.rate = value;
            Some(())
        })
    }

    pub fn set_name(&mut self, index: usize, name: &str) -> Option<()> {
        self.set_item(index, |f| {
            f.name = name.to_string();
            Some(())
        })
    }

    pub fn progress_increment_by(&mut self, index: usize, value: u16) -> Option<()> {
        self.set_item(index, |f| {
            f.progress = f.progress.saturating_add(value);
            AnimeBase::update_progress_status(f);
            Some(())
        })
    }

    pub fn progress_decrement_by(&mut self, index: usize, value: u16) -> Option<()> {
        self.set_item(index, |f| {
            f.progress = f.progress.saturating_sub(value);
            AnimeBase::update_progress_status(f);
            Some(())
        })
    }
}

impl fmt::Display for AnimeBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for item in &(self.list) {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&self.format(item));
        }
        write!(f, "{}", result)
    }
}

impl ExecCmd {
    pub fn get(cmd: &str, iter: &mut parser::Splitter) -> ExecCmd {
        let (cmd, other) = cmd.split_at(1);
        match cmd {
            // Increment series progress
            "+" => {
                if !other.is_empty() {
                    // by value
                    match other.parse() {
                        Ok(value) => ExecCmd::Increment(value),
                        Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                    }
                } else {
                    // by 1
                    ExecCmd::Increment(1)
                }
            }
            // decrement series progress
            "-" => {
                if !other.is_empty() {
                    // by value
                    match other.parse() {
                        Ok(value) => ExecCmd::Decrement(value),
                        Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                    }
                } else {
                    // by 1
                    ExecCmd::Decrement(1)
                }
            }
            // append new item
            "a" => match iter.next() {
                Some(new_name) => ExecCmd::Append(new_name.to_owned()),
                None => ExecCmd::Error(ErrorStatus::EmptyFieldError),
            },
            // delete
            "d" => ExecCmd::Delete,
            // info
            "i" => ExecCmd::Info,
            // find
            "f" => {
                if other.len() > 1 {
                    let (other, param) = other.split_at(1);
                    match other {
                        // by status
                        "s" => ExecCmd::FindParam(ParamType::Status(base::Status::from(param))),
                        // by progress
                        "p" => match param.parse() {
                            Ok(value) => ExecCmd::FindParam(ParamType::Progress(value)),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // by series maximum
                        "m" => match param.parse() {
                            Ok(value) => ExecCmd::FindParam(ParamType::Maximum(value)),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // by rate
                        "r" => match param.parse() {
                            Ok(value) => ExecCmd::FindParam(ParamType::Rate(value)),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
                    }
                } else {
                    // by name
                    match iter.next() {
                        Some(regex) => ExecCmd::Find(regex.to_owned()),
                        None => ExecCmd::Error(ErrorStatus::EmptyFieldError),
                    }
                }
            }
            // set
            "s" => {
                if !other.is_empty() {
                    let (other, param) = other.split_at(1);
                    match other {
                        // series max
                        "m" => match param.parse() {
                            Ok(value) => ExecCmd::Maximum(value),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // series name
                        "n" => match iter.next() {
                            Some(new_name) => ExecCmd::Rename(new_name.to_owned()),
                            None => ExecCmd::Error(ErrorStatus::EmptyFieldError),
                        },
                        // series progress
                        "p" => match param.parse() {
                            Ok(value) => ExecCmd::Progress(value),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // series rate
                        "r" => match param.parse() {
                            Ok(value) => ExecCmd::Rate(value),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // series status
                        "s" => ExecCmd::Status(base::Status::from(param)),
                        _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
                    }
                } else {
                    ExecCmd::Error(ErrorStatus::UnknownCommand)
                }
            }
            // write command
            "w" => ExecCmd::Write,
            // filter command
            "x" => {
                if other.len() > 1 {
                    let (other, param) = other.split_at(1);
                    match other {
                        // status
                        "s" => ExecCmd::FilterParam(ParamType::Status(base::Status::from(param))),
                        // progress
                        "p" => match param.parse() {
                            Ok(value) => ExecCmd::FilterParam(ParamType::Progress(value)),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // series max
                        "m" => match param.parse() {
                            Ok(value) => ExecCmd::FilterParam(ParamType::Maximum(value)),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        // rate
                        "r" => match param.parse() {
                            Ok(value) => ExecCmd::FilterParam(ParamType::Rate(value)),
                            Err(_) => ExecCmd::Error(ErrorStatus::IntParseError),
                        },
                        _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
                    }
                } else {
                    ExecCmd::Error(ErrorStatus::UnknownCommand)
                }
            }
            _ => ExecCmd::Error(ErrorStatus::UnknownCommand),
        }
    }
}
