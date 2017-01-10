// Cargo.toml : use crate `colored = "1.3"`
extern crate regex;

pub mod parser;
pub mod base;
pub mod extra;

use std::fs::File;
use std::io::Read;
use std::u16;
use std::env::args;
use std::process::exit;
use extra::*;
use regex::Regex;

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
    let arg_line = match args().nth(1) {
        Some(value) => value,
        None => {
            let app_name = args().nth(0).unwrap();
            println!("usage: {} '<regex>'\n{}", app_name, USAGE_STRING);
            exit(0);
        }
    };
    let mut anime_base = AnimeBase::new();
    let mut file = File::open("anime-list").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    for string in buffer.lines() {
        anime_base.push(base::Item::new(string));
    }
    println!("command list:");
    let mut iterator = parser::Splitter::new(&arg_line, parser::SplitFormat::Commands);
    let mut anime_list: Vec<usize> = Vec::new();
    // let mut color_exchange = true;
    while let Some(cmd) = iterator.next() {
        match ExecCmd::get(cmd, &mut iterator) {
            ExecCmd::Increment(value) => {
                println!("[cmd] inc by `{}`", value);
                for index in &anime_list {
                    let progress = anime_base.list.get(*index).unwrap().progress;
                    let result = match progress.checked_add(value) {
                        Some(value) => value,
                        None => u16::MAX
                    };
                    anime_base.list.get_mut(*index).unwrap().progress = result;
                    println!("> u: {}", anime_base.format_by_index(*index));
                }
            }
            ExecCmd::Decrement(value) => {
                println!("[cmd] dec by `{}`", value);
                for index in &anime_list {
                    let progress = anime_base.list.get(*index).unwrap().progress;
                    let result = match progress.checked_sub(value) {
                        Some(value) => value,
                        None => 0,
                    };
                    anime_base.list.get_mut(*index).unwrap().progress = result;
                    println!("> u: {}", anime_base.format_by_index(*index));
                }
            },
            ExecCmd::Append(name) => println!("[cmd] new anime `{}`", name),
            ExecCmd::Delete => println!("[cmd] delete item"),
            ExecCmd::Info => {
                println!("[cmd] print list");
                println!("{}", anime_base);
            },
            ExecCmd::Find(regex) => {
                println!("[cmd] find `{}`", regex);
                let re = Regex::new(&regex).unwrap();
                for (index, item) in anime_base.list.iter().enumerate() {
                    if re.is_match(&item.name) {
                        anime_list.push(index);
                        println!("> f: {}", anime_base.format(item));
                    }
                }
            },
            ExecCmd::FindParam(param) => println!("[cmd] find by param `{:?}`", param),
            ExecCmd::Maximum(value) => println!("[cmd] series limit `{}`", value.get()),
            ExecCmd::Rename(new_name) => println!("[cmd] new name `{}`", new_name),
            ExecCmd::Progress(value) => println!("[cmd] progress `{}`", value),
            ExecCmd::Status(status) => println!("[cmd] status `{:?}`", status),
            ExecCmd::Rate(value) => println!("[cmd] rate `{}`", value),
            ExecCmd::Write => println!("[cmd] write changes"),
            ExecCmd::Error(kind) => println!("[cmd] unknown command `{}`: {:?}", cmd, kind)
        };
    }
}
