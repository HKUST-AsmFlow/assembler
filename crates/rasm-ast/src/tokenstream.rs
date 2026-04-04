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

use std::sync::Arc;

use rasm_span::Span;

use crate::token::{Delimiter, Token};

pub enum TokenTree {
    Singleton(Token),
    Delimited(DelimiterSpans, Delimiter, TokenStream),
}

pub struct DelimiterSpans {
    pub open: Span,
    pub close: Span,
}

impl DelimiterSpans {
    pub fn new(open: Span, close: Span) -> Self {
        Self { open, close }
    }
}

pub struct TokenStream(pub(crate) Arc<Vec<TokenTree>>);

impl TokenStream {
    pub fn new(buf: Vec<TokenTree>) -> Self {
        Self(Arc::new(buf))
    }
}
