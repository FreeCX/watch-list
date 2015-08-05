mod extension;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
use std::process::exit;
use extension::*;

fn main() {
    let data = match env::args().nth( 1 ) {
        Some( value ) => value,
        None          => {
            println!( "usage: {} <database>", env::args().nth( 0 ).unwrap() );
            exit( 0 );
        }
    };
    let path = Path::new( &data );
    let display = path.display();
    let mut file = match File::open( &path ) {
        Ok( file ) => file,
        Err( why ) => panic!( "[error]: {}", why )
    };
    let mut anime: Vec< Database > = Vec::new(); 
    let mut s = String::new();
    match file.read_to_string( &mut s ) {
        Ok( _ ) => {
            let tokens = s.trim().tokenize( " /\n\r\t" );
            let mut item = Database { 
                name: "".to_string(), 
                status: ListStatus::Plan,
                current: 0, 
                maximum: 0,
                score: 0
            };
            let mut counter = NAME;
            for element in tokens {
                match element.slice( 0, 1 ) {
                    "\"" => {
                        counter = NAME;
                        if item.name.len() > 0 {
                            println!( "add: {}", item );
                            anime.push( item.clone() );
                        }
                    },
                    _ => {
                        counter += 1;
                    }
                }
                match counter {
                    NAME    => item.name = element.slice( 1, element.len() - 1 ).to_string(),
                    STATUS  => item.status = element.trim().get_status(),
                    CURRENT => item.current = element.parse::<u32>().unwrap_or( 0 ),
                    MAXIMUM => item.maximum = element.parse::<u32>().unwrap_or( 0 ),
                    SCORE   => item.score = element.parse::<u8>().unwrap_or( 0 ),
                    _ => {}
                };
            }
            // add last element
            println!( "add: {}", item );
            anime.push( item.clone() );
        },
        Err( why ) => panic!( "couldn't read '{}': {}", display, why )
    };
}