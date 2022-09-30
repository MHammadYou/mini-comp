use std::collections::HashMap;

use crate::lexer::*;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_col: usize,
    pub codepoint_offset: usize,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    keywords: HashMap<String, TokenType>
}

macro_rules! try_consume {
    ($self: tt, $($inner:tt),*) => {
        if let Some(c) = $self.chars.peek() {
            if try_consume!(impl c, $($inner), *) {
                let tmp = *c;
                $self.consume();
                Some(tmp)
            } else {
                None
            }
        } else {
            None
        }
    };
    (impl , ) => (false);
    (impl $c:tt, $item:tt) => (*$c == $item);
    (impl $c:tt, $item:tt, $($rest:tt), *) => (try_consume!(impl $c, $item) || try_consume!(impl $c, $($rest), *))
}




impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            cur_line: 1,
            cur_col: 1,
            codepoint_offset: 0,
            chars: chars.chars().peekable(),
            
            keywords: HashMap::from([
                (String::from("true"), TokenType::Terminal(String::from("true"))),
                (String::from("false"), TokenType::Terminal(String::from("false"))),
                (String::from("let"), TokenType::Terminal(String::from("let"))),
                (String::from("def"), TokenType::Terminal(String::from("def"))),
                (String::from("print"), TokenType::Terminal(String::from("print"))),
                (String::from("nil"), TokenType::Terminal(String::from("nil"))),
                (String::from("if"), TokenType::Terminal(String::from("if"))),
                (String::from("while"), TokenType::Terminal(String::from("wjile"))),
                (String::from("for"), TokenType::Terminal(String::from("for"))),
            ])
        }
    }

    pub fn get_tokens(&mut self) -> Vec<TokenType> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token() {
                Ok(TokenType::EOF) => break tokens.push(TokenType::EOF),
                Ok(token) => tokens.push(token),
                Err(err) => println!("{}", err)
            }
        }

        tokens
    }

    fn parse_numbers(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut raw = start.to_string();
        let radix = 10;
        let mut hint = NumericHint::Integer;

        if start == '.' {
            raw += &self.parse_digits(radix, false)?;
            println!("HIt");
            hint = NumericHint::FloatingPoint;
        } else if start.is_digit(radix) {

            raw += &self.parse_digits(radix, true)?;

            if let Some(c) = try_consume!(self, '.') {
                raw.push(c);
                raw += &self.parse_digits(radix, false)?;
                hint = NumericHint::FloatingPoint;
            }
        } else {
            println!("Compiler bug if this line hits");
            return Err(LexerError::NumericLiteralInvalidChar { raw, invalid: start})
        }

        Ok(TokenType::Numeric { raw, hint })
    }

    fn parse_digits(&mut self, radix: u32, allow_empty: bool) -> Result<String, LexerError> {
        let mut raw = String::new();
        loop {
            match self.chars.peek() {
                None => {
                    break if allow_empty || raw.len() > 0 {
                        Ok(raw)
                    } else {
                        Err(LexerError::MissingExpectedSymbol { expected: "0 - 9", found: TokenType::EOF })
                    }
                }
                Some(c) if c.is_digit(radix) || (*c == '_' && raw.len() > 0) => raw.push(*c),
                Some(c) if !c.is_ascii_alphabetic() && *c != '_' => break Ok(raw),
                Some(c) => break Err(LexerError::NumericLiteralInvalidChar { raw, invalid: *c })
            }
            self.chars.next();
        }
    }

    fn parse_string(&mut self) -> Result<TokenType, LexerError> {
        let mut buf = String::new();
        loop {
            match self.chars.next() {
                Some('"') => break Ok(TokenType::String(buf)),
                Some(c) => buf.push(c),
                None => break Err(LexerError::MissingExpectedSymbol { expected: "\"", found: TokenType::EOF })
            }
        }
    }

    fn parse_identifiers_or_terminals(&mut self, start: char) -> TokenType {
        let mut buf = start.to_string();
        loop {
            match self.chars.peek() {
                Some(c) if c.is_alphanumeric() || c.is_digit(10) || *c == '_' => {
                    buf.push(self.chars.next().unwrap())
                },
                _ => break self.tag_identifier(buf)
            }
        }
    }

    fn tag_identifier(&self, ident: String) -> TokenType {
        // if match ident.as_ref() {
        //     "false" | "true"` | "let" | "def" | "print" | "nil" | "if" | "while" | "for"  => true,
        //     _ => false
        // } {
        //     TokenType::Terminal(ident)
        // } else {
        //     TokenType::wIdentifier(ident)
        // }

        match self.keywords.get(&ident) {
            Some(_) => TokenType::Terminal(ident),
            None => TokenType::Identifier(ident)
        }
    }

    fn check_next(&mut self, next: char) -> bool {
        match self.chars.next() {
            Some(c) => {
                c == next
            },
            None => false
        }
    }

    fn transform_to_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::OpenParen }),
            ')' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::CloseParen }),
            '{' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::OpenCurly }),
            '}' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::CloseCurly }),
            '0' ..= '9' | '.' => self.parse_numbers(c),
            ';' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Separator }),
            '!' => {
                if self.check_next('=') {
                    return Ok(TokenType::Operator(OperatorKind::BangEqual))
                }
                Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Bang })
            },
            '=' => {
                if self.check_next('=') {
                    return Ok(TokenType::Operator(OperatorKind::EqualEqual))
                }
                Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Equal })
            },

            '+' => Ok(TokenType::Operations { raw: c, kind: OperationKind::Plus }),
            '-' => Ok(TokenType::Operations { raw: c, kind: OperationKind::Minus }),
            '*' => Ok(TokenType::Operations { raw: c, kind: OperationKind::Star }),
            '/' => Ok(TokenType::Operations { raw: c, kind: OperationKind::Slash }),

            '<' => {
                if self.check_next('=') {
                    return Ok(TokenType::Operator(OperatorKind::LessEqual))
                }
                Ok(TokenType::Operator(OperatorKind::Less))
            },
            '>' => {
                if self.check_next('=') {
                    return Ok(TokenType::Operator(OperatorKind::GreaterEqual))
                }
                Ok(TokenType::Operator(OperatorKind::Greater))
            }


            '"' => self.parse_string(),
            c if c.is_alphanumeric() || c == '_' => Ok(self.parse_identifiers_or_terminals(c)),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string() })
        }
    }

    pub fn consume(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.cur_col += 1;
                if c == '\n' {
                    self.cur_line += 1;
                    self.cur_col += 1;
                }
                self.codepoint_offset += 1;
                Some(c)
            },
            None => None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();

        if let Some(c) = self.consume() {
            self.transform_to_type(c)
        } else {
            Ok(TokenType::EOF)
        }
    }
}