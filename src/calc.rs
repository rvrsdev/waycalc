use std::{f64, iter, str};

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum TokenError {
    InvalidCharacter(char),
    InvalidNumber(String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let number = read_number(&mut chars)?;
                tokens.push(Token::Number(number));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Multiply);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Divide);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            _ => return Err(TokenError::InvalidCharacter(ch)),
        }
    }
    Ok(tokens)
}

pub fn read_number(chars: &mut iter::Peekable<str::Chars<'_>>) -> Result<f64, TokenError> {
    let mut number = String::new();
    let mut has_dot = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                number.push(ch);
                chars.next();
            }
            '.' => {
                if has_dot {
                    break;
                }

                has_dot = true;
                number.push(ch);
                chars.next();
            }
            _ => break,
        }
    }

    number
        .parse::<f64>()
        .map_err(|_| TokenError::InvalidNumber(number))
}
