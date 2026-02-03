#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    MachineKeyword,     // "machine"
    StateKeyword,       // "state"
    InitialKeyword,     // "initial"
    TerminalKeyword,    // "terminal"
    TransitionKeyword,  // "on"
    Identifier(String), // "TrafficLight", "Red", "Timer"
    Arrow,              // "->"
    OpenBrace,          // "{"
    CloseBrace,         // "}"
    SemiColon,          // ";"
    EOF,                // "EOF"
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        // 1. Skip whitespace
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }

        // 2. Read the next character
        match self.input.next() {
            // Case A: End of File
            None => Token::EOF,

            // Case B: Simple Symbols
            Some('{') => Token::OpenBrace,
            Some('}') => Token::CloseBrace,
            Some(';') => Token::SemiColon,

            // Case C: Complex Symbols
            Some('-') => match self.input.peek() {
                Some('>') => {
                    self.input.next();
                    Token::Arrow
                }
                Some(c) => panic!("Unexpected character {}", c),
                None => panic!("Unexpected character!"),
            },

            // Case D: Works (Keywords or Identifiers)
            Some(c) if c.is_alphabetic() => {
                let mut text = String::new();
                text.push(c);

                while let Some(&c) = self.input.peek() {
                    if c.is_alphanumeric() {
                        text.push(self.input.next().unwrap())
                    } else {
                        break;
                    }
                }

                match text.as_str() {
                    "state" => Token::StateKeyword,
                    "machine" => Token::MachineKeyword,
                    "initial" => Token::InitialKeyword,
                    "terminal" => Token::TerminalKeyword,
                    "on" => Token::TransitionKeyword,
                    _ => Token::Identifier(text),
                }
            }

            // Catch-all
            Some(c) => panic!("Unexpected character {}", c),
        }
    }
}
