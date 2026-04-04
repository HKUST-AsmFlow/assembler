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

use std::{collections::HashMap, sync::Arc};

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
}
