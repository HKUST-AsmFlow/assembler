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
    token::{Delimiter, Token, TokenKind},
    tokenstream::{DelimiterSpans, TokenStream, TokenTree},
};
use rasm_errors::DiagnosticResult;

use crate::lexer::Lexer;

impl<'session, 'src> Lexer<'session, 'src> {
    pub(crate) fn lex_to_tokenstream(
        &mut self,
        delimited: bool,
    ) -> DiagnosticResult<'session, TokenStream> {
        self.bump();

        let mut buf = Vec::new();
        loop {
            if let Some(delimiter) = self.token.kind.open_delimiter() {
                buf.push(match self.lex_to_tokentree_open_delimiter(delimiter) {
                    Ok(stream) => stream,
                    Err(diags) => return Err(diags),
                });
            } else if let Some(_) = self.token.kind.close_delimiter() {
                return if delimited {
                    Ok(TokenStream::new(buf))
                } else {
                    Err(vec![])
                };
            } else if self.token.kind == TokenKind::Eof {
                return if delimited {
                    Err(vec![])
                } else {
                    Ok(TokenStream::new(buf))
                };
            } else {
                buf.push(TokenTree::Singleton(self.bump()));
            }
        }
    }

    pub(crate) fn lex_to_tokentree_open_delimiter(
        &mut self,
        delimiter: Delimiter,
    ) -> DiagnosticResult<'session, TokenTree> {
        let open_span = self.token.span;

        let stream = self.lex_to_tokenstream(true)?;
        if let Some(close_delimiter) = self.token.kind.close_delimiter()
            && delimiter == close_delimiter
        {
            self.bump();
        } else {
            todo!()
        }

        let delimiter_span = DelimiterSpans::new(open_span, self.token.span);

        Ok(TokenTree::Delimited(delimiter_span, delimiter, stream))
    }

    fn bump(&mut self) -> Token {
        let next_token = self.next_token_from_cursor();
        mem::replace(&mut self.token, next_token)
    }
}
