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

use std::ops::Deref;

use crate::{
    context::DiagnosticContext,
    diagnostic::{RasmDiagnostic, RawDiagnostic},
    severity::Severity,
    traits::{Abort, Diagnostic, ErrorGuarantee, FatalAbort},
};

#[derive(Clone, Copy)]
pub struct DiagnosticContextRef<'a> {
    pub(crate) ctx: &'a DiagnosticContext,
}

impl<'a> DiagnosticContextRef<'a> {
    pub fn emit_bug(self, bug: impl Diagnostic<'a, Abort>) -> ! {
        bug.into_diagnostic(self, Severity::Bug).emit()
    }

    pub fn emit_error(self, error: impl Diagnostic<'a>) -> ErrorGuarantee {
        error.into_diagnostic(self, Severity::Error).emit()
    }

    pub fn emit_note(self, note: impl Diagnostic<'a>) -> ErrorGuarantee {
        note.into_diagnostic(self, Severity::Note).emit()
    }

    pub fn emit_diagnostic(self, diagnostic: RawDiagnostic) -> Option<ErrorGuarantee> {
        self.inner.lock().ok()?.emit_diagnostic(diagnostic)
    }

    pub fn fatal(self, message: impl Into<String>) -> ! {
        RasmDiagnostic::<FatalAbort>::new(self, Severity::Error, message).emit()
    }
}

impl Deref for DiagnosticContextRef<'_> {
    type Target = DiagnosticContext;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}
