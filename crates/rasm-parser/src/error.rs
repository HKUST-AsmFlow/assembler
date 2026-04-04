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

use crate::lexer::token::TokenKind;

pub enum ParseErrorKind {
    UnexpectedEof,
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
}

pub struct ParseError {
    kind: ParseErrorKind,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> Self {
        Self { kind }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
