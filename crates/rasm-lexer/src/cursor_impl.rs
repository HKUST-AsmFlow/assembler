use crate::{
    cursor::Cursor,
    token::{NumericBase, Token, TokenKind},
    utils::UnicodeCharUtils,
};

impl<'src> Cursor<'src> {
    pub fn comment(&mut self) -> TokenKind {
        self.bump_until(b'\n');

        TokenKind::Comment
    }

    pub fn identifier(&mut self) -> TokenKind {
        self.bump_while(char::is_xid_continue);

        TokenKind::Identifier
    }

    pub fn number(&mut self) -> TokenKind {
        let kind = TokenKind::Number;

        if ['+', '-'].contains(&self.peek()) {
            self.bump();
        }

        let base = match self.peek() {
            'b' => NumericBase::Binary,
            'o' => NumericBase::Octal,
            'x' => NumericBase::Hexadecimal,
            _ => NumericBase::Decimal,
        };
        self.bump();

        self.bump_while(|c| c.is_digit(base.as_radix()));

        kind(base)
    }

    pub fn string(&mut self) -> TokenKind {
        let mut terminated = false;

        while let Some(c) = self.bump() {
            match c {
                '"' => {
                    terminated = true;
                    break;
                }
                '\\' if ['\\', '\"'].contains(&self.peek()) => {
                    self.bump();
                }
                _ => (),
            }
        }

        TokenKind::String { terminated }
    }

    pub fn whitespace(&mut self) -> TokenKind {
        self.bump_while(char::is_whitespace);

        TokenKind::Whitespace
    }

    pub fn next_token(&mut self) -> Token {
        let Some(c) = self.bump() else {
            return Token::new(TokenKind::Eof, 0);
        };

        let kind = match c {
            '\n' => TokenKind::LineBreak,

            ';' => self.comment(),
            c if c.is_xid_start() => self.identifier(),
            '#' => self.number(),
            '"' => self.string(),
            c if c.is_whitespace() => self.whitespace(),

            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,

            _ => TokenKind::Unknown,
        };

        let token = Token::new(kind, self.position_within_token());
        self.reset_position_within_token();

        token
    }
}
