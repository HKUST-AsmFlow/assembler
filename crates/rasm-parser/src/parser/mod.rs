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

use std::iter::Peekable;

use crate::lexer::token::Token;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub(crate) fn new(iter: I) -> Self {
        Self {
            tokens: iter.peekable(),
        }
    }

    pub(crate) fn bump(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    pub(crate) fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
}
