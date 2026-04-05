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

use rasm_errors::context::{DiagnosticContext, DiagnosticContextRef};
use rasm_span::sourcemap::SourceMap;

pub struct ParserSession {
    source_map: SourceMap,
    diagnostic_context: DiagnosticContext,
}

impl ParserSession {
    pub fn with_diagnostic_context(ctxt: DiagnosticContext, source_map: SourceMap) -> Self {
        Self {
            diagnostic_context: ctxt,
            source_map,
        }
    }
}

impl ParserSession {
    #[inline]
    pub fn source_map(&self) -> &SourceMap {
        &self.source_map
    }

    pub fn diagnostic_context(&self) -> DiagnosticContextRef {
        self.diagnostic_context.r#ref()
    }
}
