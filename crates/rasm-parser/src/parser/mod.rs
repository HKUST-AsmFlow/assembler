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

use std::{iter::Peekable, sync::Arc};

use rasm_span::sourcemap::SourceMap;

use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    lexer::token::{Token, TokenKind},
};

mod directive;
mod line;
mod program;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    source_map: Arc<SourceMap>,
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub(crate) fn new(iter: I, source_map: SourceMap) -> Self {
        Self {
            source_map: Arc::new(source_map),
            tokens: iter.peekable(),
        }
    }

    pub(crate) fn bump(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub(crate) fn expect(&mut self, kind: TokenKind) -> ParseResult<Token> {
        self.skip(&[TokenKind::Whitespace]);

        match self.bump() {
            Some(token) if token.kind == kind => Ok(token),
            Some(token) => Err(ParseError::new(ParseErrorKind::UnexpectedToken {
                expected: kind,
                found: token.kind,
            })),
            None => Err(ParseError::new(ParseErrorKind::UnexpectedEof)),
        }
    }

    pub(crate) fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub(crate) fn skip(&mut self, kinds: impl AsRef<[TokenKind]>) {
        while let Some(token) = self.peek() {
            if kinds.as_ref().contains(&token.kind) {
                self.bump();
            } else {
                break;
            }
        }
    }
}

pub trait Parseable<T> {
    fn parse(&mut self) -> ParseResult<T>;
}
