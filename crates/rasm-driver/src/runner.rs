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

use std::panic::{self, AssertUnwindSafe};

use rasm_session::Session;

use crate::args::StructuredOptions;

pub struct Assembler {
    pub session: Session,
}

pub fn run_assembler<R>(config: StructuredOptions, f: impl FnOnce(&Assembler) -> R) -> R {
    let session = rasm_session::build_session(config.input_files());
    let assembler = Assembler { session };

    let res = panic::catch_unwind(AssertUnwindSafe(|| f(&assembler)));
    let res = match res {
        Ok(res) => res,
        Err(err) => panic::resume_unwind(err),
    };

    res
}
