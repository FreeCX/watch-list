#[derive(Debug, Clone, Copy)]
enum StateMachine {
    Normal,
    Separator,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitFormat {
    Anime,
    Commands,
}

pub struct Splitter<'a> {
    start: usize,
    state: StateMachine,
    string: &'a str,
    fmt: SplitFormat,
}

impl<'a> Splitter<'a> {
    pub fn new(string: &'a str, fmt: SplitFormat) -> Splitter {
        Splitter {
            start: 0,
            state: StateMachine::Normal,
            string: string,
            fmt: fmt,
        }
    }

    fn anime_cycle(state: StateMachine, character: char) -> (StateMachine, Option<char>) {
        use self::StateMachine::*;

        match (state, character) {
            (Normal, '"') => (Text, None),
            (Normal, ' ') => (Separator, None),
            (Normal, _) => (Normal, Some(character)),
            (Separator, ' ') => (Separator, None),
            (Separator, '"') => (Text, None),
            (Separator, _) => (Normal, Some(character)),
            (Text, '"') => (Normal, None),
            (Text, _) => (Text, Some(character)),
        }
    }

    fn command_cycle(state: StateMachine, character: char) -> (StateMachine, Option<char>) {
        use self::StateMachine::*;

        match (state, character) {
            (Normal, '"') => (Text, None),
            (Normal, '/') => (Separator, None),
            (Normal, _) => (Normal, Some(character)),
            (Separator, '/') => (Separator, None),
            (Separator, '"') => (Text, None),
            (Separator, _) => (Normal, Some(character)),
            (Text, '"') => (Normal, None),
            (Text, _) => (Text, Some(character)),
        }
    }
}

impl<'a> Iterator for Splitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        for (index, character) in self.string[self.start..].chars().enumerate() {
            let (new_state, new_char) = match self.fmt {
                SplitFormat::Anime => Splitter::anime_cycle(self.state, character),
                SplitFormat::Commands => Splitter::command_cycle(self.state, character),
            };
            let index = index + self.start;
            self.state = new_state;
            match (new_state, new_char) {
                (StateMachine::Separator, None) => {
                    let last_start = self.start;
                    self.start = index + 1;
                    if index - last_start > 0 {
                        return Some(&self.string[last_start..index]);
                    }
                }
                (StateMachine::Text, None) => self.start = index + 1,
                (StateMachine::Normal, None) => {
                    let result = &self.string[self.start..index - 1];
                    self.start = index + 1;
                    return Some(result);
                }
                _ => {}
            };
        }
        if self.start < self.string.len() {
            let result = &self.string[self.start..];
            self.start = self.string.len();
            Some(result)
        } else {
            None
        }
    }
}
