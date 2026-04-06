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

use std::{mem, sync::Arc};

use rasm_span::Span;

use crate::token::{Delimiter, Token};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TokenTree {
    Singleton(Token),
    Delimited(DelimiterSpans, Delimiter, TokenStream),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct DelimiterSpans {
    pub open: Span,
    pub close: Span,
}

impl DelimiterSpans {
    pub fn new(open: Span, close: Span) -> Self {
        Self { open, close }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct TokenStream(pub(crate) Arc<Vec<TokenTree>>);

impl TokenStream {
    pub fn new(buf: Vec<TokenTree>) -> Self {
        Self(Arc::new(buf))
    }

    pub fn get(&self, i: usize) -> Option<&TokenTree> {
        self.0.get(i)
    }
}

#[derive(Clone, Debug)]
pub struct TokenStreamCursor {
    // Cursor for the current (innermost) token stream. The index within the
    // cursor can point to any token tree in the stream (or one past the end).
    // The delimiters for this token stream are found in `self.stack.last()`;
    // if that is `None` we are in the outermost token stream which never has
    // delimiters.
    current: TokenTreeCursor,
    // Token streams surrounding the current one. The index within each cursor
    // always points to a `TokenTree::Delimited`.
    stack: Vec<TokenTreeCursor>,
}

impl TokenStreamCursor {
    pub fn new(stream: TokenStream) -> Self {
        Self {
            current: TokenTreeCursor::new(stream),
            stack: vec![],
        }
    }

    pub fn next(&mut self) -> Token {
        loop {
            return if let Some(tree) = self.current.current() {
                match tree {
                    &TokenTree::Singleton(ref token) => {
                        let result = token.clone();
                        self.current.bump();

                        result
                    }
                    &TokenTree::Delimited(spans, delimiter, ref stream) => {
                        let trees = TokenTreeCursor::new(stream.clone());
                        self.stack.push(mem::replace(&mut self.current, trees));

                        Token::new(delimiter.left(), spans.open)
                    }
                }
            } else if let Some(parent) = self.stack.pop() {
                let Some(&TokenTree::Delimited(spans, delimiter, _)) = parent.current() else {
                    panic!("parent should be Delimited");
                };

                self.current = parent;
                self.current.bump();

                Token::new(delimiter.right(), spans.close)
            } else {
                Token::DUMMY_TOKEN
            };
        }
    }
}

#[derive(Clone, Debug)]
pub struct TokenTreeCursor {
    stream: TokenStream,
    // Points to the current token tree in the stream. In `TokenStreamCursor::curr`,
    // this can be any token tree. In `TokenStreamCursor::stack`, this is always a
    // `TokenTree::Delimited`.
    index: usize,
}

impl TokenTreeCursor {
    pub fn new(stream: TokenStream) -> Self {
        Self { stream, index: 0 }
    }

    pub fn bump(&mut self) {
        self.index += 1;
    }

    pub fn current(&self) -> Option<&TokenTree> {
        self.stream.get(self.index)
    }
}
