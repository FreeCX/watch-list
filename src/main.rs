extern crate regex;
#[macro_use]
extern crate log;
extern crate colored;
extern crate dirs;
extern crate tini;

mod base;
mod extra;
mod logger;
mod parser;

use colored::*;
use extra::*;
use regex::Regex;
use std::env::args;
use std::fs;
use std::process::exit;
use tini::Ini;

// TODO: check & rewrite
static USAGE_STRING: &str = "\
>> доступные команды:
 -{n}       -- номер серии -n { стандартное значение = 1 }
 +{n}       -- номер серии +n { стандартное значение = 1 }
 a          -- добавить элемент [ a/имя | a/\"имя\" ]
 d          -- удалить элементы { найденые элементы параметром f }
 i          -- вывести весь список
 f{??}      -- поиск по параметру
   f        -- поиск по названию [ f/\"имя или regex\" ]
   fs{??}   -- по статусу { ?? -- буква статуса }
     где ??: c -- complete, d -- drop, h -- hold, p -- plan, w -- watch
   fp{??}   -- по номеру серии { ?? -- номер серии }
   fm{??}   -- по количеству серий в сезоне { ?? -- количество серий в сезоне }
   fr{??}   -- по оценке { ?? -- оценка }
 x{??}      -- фильтровать список полученный после f{??}
   xs{??}   -- по статусу { ?? -- буква статуса }
     где ??: c -- complete, d -- drop, h -- hold, p -- plan, w -- watch
   xp{??}   -- по номеру серии { ?? -- номер серии }
   xm{??}   -- по количеству серий в сезоне { ?? -- количество серий в сезоне }
   xr{??}   -- по оценке { ?? -- оценка }
 s{??}      -- установить параметр
  sn        -- изменить имя на новое [ sn/имя | sn/\"имя\" ]
  sm{число} -- изменить максимальный номер серии { ? в случае онгоинга }
  sp{число} -- изменить номер серии на { число }
  sr{число} -- изменить рейтинг на { число }
  ss{??}    -- изменить статуc на { ?? -- буква статуса }
    где ??: c -- complete, d -- drop, h -- hold, p -- plan, w -- watch
 w          -- записать изменения в базу
>> example: 'f/\"One Piece\"/sm?/+5/-/sr7/sp23/ssc/sn/d.gray-man/sm24/w'";

fn main() {
    let mut config_file = dirs::home_dir().expect("Cannot detect home folder");
    config_file.push(".config/watch-list/config.ini");
    let config = Ini::from_file(config_file.as_path()).expect("Cannot open config file");

    let log_level: String = config.get("main", "log_level").unwrap_or_else(|| "warn".to_owned());
    let filename: String = config.get("main", "open_file").expect("List file not set");
    logger::init(&log_level).unwrap();

    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        let app_name = &args[0];
        println!("usage: {} '<regex>'\n{}", app_name, USAGE_STRING);
        exit(0);
    }
    let arg_line = &args[1];

    let mut update_flag = false;
    let mut save_flag = false;
    let mut delete_flag = false;
    let mut filter_command = false;
    let mut parity_item = 0;

    debug!("read list from file `{}`", filename);
    let mut anime_base = AnimeBase::new();
    let buffer = fs::read_to_string(&filename).expect("Cannot open list file");
    for string in buffer.lines() {
        anime_base.push(base::Item::new(string));
    }

    let mut colorizer = |s: String| -> ColoredString {
        parity_item = (parity_item + 1) % 2;
        if parity_item % 2 == 0 {
            s.white().bold()
        } else {
            s.green().bold()
        }
    };

    debug!("command list:");
    let mut iterator = parser::Splitter::new(arg_line, parser::SplitFormat::Commands);
    let mut anime_list: Vec<usize> = Vec::new();
    let mut commands = Vec::new();
    // collect all input commands
    while let Some(item) = iterator.next() {
        let cmd = ExecCmd::get(item, &mut iterator);
        if let ExecCmd::FilterParam(_) = cmd {
            filter_command = true
        }
        commands.push((item, cmd));
    }
    for (item, cmd) in commands {
        match cmd {
            ExecCmd::Increment(value) => {
                debug!("command inc by `{}`", value);
                for index in &anime_list {
                    anime_base.progress_increment_by(*index, value).expect("Problem with increment_by");
                }
                update_flag = true;
            }
            ExecCmd::Decrement(value) => {
                debug!("command dec by `{}`", value);
                for index in &anime_list {
                    anime_base.progress_decrement_by(*index, value).expect("Problem with decrement_by");
                }
                update_flag = true;
            }
            ExecCmd::Append(name) => {
                debug!("command new anime `{}`", name);
                let new_item = anime_base.append(&name);
                anime_list.push(new_item);
                let result = format!("> append: {}", anime_base.format_by_index(new_item)).red();
                println!("{}", result);
            }
            ExecCmd::Delete => {
                debug!("command delete item");
                let mut remove_list = anime_list.clone();
                remove_list.sort_by(|a, b| b.cmp(a));
                for index in remove_list {
                    let result = format!("> delete: {}", anime_base.format_by_index(index)).red();
                    anime_base.list.swap_remove(index);
                    println!("{}", result);
                }
                delete_flag = true;
            }
            ExecCmd::Info => {
                debug!("command print list");
                for item in &anime_base.list {
                    let item = anime_base.format(item).to_string();
                    println!("{}", colorizer(item));
                }
            }
            ExecCmd::Find(regex) => {
                debug!("command find `{}`", regex);
                let re = Regex::new(&regex).expect("Problem with regex");
                for (index, item) in anime_base.list.iter().enumerate() {
                    if re.is_match(&item.name) {
                        anime_list.push(index);
                        if !filter_command {
                            let item = format!(">  found: {}", anime_base.format(item));
                            println!("{}", colorizer(item));
                        }
                    }
                }
            }
            ExecCmd::FindParam(param) => {
                debug!("command find by param `{:?}`", param);
                for (index, item) in anime_base.list.iter().enumerate() {
                    let is_match = match param {
                        ParamType::Status(value) => item.status == value,
                        ParamType::Progress(value) => item.progress == value,
                        ParamType::Maximum(value) => item.maximum == value,
                        ParamType::Rate(value) => item.rate == value,
                    };
                    if is_match {
                        anime_list.push(index);
                        if !filter_command {
                            let item = format!(">  found: {}", anime_base.format(item));
                            println!("{}", colorizer(item));
                        }
                    }
                }
            }
            ExecCmd::FilterParam(param) => {
                debug!("command filter by param `{:?}`", param);
                let mut new_anime_list = Vec::new();
                for index in anime_list.into_iter() {
                    let item = anime_base.get_item(index).expect("Cannot get item");
                    let is_match = match param {
                        ParamType::Status(value) => item.status == value,
                        ParamType::Progress(value) => item.progress == value,
                        ParamType::Maximum(value) => item.maximum == value,
                        ParamType::Rate(value) => item.rate == value,
                    };
                    if is_match {
                        new_anime_list.push(index);
                        let item = format!("> filter: {}", anime_base.format(item));
                        println!("{}", colorizer(item));
                    }
                }
                anime_list = new_anime_list;
            }
            ExecCmd::Maximum(value) => {
                debug!("command series limit to `{}`", value);
                for index in &anime_list {
                    anime_base.set_maximum(*index, value).expect("Problem with set_maximum");
                }
                update_flag = true;
            }
            ExecCmd::Rename(new_name) => {
                debug!("command new name `{}`", new_name);
                for index in &anime_list {
                    anime_base.set_name(*index, &new_name).expect("Problem with set_name");
                }
                update_flag = true;
            }
            ExecCmd::Progress(value) => {
                debug!("command progress `{}`", value);
                for index in &anime_list {
                    anime_base.set_progress(*index, value).expect("Problem with set_progress");
                }
                update_flag = true;
            }
            ExecCmd::Status(status) => {
                debug!("command status `{:?}`", status);
                for index in &anime_list {
                    anime_base.set_status(*index, status).expect("Problem with set_status");
                }
                update_flag = true;
            }
            ExecCmd::Rate(value) => {
                debug!("command rate `{}`", value);
                for index in &anime_list {
                    anime_base.set_rate(*index, value).expect("Problem with set_rate");
                }
                update_flag = true;
            }
            ExecCmd::Write => {
                debug!("command write changes");
                let mut file = fs::File::create(&filename).expect("Cannot write list file");
                anime_base.write_to_file(&mut file).expect("Cannot write to list file");
                save_flag = true;
            }
            ExecCmd::Error(kind) => warn!("`{}`: {:?}", item, kind),
        };
    }
    if update_flag {
        for index in &anime_list {
            let item = format!("> update: {}", anime_base.format_by_index(*index));
            println!("{}", colorizer(item))
        }
    }
    if save_flag {
        println!("{}", if update_flag || delete_flag { "> changes saved".red() } else { "> nothing to save".red() });
    }
}
