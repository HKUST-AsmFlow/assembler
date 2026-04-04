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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericBase {
    Binary,
    Decimal,
    Hexadecimal,
    Octal,
}

impl NumericBase {
    pub fn as_radix(self) -> u32 {
        match self {
            NumericBase::Binary => 2,
            NumericBase::Decimal => 10,
            NumericBase::Hexadecimal => 16,
            NumericBase::Octal => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Colon,
    Comment,
    Comma,
    Dot,
    Eof,
    Identifier,
    LeftBracket,
    LineBreak,
    Number(NumericBase),
    RightBracket,
    String { terminated: bool },
    Unknown,
    Whitespace,
}

#[derive(Debug, Default)]
pub struct Token {
    pub kind: TokenKind = TokenKind::Eof,
    pub length: usize = 0,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, length: usize) -> Self {
        Self { kind, length }
    }
}
