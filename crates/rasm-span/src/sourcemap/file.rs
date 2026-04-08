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

pub struct SourceFile {
    pub content: Arc<str>,
    pub name: String,
    pub line_starts: Vec<u32>,
    pub start_pos: usize,
}

impl SourceFile {
    pub fn new(content: String, name: String, start_pos: usize) -> Self {
        let lines = content
            .lines()
            .scan(0, |acc, line| {
                let start = *acc;
                *acc += line.len() as u32 + 1;

                Some(start)
            })
            .collect();

        Self {
            content: Arc::from(content),
            name,
            line_starts: lines,
            start_pos,
        }
    }

    pub fn lookup_line(&self, pos: usize) -> Option<usize> {
        self.line_starts
            .partition_point(|x| *x as usize <= pos)
            .checked_sub(1)
    }

    pub fn lookup_pos_as_location(&self, pos: usize) -> (usize, usize) {
        let relative = pos - self.start_pos;
        match self.lookup_line(relative) {
            Some(a) => {
                let line = a + 1;
                let line_start = self.line_starts[a];
                let column = relative - line_start as usize;

                (line, column)
            }
            None => (0, relative),
        }
    }
}
