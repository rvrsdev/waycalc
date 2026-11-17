/**
One element of the expression

Ex. `2/3+3` -> `[Number(2.0), Divide, Number(3.0), Plus, Number(3.0)]`
*/
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
}

/// Errors that can ccur while parsing an expression
#[derive(Debug, PartialEq)]
pub enum CalcError {
    /// Expression has a symbol that parser could not parse
    InvalidCharacter(char),

    /// Incorrect written number, ex. `2..3`
    InvalidNumber(String),

    /// Ex. `2+`, missing number after an operator
    UnexpectedEnd,

    /// Ex., `+2`, unexpected token before an operator
    UnexpectedToken(Token),

    /// Not closed paren expression, ex. `(2+3`
    MissingRightParen,

    /// You can't divide by zero!
    DivisionByZero,

    /// Extra tokens after correct expression, ex. `2+3 4`
    TrailingTokens,
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::InvalidCharacter(ch) => {
                write!(f, "Invalid character: {ch}")
            }
            CalcError::InvalidNumber(number) => {
                write!(f, "Invalid Number: {number}")
            }
            CalcError::UnexpectedEnd => {
                write!(f, "Expression ended unexpectedly")
            }
            CalcError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {token:?}")
            }
            CalcError::MissingRightParen => {
                write!(f, "Expected closing paren")
            }
            CalcError::DivisionByZero => {
                write!(f, "Division by zero")
            }
            CalcError::TrailingTokens => {
                write!(f, "Trailing symbols after an expression")
            }
        }
    }
}

/**
Accepting an input and returning result
```rust
let result = calc::evaluate("2/3+3");
```
*/
pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    // 1. Tokenizing input
    let tokens = tokenize(input)?;

    // 2. Creating parser with position on first token
    let mut parser = Parser::new(tokens);

    // 3. Parsing and calculating expression
    let result = parser.parse_expression()?;

    // 4. Checking for extra tokens
    if parser.position != parser.tokens.len() {
        return Err(CalcError::TrailingTokens);
    }

    Ok(result)
}

/// Transforming an expression into an array of tokens
pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
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

            _ => return Err(CalcError::InvalidCharacter(ch)),
        }
    }

    Ok(tokens)
}

/**
Reads a sequence of digits and a single period as a number

Ex. from `12.5+3` reading `12.5` and leaves `+3` for main cycle
*/
fn read_number(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<f64, CalcError> {
    let mut number = String::new();
    let mut has_dot = false;

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                number.push(ch);
                chars.next();
            }

            '.' => {
                // Second dot ending a number.
                // Ex. 2.3.4 will beсome Number(2.3), and then InvalidCharacter error will be called.
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
        .map_err(|_| CalcError::InvalidNumber(number))
}

/**
Storing all tokens and current iterating position

A cursor for tokens array
*/
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Creating parser with tokens on position zero
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// See current token without advancing iterator position
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Take current token and advance to the next one
    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).copied();

        if token.is_some() {
            self.position += 1;
        }

        token
    }

    /**
    Addition and subtraction

    Grammar:
    ```text
    expression := term (("+" | "-") term)*
    ```
    */
    fn parse_expression(&mut self) -> Result<f64, CalcError> {
        // Getting left part: number, parentheses or expression.
        let mut left = self.parse_term()?;

        // While adding or substacting
        while let Some(Token::Plus | Token::Minus) = self.peek() {
            // Getting an operator
            let operator = self.next().expect("Оператор уже проверен через peek");

            // Getting right part
            let right = self.parse_term()?;

            left = match operator {
                Token::Plus => left + right,
                Token::Minus => left - right,

                // cannot pass here
                _ => unreachable!(),
            };
        }

        Ok(left)
    }

    /**
    Multiplication and division

    Grammar:
    ```text
    term := factor (("*" | "/") factor)*
    ```
    This method makes `*` and `/` a greater priority than `+` and `-`
    */
    fn parse_term(&mut self) -> Result<f64, CalcError> {
        let mut left = self.parse_factor()?;

        while let Some(Token::Multiply | Token::Divide) = self.peek() {
            let operator = self.next().expect("Оператор уже проверен через peek");

            let right = self.parse_factor()?;

            left = match operator {
                Token::Multiply => left * right,

                Token::Divide => {
                    // Cannot divide by zero
                    if right == 0.0 {
                        return Err(CalcError::DivisionByZero);
                    }

                    left / right
                }

                _ => unreachable!(),
            };
        }

        Ok(left)
    }

    /**
    Number, parentheses or unary minus

    Grammar:
    ```text
    factor := number | "(" expression ")" | "-" factor
    ```
    */
    fn parse_factor(&mut self) -> Result<f64, CalcError> {
        match self.next() {
            // Number
            Some(Token::Number(value)) => Ok(value),

            // Unary minus
            Some(Token::Minus) => {
                let value = self.parse_factor()?;
                Ok(-value)
            }

            // Parentheses: first, calculating an expression, then waiting for the right paren
            Some(Token::LeftParen) => {
                let value = self.parse_expression()?;

                match self.next() {
                    Some(Token::RightParen) => Ok(value),
                    _ => Err(CalcError::MissingRightParen),
                }
            }

            // +2 or single left paren handling
            Some(token) => Err(CalcError::UnexpectedToken(token)),

            // Ex. 2+ without any expression after handling.
            None => Err(CalcError::UnexpectedEnd),
        }
    }
}
