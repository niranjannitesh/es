#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    String(String),
    Identifier(String),
    Equal,
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    If,
    Else,
    While,
    Let,
    EOF,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.input.next() {
            Some(ch) => match ch {
                '0'..='9' => self.number(ch),
                '"' => self.string(),
                'a'..='z' | 'A'..='Z' | '_' => self.identifier(ch),
                '=' => Token::Equal,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ';' => Token::Semicolon,
                _ => panic!("Unexpected character: {}", ch),
            },
            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.input.next();
        }
    }

    fn number(&mut self, first_digit: char) -> Token {
        let mut number = first_digit.to_string();
        while let Some(&ch) = self.input.peek() {
            if ch.is_digit(10) || ch == '.' {
                number.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        Token::Number(number.parse().unwrap())
    }

    fn string(&mut self) -> Token {
        let mut string = String::new();
        while let Some(ch) = self.input.next() {
            if ch == '"' {
                break;
            }
            string.push(ch);
        }
        Token::String(string)
    }

    fn identifier(&mut self, first_char: char) -> Token {
        let mut ident = first_char.to_string();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.input.next();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "let" => Token::Let,
            _ => Token::Identifier(ident),
        }
    }
}
