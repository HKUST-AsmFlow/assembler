/*
 * Copyright (c) 2026  AsmFlow Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::iter;

use rasm_span::sourcemap::SourceMap;

use crate::lexer::{
    cursor::Cursor,
    token::{NumericBase, Token, TokenKind},
    utils::UnicodeCharUtils,
};

mod cursor;
pub mod token;
mod utils;

impl<'str> Cursor<'str> {
    pub(crate) fn comment(&mut self) -> TokenKind {
        self.bump_until(b'\n');

        TokenKind::Comment
    }

    pub(crate) fn identifier(&mut self) -> TokenKind {
        self.bump_while(char::is_xid_continue);

        TokenKind::Identifier
    }

    pub(crate) fn number(&mut self) -> TokenKind {
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

    pub(crate) fn string(&mut self) -> TokenKind {
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

    pub(crate) fn whitespace(&mut self) -> TokenKind {
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

pub fn tokenize(
    name: &str,
    input: &str,
    source_map: &mut SourceMap,
) -> impl Iterator<Item = Token> {
    source_map.add_source_file(name.to_string(), input.to_string());

    let mut cursor = Cursor::new(input);
    iter::from_fn(move || {
        let token = cursor.next_token();
        if token.kind != TokenKind::Eof {
            Some(token)
        } else {
            None
        }
    })
}
