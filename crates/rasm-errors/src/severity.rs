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
use annotate_snippets::Level;

pub enum Severity {
    Bug,
    Error,
    Note,
}

impl From<Severity> for Level<'static> {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Bug => Level::ERROR.with_name("error: internal assembler error"),
            Severity::Error => Level::ERROR,
            Severity::Note => Level::NOTE,
        }
    }
}
