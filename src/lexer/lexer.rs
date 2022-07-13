use lexer::*;

pub struct Lexer<'a> {
    pub cur_line: usize,
    pub cur_col: usize,

    pub codepoint_offset: usize,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    balancing_state: std::collections::HashMap<char, BalancingDepthType>,
}


impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            cur_line: 1,
            cur_col: 1,
            codepoint_offset: 0,
            chars: chars.chars().peekable(),
            balancing_state: std::collections::HashMap::new(),
        }
    }

    fn map_balance(c: &char) -> char {
        match c {
            '(' => ')',
            ')' => '(',
            '{' => '}',
            '}' => '{',
            '[' => ']',
            ']' => '[',
            _ => panic!("Invalid symbol")
        }
    }

    fn push_symbol(&mut self, c: &char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            *v += 1;
            *v - 1
        } else {
            self.balancing_state.insert(*c, 1);
            0
        }
    }
    
    fn pop_symbol(&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
        if let Some(v) = self.balancing_state.get_mut(&Lexer::map_balance(&c)) {
            if *v >= 1 {
                *v -= 1;
                Ok(*v)
            } else {
                Err(LexerError::MissingBalancedSymbol { symbol: *c, open: Lexer::map_balance(&c) })
            }
        } else {
            Err(LexerError::MissingBalancedSymbol { symbol: *c, open: Lexer::map_balance(&c) })
        }
    }

    fn parse_numbers(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut seen_dot = false;
        let mut num = start.to_string();
        let radix = 10;

        if start == '.' {
            seen_dot = true;
        }

        loop {
            match self.chars.peek() {
                Some(c) if *c == '.' && !seen_dot => {
                    num.push(*c);
                    self.consume();
                    seen_dot = true;
                },
                Some(c) if c.is_digit(radix) => {
                    num.push(*c);
                    self.consume();
                },
                Some(c) if c.is_ascii_alphabetic() || c.is_digit(10) => {
                    num.push(*c);
                    return Err(LexerError::NumericLiteralInvalidChar { raw: num });
                },
                _ => break Ok(TokenType::Numeric { raw: num, hint: if seen_dot { NumericHint::FloatingPoint } else { NumericHint::Integer } })
            }
        }
    }

    fn transform_to_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Open(self.push_symbol(&c)) }),
            ')' | ']' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?) }),
            '0' ..= '9' | '.' => self.parse_numbers(c),
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