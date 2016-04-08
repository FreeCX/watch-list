mod extension;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
use std::process::exit;
use extension::*;

fn main() {
    let data = match env::args().nth(1) {
        Some(value) => value,
        None => {
            println!("usage: {} <database>", env::args().nth(0).unwrap());
            exit(0);
        }
    };
    let path = Path::new(&data);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("[error]: {}", why),
    };
    let mut anime: Vec<Database> = Vec::new();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    for string in s.lines() {
        let name_index = string.rfind('"').unwrap();
        let next_index = name_index + 2;
        let data: Vec<_> = (&string[next_index..]).split(' ').collect();
        let name: &str = &string[1..name_index];
        let progress: Vec<_> = data[2].split('/').collect();
        let score: u8 = data[4].parse().unwrap();
        let status = match data[0] {
            "complete" => ListStatus::Complete,
            "hold" => ListStatus::Hold,
            "drop" => ListStatus::Drop,
            "plan" => ListStatus::Plan,
            "watch" => ListStatus::Watch,
            _ => ListStatus::Error
        };
        let item = Database {
            name: name.to_owned(),
            status: status,
            current: progress[0].parse::<u32>().unwrap(),
            maximum: progress[1].parse::<ShowFormat>().unwrap(),
            score: score
        };
        println!("item: {}", item);
        anime.push(item);
    }
}
