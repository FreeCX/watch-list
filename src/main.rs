#![feature(collections)]
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const NAME: i8 = 0;
pub const STATUS: i8 = 1;
pub const CURRENT: i8 = 3;
pub const MAXIMUM: i8 = 4;
pub const SCORE: i8 = 6;

struct Database {
    name: String,
    status: String,
    current: i32,
    maximum: i32, 
    score: i8
}

fn tokenize( data: & str, delimeters: & str ) -> Vec< String >  {
    let mut token: Vec< String > = Vec::new();
    let mut prev = 0;
    let mut next = 0;
    let mut comma = false;

    for i in data.chars() {
        comma = match i {
            '"' => !comma,
            _ => comma
        };
        if !comma {
            for j in delimeters.chars() {
                if i == j {
                    if next - prev >= 1 {
                        token.push( data.slice_chars( prev, next ).to_string() );
                        prev = next + 1;
                        break;
                    }
                    prev = next + 1;
                }
            }
        }
        next += 1;
    }
    // add last token
    if next - prev >= 1 {
        token.push( data.slice_chars( prev, next ).to_string() );
    }
    return token;
}

fn print_item( item: & Database ) {
    println!( "<add>: '{}', status: {}, progress: {} / {}, score: {}", 
              item.name, item.status, item.current, item.maximum, item.score );
}

fn main() {
    let path = Path::new( "anime-list" );
    let display = path.display();
    let mut file = match File::open( &path ) {
        Ok( file ) => file,
        Err( why ) => panic!( "[error]: {}", Error::description( &why ) )
    };
    let mut anime: Vec< Database > = Vec::new(); 
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok( _ ) => {
            let tokens = tokenize( s.trim(), " /\n\r\t" );
            let mut item = Database { 
                name: "".to_string(), 
                status: "plan".to_string(), 
                current: 0, 
                maximum: 0,
                score: 0
            };
            let mut counter = NAME;
            for element in tokens {
                match element.slice_chars( 0, 1 ) {
                    "\"" => {
                        counter = NAME;
                        if item.name.len() > 0 {
                            print_item( &item );
                            anime.push( Database {
                                name: item.name.to_string(),
                                status: item.status.to_string(),
                                current: item.current,
                                maximum: item.maximum,
                                score: item.score
                            });
                        }
                    },
                    _ => {
                        counter += 1;
                    }
                }
                match counter {
                    NAME => item.name  = element.slice_chars( 1, element.len() - 1 ).to_string(),
                    STATUS => item.status = element.to_string(),
                    CURRENT => item.current = element.parse::<i32>().unwrap(),
                    MAXIMUM => item.maximum = element.parse::<i32>().unwrap(),
                    SCORE => item.score = element.parse::<i8>().unwrap(),
                    _ => {}
                };
            }
        },
        Err( why ) => panic!( "couldn't read '{}': {}", display, Error::description( &why ) )
    };
}