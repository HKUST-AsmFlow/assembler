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

use std::path::PathBuf;

use rasm_span::sourcemap::build_source_map;

use crate::parse::ParserSession;

pub mod early;
pub mod parse;

pub fn build_session(input_files: &[PathBuf]) -> Session {
    Session {
        parser: ParserSession::new(build_source_map(input_files)),
    }
}

pub struct Session {
    pub parser: ParserSession,
}
