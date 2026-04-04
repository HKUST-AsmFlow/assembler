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

use std::str::Chars;

pub struct Cursor<'src> {
    chars: Chars<'src>,
    remaining_length: usize,
}

impl<'src> Cursor<'src> {
    pub const EOF_CHAR: char = '\0';

    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
            remaining_length: input.len(),
        }
    }

    pub fn as_str(&self) -> &'src str {
        self.chars.as_str()
    }

    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn bump_until(&mut self, b: u8) {
        self.chars = memchr::memchr(b, self.as_str().as_bytes())
            .map(|index| &self.as_str()[index..])
            .unwrap_or("")
            .chars();
    }

    pub fn bump_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while predicate(self.peek()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn position_within_token(&self) -> usize {
        self.remaining_length - self.chars.as_str().len()
    }

    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(Self::EOF_CHAR)
    }

    pub fn reset_position_within_token(&mut self) {
        self.remaining_length = self.chars.as_str().len();
    }
}
