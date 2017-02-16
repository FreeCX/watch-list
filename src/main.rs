extern crate regex;
#[macro_use]
extern crate log;
extern crate tini;

pub mod parser;
pub mod base;
pub mod extra;
pub mod logger;

use std::fs::File;
use std::io::Read;
use std::env::{self, args};
use std::process::exit;
use extra::*;
use regex::Regex;
use tini::Ini;

static USAGE_STRING: &'static str = "\
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
   fr{??}   -- по оценке { ?? -- оценка }
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
    let mut config_file = env::home_dir().unwrap();
    config_file.push(".config/watch-list/config.ini");
    let config = Ini::from_file(config_file.as_path()).unwrap();
    let log_level: String = config.get("main", "log_level").unwrap();
    let filename: String = config.get("main", "open_file").unwrap();
    logger::init(&log_level).unwrap();

    let arg_line = match args().nth(1) {
        Some(value) => value,
        None => {
            let app_name = args().nth(0).unwrap();
            println!("usage: {} '<regex>'\n{}", app_name, USAGE_STRING);
            exit(0);
        }
    };

    debug!("read list from file `{}`", filename);
    let mut anime_base = AnimeBase::new();
    let mut file = File::open(&filename).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    for string in buffer.lines() {
        anime_base.push(base::Item::new(string));
    }

    debug!("command list:");
    let mut iterator = parser::Splitter::new(&arg_line, parser::SplitFormat::Commands);
    let mut anime_list: Vec<usize> = Vec::new();
    while let Some(cmd) = iterator.next() {
        match ExecCmd::get(cmd, &mut iterator) {
            ExecCmd::Increment(value) => {
                debug!("command inc by `{}`", value);
                for index in &anime_list {
                    anime_base.progress_increment(*index).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Decrement(value) => {
                debug!("command dec by `{}`", value);
                for index in &anime_list {
                    anime_base.progress_decrement(*index).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Append(name) => {
                debug!("command new anime `{}`", name);
                let new_item = anime_base.append(&name);
                anime_list.push(new_item);
                println!("> append: {}", anime_base.format_by_index(new_item));
            }
            ExecCmd::Delete => {
                debug!("command delete item");
                // TODO: implement
            }
            ExecCmd::Info => {
                debug!("command print list");
                println!("{}", anime_base);
            }
            ExecCmd::Find(regex) => {
                debug!("command find `{}`", regex);
                let re = Regex::new(&regex).unwrap();
                for (index, item) in anime_base.list.iter().enumerate() {
                    if re.is_match(&item.name) {
                        anime_list.push(index);
                        println!(">  found: {}", anime_base.format(item));
                    }
                }
            }
            ExecCmd::FindParam(param) => {
                debug!("command find by param `{:?}`", param);
                // TODO: implement
            }
            ExecCmd::Maximum(value) => {
                debug!("command series limit to `{}`", value);
                for index in &anime_list {
                    anime_base.set_maximum(*index, value).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Rename(new_name) => {
                debug!("command new name `{}`", new_name);
                for index in &anime_list {
                    anime_base.set_name(*index, &new_name).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Progress(value) => {
                debug!("command progress `{}`", value);
                for index in &anime_list {
                    anime_base.set_progress(*index, value).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Status(status) => {
                debug!("command status `{:?}`", status);
                for index in &anime_list {
                    anime_base.set_status(*index, status).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Rate(value) => {
                debug!("command rate `{}`", value);
                for index in &anime_list {
                    anime_base.set_rate(*index, value).unwrap();
                    println!("> update: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Write => {
                debug!("command write changes");
                let mut file = File::create(&filename).unwrap();
                anime_base.write_to_file(&mut file).expect("Can't write to file!");
            }
            ExecCmd::Error(kind) => warn!("command unknown `{}`: {:?}", cmd, kind),
        };
    }
}
