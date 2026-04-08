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

use crate::location::Location;
use crate::Span;

mod file;

#[derive(Clone)]
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

    pub fn get_file(&self, path: String) -> Option<Arc<SourceFile>> {
        self.files.get(*self.map.get(&path)?).cloned()
    }

    pub fn lookup_file_index(&self, pos: usize) -> usize {
        self.files.partition_point(|x| x.start_pos <= pos) - 1
    }

    pub fn lookup_offset(&self, pos: usize) -> (Arc<SourceFile>, usize) {
        let file = self.lookup_file_index(pos);
        let sf = self.files.get(file).unwrap().clone();
        let offset = pos - sf.start_pos;
        (sf, offset)
    }

    pub fn lookup_source_file(&self, pos: usize) -> Arc<SourceFile> {
        let index = self.lookup_file_index(pos);
        self.files[index].clone()
    }

    pub fn lookup_source_location(&self, pos: usize) -> Location {
        let file = self.lookup_source_file(pos);
        let (line, column) = file.lookup_pos_as_location(pos);

        Location { file, line, column }
    }

    pub fn span_to_source<F, T>(&self, sp: Span, f: F) -> T
    where
        F: Fn(&str, usize, usize) -> T,
    {
        let (start_f, start_pos) = self.lookup_offset(sp.lo as usize);
        let (end_f, end_pos) = self.lookup_offset((sp.lo + sp.len - 1) as usize);

        assert_eq!(start_f.name, end_f.name);

        f(start_f.content.as_ref(), start_pos, end_pos)
    }

    pub fn span_to_snippet(&self, sp: Span) -> String {
        self.span_to_source(sp, |src, start_index, end_index| {
            src.get(start_index..end_index)
                .map(ToString::to_string)
                .unwrap()
        })
    }
}

pub fn build_source_map(input_files: &[PathBuf]) -> SourceMap {
    let mut map = SourceMap::new();
    input_files.iter().for_each(|file| {
        map.add_source_file(
            file.to_string_lossy().to_string(),
            fs::read_to_string(file).unwrap(),
        )
    });

    map
}
