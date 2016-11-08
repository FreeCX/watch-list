pub mod parser;
pub mod base;
pub mod extra;

use std::fs::File;
use std::io::Read;
use std::env::args;
use std::process::exit;
use extra::*;

static USAGE_STRING: &'static str = "\
>> available commands:
 -{n}      -- номер серии -n { стандартное значение = 1 }
 +{n}      -- номер серии +n { стандартное значение = 1 }
 a         -- добавить элемент [ a/имя | a/\"имя\" ]
 d         -- удалить элементы { найденые элементы параметром f }
 i         -- вывести весь список
 f         -- поиск по названию [ f/\"имя или regex\" ]
 g{??}     -- поиск по параметру
    gs{??} -- статусу { ?? -- буква статуса }
    gp{??} -- номеру серии { ?? -- номер серии }
    gr{??} -- оценке { ?? -- оценка }
 m{число}  -- установить максимальный номер серии { ? в случае онгоинга }
 n         -- изменить имя на новое [ n/имя | n/\"имя\" ]
 p{число}  -- установить номер серии на { число }
 s{??}     -- установить статуc
    sc     -- complete
    sd     -- drop
    sh     -- hold
    sp     -- plan
    sw     -- watch
 r{число}  -- установить рейтинг { число }
 w         -- записать изменения в базу
>> example: 'f/\"One Piece\"/m?/+5/-/r7/p23/sc/n/d.gray-man/m24/w'";

fn main() {
    let arg_line = match args().nth(1) {
        Some(value) => value,
        None => {
            let app_name = args().nth(0).unwrap();
            println!("usage: {} '<regex>'\n{}", app_name, USAGE_STRING);
            exit(0);
        }
    };
    let mut anime_base = Vec::new();
    let mut file = File::open("anime-list").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    for string in buffer.lines() {
        anime_base.push(base::Item::new(string));
    }
    println!("command list:");
    let mut iterator = parser::Splitter::new(&arg_line, parser::SplitFormat::Commands);
    // let mut find_list: Vec<&base::Item> = Vec::new();
    while let Some(cmd) = iterator.next() {
        match ExecCmd::get(cmd, &mut iterator) {
            ExecCmd::Increment(value) => println!("[cmd] inc by `{}`", value),
            ExecCmd::Decrement(value) => println!("[cmd] dec by `{}`", value),
            ExecCmd::Append(name) => println!("[cmd] new anime `{}`", name),
            ExecCmd::Delete => println!("[cmd] delete item"),
            ExecCmd::Info => {
                println!("[cmd] print list");
                for item in &anime_base {
                    println!("{}", item);
                }
            },
            ExecCmd::Find(regex) => println!("[cmd] find `{}`", regex),
            ExecCmd::FindParam(param) => println!("[cmd] find by param `{:?}`", param),
            ExecCmd::Maximum(value) => println!("[cmd] series limit `{}`", value.get()),
            ExecCmd::Rename(new_name) => println!("[cmd] new name `{}`", new_name),
            ExecCmd::Progress(value) => println!("[cmd] progress `{}`", value),
            ExecCmd::Status(status) => println!("[cmd] status `{:?}`", status),
            ExecCmd::Rate(value) => println!("[cmd] rate `{}`", value),
            ExecCmd::Write => println!("[cmd] write changes"),
            _ => println!("[cmd] unknown command `{}`", cmd)
        };
    }
}
