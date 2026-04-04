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

use rasm_ast::Program;

use crate::{
    error::ParseResult,
    lexer::token::{Token, TokenKind},
    parser::{Parseable, Parser},
};

impl<I> Parseable<Program> for Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn parse(&mut self) -> ParseResult<Program> {
        let mut lines = Vec::new();

        while let Some(token) = self.peek() {
            if token.kind == TokenKind::Eof {
                break;
            }

            lines.push(self.parse()?);
        }

        Ok(Program { lines })
    }
}
