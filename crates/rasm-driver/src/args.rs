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

use getopts::{HasArg, Matches, Occur, Options};

pub struct StructuredOptions {
    input_files: Vec<PathBuf>,
    output_file: PathBuf,
}

impl StructuredOptions {
    pub fn from_matches(matches: Matches) -> Self {
        let input_files = matches.free.iter().map(PathBuf::from).collect();
        let output_file = matches
            .opt_get_default("o", PathBuf::from("a.out"))
            .unwrap();

        Self {
            input_files,
            output_file,
        }
    }

    pub fn input_files(&self) -> &[PathBuf] {
        &self.input_files
    }
}

pub fn rasm_optgroups() -> Options {
    let mut optgroups = Options::new();
    optgroups.opt(
        "o",
        "",
        "Output file of the assembler",
        "<OUTPUT>",
        HasArg::Yes,
        Occur::Optional,
    );

    optgroups
}
