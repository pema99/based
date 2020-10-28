#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Float,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Equals,
    EqualsEquals,
    Comma,
    Semicolon,

    Plus,
    Minus,
    Star,
    Slash,

    Fun,
    Let,
    Return,

    Identifier(String),
    FloatLiteral(f64),
}

#[derive(Debug, Clone)]
pub enum ScanningProduct {
    Skip,
    Finished,
    Token(Token),
}

#[derive(Clone, Debug)]
pub enum ScanningError {
    UnexpectedCharacter(char),
    InvalidLiteral,
    UnexpectedEndOfFile,
}

type ScanningResult = Result<ScanningProduct, ScanningError>;

pub struct Scanner<I: Iterator<Item = char>> {
    input: I,
    line: u32,
    offset: u32,
    peeked: Option<char>,
}
impl<I: Iterator<Item = char>> Scanner<I> {
    pub fn new(input: I) -> Self {
        Scanner {
            input,
            line: 1,
            offset: 0,
            peeked: None,
        }
    }

    pub fn scan_all(mut self) -> Result<Vec<Token>, ScanningError> {
        let mut output = Vec::new();

        loop {
            match self.scan_token()? {
                ScanningProduct::Skip => (),
                ScanningProduct::Finished => return Ok(output),
                ScanningProduct::Token(token) => {
                    output.push(token);
                }
            }
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.offset += 1;
        match self.peeked {
            None => self.input.next(),
            Some(c) => {
                self.peeked = None;
                Some(c)
            }
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        match self.peeked {
            Some(c) => Some(c),
            None => {
                self.peeked = self.input.next();
                self.peeked
            }
        }
    }

    pub fn keyword(&self, what: &str) -> Option<Token> {
        match what.to_owned().to_lowercase().as_str() {
            "return" => Some(Token::Return),
            "fun" => Some(Token::Fun),
            "let" => Some(Token::Let),
            _ => None,
        }
    }

    pub fn scan_token(&mut self) -> ScanningResult {
        let c = match self.advance() {
            Some(c) => c,
            None => {
                return Ok(ScanningProduct::Finished);
            }
        };
        let peeked = self.peek();

        let tok = |t| Ok(ScanningProduct::Token(t));

        match c {
            '(' => tok(Token::LeftParen),
            ')' => tok(Token::RightParen),
            '{' => tok(Token::LeftBrace),
            '}' => tok(Token::RightBrace),
            '=' => tok(Token::Equals),
            '-' => tok(Token::Minus),
            '+' => tok(Token::Plus),
            '/' => match peeked {
                Some('/') => {
                    while self.advance().ok_or(ScanningError::UnexpectedEndOfFile)? != '\n' {}
                    self.offset = 0;
                    self.line += 1;

                    Ok(ScanningProduct::Skip)
                }
                _ => tok(Token::Slash),
            },
            '*' => tok(Token::Star),
            ',' => tok(Token::Comma),
            ';' => tok(Token::Semicolon),

            '\n' => {
                self.line += 1;
                self.offset = 0;
                Ok(ScanningProduct::Skip)
            }
            c if c.is_whitespace() => Ok(ScanningProduct::Skip),
            c if c.is_numeric() => self.scan_numerics(c),
            c if c.is_alphanumeric() || c == '_' => self.scan_identifier(c),
            c => return Err(ScanningError::UnexpectedCharacter(c)),
        }
    }

    pub fn scan_identifier(&mut self, begin: char) -> ScanningResult {
        let mut ident = String::new();
        ident.push(begin);

        loop {
            match self.peek() {
                Some(c) if c.is_alphanumeric() || c == '_' => ident.push(self.advance().unwrap()),
                _ => {
                    break;
                }
            }
        }

        Ok(match self.keyword(&ident) {
            Some(k) => ScanningProduct::Token(k),
            None => ScanningProduct::Token(Token::Identifier(ident)),
        })
    }

    pub fn scan_numerics(&mut self, begin: char) -> ScanningResult {
        let mut text = String::new();
        text.push(begin);

        while self.peek().unwrap().is_numeric() {
            text.push(self.advance().unwrap());
        }

        if self.peek().unwrap() == '.' {
            text.push(self.advance().unwrap());
            while self.peek().unwrap().is_numeric() {
                text.push(self.advance().unwrap());
            }

            match text.parse::<f64>() {
                Ok(f) => Ok(ScanningProduct::Token(Token::FloatLiteral(f))),
                Err(_) => Err(ScanningError::InvalidLiteral),
            }
        } else {
            match text.parse::<i64>() {
                Ok(i) => Ok(ScanningProduct::Token(Token::FloatLiteral(i as f64))),
                Err(_) => Err(ScanningError::InvalidLiteral),
            }
        }
    }
}
