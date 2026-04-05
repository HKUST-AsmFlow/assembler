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

use std::mem;

use rasm_ast::{
    token::{Token, TokenKind as AstTokenKind},
    tokenstream::TokenStream,
};
use rasm_errors::DiagnosticResult;
use rasm_lexer::{
    cursor::Cursor,
    token::{NumericBase, TokenKind},
};
use rasm_session::parse::ParserSession;
use rasm_span::Span;

mod tokenstream;

pub fn into_tokenstream<'session, 'src>(
    session: &'session ParserSession,
    src: &'src str,
    start_pos: u32,
) -> DiagnosticResult<'session, TokenStream> {
    let mut lexer = Lexer::new(src, start_pos, session);
    lexer.lex_to_tokenstream(false)
}

pub struct Lexer<'session, 'src> {
    cursor: Cursor<'src>,
    session: &'session ParserSession,
    source: &'src str,
    pos: u32,
    start_pos: u32,
    token: Token,
}

impl<'session, 'src> Lexer<'session, 'src> {
    pub fn new(source: &'src str, start_pos: u32, session: &'session ParserSession) -> Self {
        Self {
            cursor: Cursor::new(source),
            session,
            source,
            pos: start_pos,
            start_pos,
            token: Token::DUMMY_TOKEN,
        }
    }

    pub(crate) fn make_span(&self, mut lo: u32, mut hi: u32) -> Span {
        if lo > hi {
            mem::swap(&mut lo, &mut hi);
        }

        Span::new(lo, hi - lo)
    }

    pub(crate) fn next_token_from_cursor(&mut self) -> Token {
        loop {
            let token = self.cursor.next_token();
            let start = self.pos;
            self.pos += token.length as u32;

            let kind = match token.kind {
                TokenKind::Comment | TokenKind::Whitespace => continue,
                TokenKind::Identifier => self.identifier(start),
                TokenKind::Number(base) => self.number(start, base),
                TokenKind::String { terminated } => self.string(start, terminated),
                TokenKind::Colon => AstTokenKind::Colon,
                TokenKind::Comma => AstTokenKind::Comma,
                TokenKind::Dot => AstTokenKind::Dot,
                TokenKind::LeftBracket => AstTokenKind::LeftBracket,
                TokenKind::RightBracket => AstTokenKind::RightBracket,
                _ => continue,
            };

            let span = self.make_span(start, self.pos);
            return Token::new(kind, span);
        }
    }

    #[inline]
    pub(crate) fn src_index(&self, pos: u32) -> usize {
        (pos - self.start_pos) as usize
    }

    pub(crate) fn str_from(&self, start: u32) -> &'src str {
        self.str_from_to(start, self.pos)
    }

    pub(crate) fn str_from_to(&self, start: u32, end: u32) -> &'src str {
        &self.source[self.src_index(start)..self.src_index(end)]
    }

    fn identifier(&mut self, start_pos: u32) -> AstTokenKind {
        let str = self.str_from(start_pos);

        AstTokenKind::Identifier(str.to_string())
    }

    fn number(&mut self, start_pos: u32, base: NumericBase) -> AstTokenKind {
        todo!()
    }

    fn string(&mut self, start_pos: u32, temrinated: bool) -> AstTokenKind {
        todo!()
    }
}
