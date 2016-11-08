use parser;
use base;

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