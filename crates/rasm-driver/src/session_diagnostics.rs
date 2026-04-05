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
use rasm_errors::context::DiagnosticContextRef;
use rasm_errors::diagnostic::RasmDiagnostic;
use rasm_errors::severity::Severity;
use rasm_errors::traits::{Abort, Diagnostic};

pub(crate) struct InternalAssemblerError;

impl<'a> Diagnostic<'a, Abort> for InternalAssemblerError {
    fn into_diagnostic(self, dcr: DiagnosticContextRef<'a>, severity: Severity) -> RasmDiagnostic<'a, Abort> {
        todo!()
    }
}
