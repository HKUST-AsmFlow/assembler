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
    token::Token,
    tokenstream::{TokenStream, TokenStreamCursor},
};
use rasm_session::parse::ParserSession;

mod program;

pub struct Parser<'session> {
    session: &'session ParserSession,
    token: Token = Token::DUMMY_TOKEN,
    previous_token: Token = Token::DUMMY_TOKEN,
    cursor: TokenStreamCursor,
}

impl<'session> Parser<'session> {
    pub fn new(session: &'session ParserSession, token_stream: TokenStream) -> Self {
        Self {
            session,
            cursor: TokenStreamCursor::new(token_stream),
            ..
        }
    }

    pub fn bump(&mut self) {
        let next = self.cursor.next();
        self.previous_token = mem::replace(&mut self.token, next);
    }
}
