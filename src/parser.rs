use crate::ast::*;
use crate::scanner::*;

use std::iter::Iterator;
use std::iter::Peekable;

type ItemType = Token;

#[derive(Debug, Clone)]
pub enum ParsingError {
    UnexpectedToken(ItemType),
    UnexpectedEndOfInput,
}

type ParsingResult<T> = Result<T, ParsingError>;

pub trait TokenSource {
    fn next(&mut self) -> Option<ItemType>;
    fn peek(&mut self) -> Option<&ItemType>;

    fn expect_next(&mut self) -> ParsingResult<ItemType> {
        match TokenSource::next(self) {
            None => Err(ParsingError::UnexpectedEndOfInput),
            Some(t) => Ok(t),
        }
    }

    fn expect_token(&mut self, token: Token) -> ParsingResult<ItemType> {
        match TokenSource::expect_next(self)? {
            t if t == token => Ok(t),
            t => Err(ParsingError::UnexpectedToken(t)),
        }
    }

    fn expect_identifier(&mut self) -> ParsingResult<String> {
        let token = TokenSource::expect_next(self)?;
        match token {
            Token::Identifier(s) => Ok(s),
            _ => Err(ParsingError::UnexpectedToken(token)),
        }
    }
}

impl<T> TokenSource for Peekable<T>
where
    T: Iterator<Item = ItemType>,
{
    fn next(&mut self) -> Option<ItemType> {
        std::iter::Iterator::next(self)
    }

    fn peek(&mut self) -> Option<&ItemType> {
        self.peek()
    }
}

pub fn parse_program(tokens: &mut impl TokenSource) -> ParsingResult<Program> {
    let mut program = Program::new();

    'parsing: loop {
        let token = tokens.next();
        if token.is_none() {
            break 'parsing;
        }
        token.unwrap();

        // identifier
        let ident = tokens.expect_identifier()?;

        // arg list
        tokens.expect_token(Token::LeftParen)?;
        tokens.expect_token(Token::RightParen)?;

        // body
        tokens.expect_token(Token::LeftBrace)?;

        let statements = parse_statements(tokens)?;

        program.functions.push(FuncDecl {
            name: ident,
            body: statements,
        });
    }

    Ok(program)
}

pub fn parse_statements(tokens: &mut impl TokenSource) -> ParsingResult<Vec<Stmt>> {
    let mut output = Vec::new();

    'parsing: loop {
        let token = tokens.next();
        if token.is_none() {
            break 'parsing;
        }
        let token = token.unwrap();

        match &token {
            Token::Return => {
                output.push(Stmt::Return(Some(parse_expr_bp(tokens, 0)?)));
            }
            Token::Let => {
                let ident = tokens.expect_identifier()?;
                tokens.expect_token(Token::Equals)?;

                output.push(Stmt::Assignment(ident, parse_expr_bp(tokens, 0)?));
            }
            Token::RightBrace => break 'parsing,
            _ => {
                return Err(ParsingError::UnexpectedToken(token));
            }
        }

        tokens.expect_token(Token::Semicolon);
    }

    Ok(output)
}

pub fn infix_binding_power(t: &Token) -> Option<(u8, u8)> {
    match t {
        Token::Plus => Some((1, 2)),
        Token::Minus => Some((1, 2)),
        Token::Star => Some((3, 4)),
        Token::Slash => Some((3, 4)),
        _ => None,
    }
}

pub fn prefix_binding_power(t: &Token) -> Option<((), u8)> {
    match t {
        Token::Minus => Some(((), 5)),
        _ => None,
    }
}

pub fn parse_expr_bp(lexer: &mut impl TokenSource, min_bp: u8) -> ParsingResult<Expr> {
    let token = lexer.expect_next()?;
    // atoms
    let mut lhs = match &token {
        Token::FloatLiteral(f) => Expr::Constant(*f),
        Token::Identifier(i) => match lexer.peek() {
            Some(t) if *t == Token::LeftParen => {
                lexer.next();

                let mut exprs = Vec::new();
                if *lexer.peek().unwrap() != Token::RightParen {
                    loop {
                        let e = parse_expr_bp(lexer, 0)?;
                        exprs.push(Box::new(e));
                        match lexer.expect_next()? {
                            t if t == Token::RightParen => {
                                break;
                            }
                            t if t == Token::Comma => {
                                continue;
                            }
                            t => Err(ParsingError::UnexpectedToken(t))?,
                        }
                    }
                } else {
                    lexer.next();
                }

                Expr::Call(i.clone(), exprs)
            }
            _ => Expr::Symbol(i.clone()),
        },
        Token::LeftParen => {
            let e = parse_expr_bp(lexer, 0)?;
            lexer.expect_token(Token::RightParen)?;
            e
        }
        t if prefix_binding_power(t).is_some() => {
            let ((), r_bp) = prefix_binding_power(t).unwrap();

            let rhs = parse_expr_bp(lexer, r_bp)?;
            let fnc = match t {
                Token::Minus => Expr::Unary(Op::Sub, Box::new(rhs)),
                _ => unreachable!(), // at this point we know we have a valid unary operator, so this is fine
            };

            fnc
        }
        _ => return Err(ParsingError::UnexpectedToken(token)),
    };

    loop {
        let (t, (l_bp, r_bp)) = match lexer.peek() {
            Some(t) if infix_binding_power(t).is_some() => {
                (t.clone(), infix_binding_power(t).unwrap())
            }
            _ => break,
        };
        if l_bp < min_bp {
            break;
        }

        lexer.next().unwrap();
        let rhs = parse_expr_bp(lexer, r_bp)?;

        let op = match &t {
            Token::Plus => Op::Add,
            Token::Minus => Op::Sub,
            Token::Star => Op::Mul,
            Token::Slash => Op::Div,
            _ => unreachable!(),
        };

        lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
    }

    Ok(lhs)
}
