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

use rasm_span::Span;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Delimiter {
    Bracket,
}

impl Delimiter {
    pub fn left(self) -> TokenKind {
        match self {
            Self::Bracket => TokenKind::LeftBracket,
        }
    }

    pub fn right(self) -> TokenKind {
        match self {
            Self::Bracket => TokenKind::RightBracket,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenKind {
    Colon,
    Comma,
    Dot,
    Eof,
    Identifier(String),
    LeftBracket,
    Literal,
    RightBracket,
}

impl TokenKind {
    pub fn close_delimiter(&self) -> Option<Delimiter> {
        match self {
            Self::RightBracket => Some(Delimiter::Bracket),
            _ => None,
        }
    }

    pub fn open_delimiter(&self) -> Option<Delimiter> {
        match self {
            Self::LeftBracket => Some(Delimiter::Bracket),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub const DUMMY_TOKEN: Token = Token::new(TokenKind::Eof, Span::DUMMY_SPAN);

    pub const fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
