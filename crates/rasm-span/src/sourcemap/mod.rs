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

use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

pub use file::SourceFile;

mod file;

pub struct SourceMap {
    files: Vec<Arc<SourceFile>>,
    map: HashMap<String, usize>,
    next_start_pos: usize,
}

impl SourceMap {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            map: HashMap::new(),
            next_start_pos: 0,
        }
    }

    pub fn add_source_file(&mut self, name: String, source: String) {
        let next_idx = self.files.len();
        let start_pos = self.next_start_pos;

        let file = Arc::new(SourceFile::new(source, name.clone(), start_pos));
        self.files.push(file);
        self.map.insert(name, next_idx);

        self.next_start_pos += self.files.last().unwrap().content.len() + 1;
    }
}

pub fn build_source_map(input_files: &[PathBuf]) -> SourceMap {
    let mut map = SourceMap::new();
    input_files.iter().for_each(|file| {
        map.add_source_file(
            file.absolute().unwrap().to_string_lossy().to_string(),
            fs::read_to_string(file).unwrap(),
        )
    });

    map
}
