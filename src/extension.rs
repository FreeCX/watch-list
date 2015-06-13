use std::fmt;

pub const NAME: i8 = 0;
pub const STATUS: i8 = 1;
pub const CURRENT: i8 = 3;
pub const MAXIMUM: i8 = 4;
pub const SCORE: i8 = 6;

#[derive(Clone)]
pub enum ListStatus {
    Error,
    Complete,
    Drop,
    Plan,
    Watch,
    Hold,
}

#[derive(Clone)]
pub struct Database {
    pub name: String,
    pub status: ListStatus,
    pub current: u32,
    pub maximum: u32, 
    pub score: u8
}

impl fmt::Display for ListStatus {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match *self {
            ListStatus::Complete => f.write_str( "complete" ),
            ListStatus::Drop     => f.write_str( "drop" ),
            ListStatus::Plan     => f.write_str( "plan" ),
            ListStatus::Watch    => f.write_str( "watch" ),
            ListStatus::Hold     => f.write_str( "hold" ),
            ListStatus::Error    => f.write_str( "<error>" )
        }
    }
} 

impl fmt::Display for Database {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "'{:>16}', status: {:>8}, progress: {:>2} / {:>2}, score: {}", 
                self.name, self.status.to_string(), self.current, self.maximum, self.score )
    }
}

pub trait StrSpec {
    fn slice( &self, begin: usize, end: usize ) -> &str;
    fn get_status( &self ) -> ListStatus;
    fn tokenize( &self, delimeters: & str ) -> Vec< String >;
}

impl StrSpec for str {
    fn slice( &self, begin: usize, end: usize ) -> &str {
        assert!( begin <= end );
        let mut begin_byte;
        let mut end_byte;

        begin_byte = if begin >= 0 { Some( begin ) } else { None };
        end_byte = if end <= self.len() { Some( end ) } else { Some( self.len() ) };
        match ( begin_byte, end_byte ) {
            ( None, _ ) => panic!( "slice: `begin` is beyond end of string" ),
            ( _, None ) => panic!( "slice: `end` is beyond end of string" ),
            ( Some( a ), Some( b ) ) => unsafe { 
                self.slice_unchecked( a, b )
            }
        }
    }
    fn get_status( &self ) -> ListStatus {
        match self {
            "complete" => ListStatus::Complete,
            "drop" => ListStatus::Drop,
            "plan" => ListStatus::Plan,
            "watch" => ListStatus::Watch,
            "hold" => ListStatus::Hold,
            _ => ListStatus::Error
        }
    }
    fn tokenize( &self, delimeters: & str ) -> Vec< String > {
        let mut token: Vec< String > = Vec::new();
        let mut prev = 0;
        let mut next = 0;
        let mut comma = false;

        for i in self.chars() {
            comma = match i {
                '"' => !comma,
                _ => comma
            };
            if !comma {
                for j in delimeters.chars() {
                    if i == j {
                        if next - prev >= 1 {
                            token.push( self.slice( prev, next ).to_string() );
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
            token.push( self.slice( prev, next ).to_string() );
        }
        return token;
    }
}