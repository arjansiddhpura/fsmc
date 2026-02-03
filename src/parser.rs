use crate::ast::*;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // Helper: Returns the current token without consuming it
    fn peek(&self) -> &Token {
        if self.pos < self.tokens.len() {
            &self.tokens[self.pos]
        } else {
            &Token::EOF
        }
    }

    // Helper: Returns the current token AND advances position
    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        token
    }

    pub fn parse_machine(&mut self) -> Result<Machine, String> {
        // 1. Expect the 'machine' keyword
        if *self.peek() != Token::MachineKeyword {
            return Err("Expected 'machine' keyword!".to_string());
        }
        self.advance();

        // 2. Expect machine name
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected machine name!".to_string()),
        };

        // 3. Expect '{'
        if self.advance() != Token::OpenBrace {
            return Err("Expected '{' after machine name!".to_string());
        }

        // 4. Parse states
        let mut states = Vec::new();
        while *self.peek() == Token::StateKeyword {
            states.push(self.parse_state()?);
        }

        // 5. Expect '}'
        if self.advance() != Token::CloseBrace {
            return Err("Expected '}' at the end of the file!".to_string());
        }

        Ok(Machine { name, states })
    }

    pub fn parse_state(&mut self) -> Result<State, String> {
        // 1. Expect the 'state' keyword
        if *self.peek() != Token::StateKeyword {
            return Err("Expected 'state' keyword!".to_string());
        }
        self.advance();

        // 2. Expect state name
        let name = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected state name!".to_string()),
        };

        // 3. Expect '{'
        if self.advance() != Token::OpenBrace {
            return Err("Expected '{' after state name!".to_string());
        }

        // 4. Parse transitions
        let mut transitions = Vec::new();
        while *self.peek() == Token::TransitionKeyword {
            transitions.push(self.parse_transition()?);
        }

        // 5. Expect '}'
        if self.advance() != Token::CloseBrace {
            return Err("Expected '}' at the end of the file!".to_string());
        }

        Ok(State { name, transitions })
    }

    pub fn parse_transition(&mut self) -> Result<Transition, String> {
        // 1. Expect the 'on' keyword
        if *self.peek() != Token::TransitionKeyword {
            return Err("Expected 'on' keyword!".to_string());
        }
        self.advance();

        // 2. Expect event name
        let event = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected event name!".to_string()),
        };

        // 3. Expect arrow
        if *self.peek() != Token::Arrow {
            return Err("Expected '->' after the event name!".to_string());
        }
        self.advance();

        // 4. Expect target name
        let target = match self.advance() {
            Token::Identifier(n) => n.clone(),
            _ => return Err("Expected target state name!".to_string()),
        };

        // 5. Expect ';'
        if *self.peek() != Token::SemiColon {
            return Err("Expected ';' after end of transition description!".to_string());
        }
        self.advance();

        Ok(Transition { event, target })
    }
}
