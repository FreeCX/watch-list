use std::io::fs::File;

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

fn print_item( item: & Database ) {
    println!( "<add>: '{}', status: {}, progress: {} / {}, score: {}", 
              item.name, item.status, item.current, item.maximum, item.score );
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
                status: "plan".to_string(), 
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
        Err( why ) => panic!( "couldn't read '{}': {}", display, why.desc )
    };
}