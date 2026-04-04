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

use crate::lexer::cursor::Cursor;
use crate::lexer::token::{Token, TokenKind};

mod cursor;
mod token;

impl<'str> Cursor<'str> {
    pub fn next_token(&mut self) -> Token {
        let Some(c) = self.bump() else {
            return Token::new(TokenKind::Eof, 0);
        };

        let kind = match c {
            _ => TokenKind::Unknown,
        };
        
        let token = Token::new(kind, self.position_within_token());
        self.reset_position_within_token();

        token
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut cursor = Cursor::new(input);
    iter::from_fn(move || {
        let token = cursor.next_token();
        if token.kind != TokenKind::Eof { Some(token) } else { None}
    })
}
