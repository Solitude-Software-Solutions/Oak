use std::str;
use std::io;

#[derive(Debug)]
pub enum Token
{
    NEW,
    FETCH,
    RENAME,
    SET,
    ERASE,
    WITHIN,
    TO,
    COLLECTION,
    CLUSTER,
    RECORD,
    INTEGER,
    FLOAT,
    STRING,
    BOOLEAN,
    INT,
    FLT,
    STR,
    BOOL,
    TRUE,
    FALSE,
    Number(f64),
    Identifier(String),
    Semicolon,
}


pub struct Lexer
{
    input: String,
    index: usize,
}



//implementing the lexer struct
impl Lexer
{
    //Lexer.new() makes a new instance of the lexer struct and returns it
    pub fn new(input:&str) -> Self
    {
        Lexer{
             input: input.to_string(),
             index: 0
        }
    }

    // Lexer.next_token() returns the next token identifies in the input string and returns the next token
    pub fn next_token(&mut self) -> Option<Token>
    {
        self.skip_whitespace();

        //if the index is greater than the length of the input string, return None
        if self.index >= self.input.len()
        {
            return None;
        }
        //create a variable to hold the current character
        let character = self.input.chars().nth(self.index)?;
        match character
        {
            ';' => {
                self.index += 1;
                Some(Token::Semicolon)
            }

            character if character.is_alphabetic()=> {
                let word = self.read_identifier();
                match word.to_lowercase().as_str()
                {
                "NEW" => Some(Token::NEW),
                "FETCH" => Some(Token::FETCH),
                "RENAME" => Some(Token::RENAME),
                "SET" => Some(Token::SET),
                "ERASE" => Some(Token::ERASE),
                "WITHIN" => Some(Token::WITHIN),
                "TO" => Some(Token::TO),
                "COLLECTION" => Some(Token::COLLECTION),
                "CLUSTER" => Some(Token::CLUSTER),
                "RECORD" => Some(Token::RECORD),
                "INTEGER" => Some(Token::INTEGER),
                "FLOAT" => Some(Token::FLOAT),
                "STRING" => Some(Token::STRING),
                "BOOLEAN" => Some(Token::BOOLEAN),
                "INT" => Some(Token::INT),
                "FLT" => Some(Token::FLT),
                "STR" => Some(Token::STR),
                "BOOL" => Some(Token::BOOL),
                "TRUE" => Some(Token::TRUE),
                "FALSE" => Some(Token::FALSE),
                _ => Some(Token::Identifier(word)),
                }
            }
            character if character.is_digit(10) => Some(Token::Number(self.read_number())),
            _ => {
                self.index += 1;
                self.next_token()
            }
        }
    }

    //skips over any whitespace chars in the input string
    fn skip_whitespace(&mut self) {
            while self.index < self.input.len() && self.input.chars().nth(self.index).map_or(false, |character| character.is_whitespace()) {
                self.index += 1;
            }
        }

    //reads an identifier from the input string like the keywords
    fn read_identifier(&mut self) -> String {
        let start = self.index;
        while self.index < self.input.len() && self.input.chars().nth(self.index).map_or(false, |character| character.is_alphabetic()) {
            self.index += 1;
        }
        //return the identifier as a string
        self.input[start..self.index].to_string()
    }

    //reads a number from the input string, whole or decimal
    fn read_number(&mut self) -> f64 {
        let start = self.index;
        while self.index < self.input.len() && self.input.chars().nth(self.index).map_or(false, |character| character.is_digit(10) || character == '.') {
            self.index += 1;
        }
        //parse the number from the input string to a float
        self.input[start..self.index].parse().unwrap_or(0.0)
    }
}
