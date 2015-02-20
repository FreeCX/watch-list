use std::io::fs::File;
use std::clone;
use std::fmt;

pub const NAME: i8 = 0;
pub const STATUS: i8 = 1;
pub const CURRENT: i8 = 3;
pub const MAXIMUM: i8 = 4;
pub const SCORE: i8 = 6;

#[derive(Clone)]
enum ListStatus {
    Error,
    Complete,
    Drop,
    Plan,
    Watch,
    Hold,
}

#[derive(Clone)]
struct Database {
    name: String,
    status: ListStatus,
    current: i32,
    maximum: i32, 
    score: u8
}

fn get_status( data: & str ) -> ListStatus {
    match data {
        "complete" => ListStatus::Complete,
        "drop" => ListStatus::Drop,
        "plan" => ListStatus::Plan,
        "watch" => ListStatus::Watch,
        "hold" => ListStatus::Hold,
        _ => ListStatus::Error
    }
}

impl fmt::String for ListStatus {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            ListStatus::Complete => f.write_str( "complete" ),
            ListStatus::Drop => f.write_str( "drop" ),
            ListStatus::Plan => f.write_str( "plan" ),
            ListStatus::Watch => f.write_str( "watch" ),
            ListStatus::Hold => f.write_str( "hold" ),
            ListStatus::Error => f.write_str( "<error>" )
        }
    }
} 

impl fmt::String for Database {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "'{:>16}', status: {:>8}, progress: {:>2} / {:>2}, score: {}", 
                self.name, self.status.to_string(), self.current, self.maximum, self.score )
    }
}

fn tokenize( data: & str, delimeters: & str ) -> Vec< String >  {
    let mut token: Vec< String > = Vec::new();
    let mut prev = 0us;
    let mut next = 0us;
    let mut comma = false;

    for i in data.chars() {
        comma = match i {
            '"' => !comma,
            _ => comma
        };
        if !comma {
            for j in delimeters.chars() {
                if i == j {
                    if next - prev >= 1us {
                        token.push( data.slice_chars( prev, next ).to_string() );
                        prev = next + 1us;
                        break;
                    }
                    prev = next + 1us;
                }
            }
        }
        next += 1us;
    }
    // add last token
    if next - prev >= 1us {
        token.push( data.slice_chars( prev, next ).to_string() );
    }
    return token;
}

fn main() {
    let path = Path::new( "anime-list" );
    let display = path.display();
    let mut file = match File::open( &path ) {
        Ok( file ) => file,
        Err( io_error ) => {
            panic!( io_error.desc );
        }
    };
    let mut anime: Vec< Database > = Vec::new(); 
    match file.read_to_string() {
        Ok( string ) => {
            let token = tokenize( string.as_slice(), " /\n" );
            let mut item = Database { 
                name: "".to_string(), 
                status: ListStatus::Plan,
                current: 0, 
                maximum: 0,
                score: 0
            };
            let mut counter = NAME;
            for element in token.iter() {
                match element.slice_chars( 0, 1 ) {
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
                    NAME => item.name = element.slice_chars( 1, element.len() - 1 ).to_string(),
                    STATUS => item.status = get_status( element.as_slice() ),
                    CURRENT => item.current = element.parse::<i32>().unwrap_or( 0 ),
                    MAXIMUM => item.maximum = element.parse::<i32>().unwrap_or( 0 ),
                    SCORE => item.score = element.parse::<u8>().unwrap_or( 0 ),
                    _ => {}
                };
            }
            // add last element
            println!( "add: {}", item );
            anime.push( item.clone() );
        },
        Err( why ) => panic!( "couldn't read '{}': {}", display, why.desc )
    };
}