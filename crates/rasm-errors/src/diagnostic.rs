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

use std::marker::PhantomData;

use rasm_span::Span;

use crate::{
    context::DiagnosticContextRef,
    severity::Severity,
    traits::{EmissionProof, ErrorGuarantee},
};

pub struct RasmDiagnostic<'diag, G: EmissionProof = ErrorGuarantee> {
    ctx: DiagnosticContextRef<'diag>,
    raw: RawDiagnostic,
    phantom: PhantomData<G>,
}

impl<'diag, G> RasmDiagnostic<'diag, G>
where
    G: EmissionProof,
{
    pub fn new(
        ctx: DiagnosticContextRef<'diag>,
        severity: Severity,
        message: impl Into<String>,
    ) -> Self {
        let mut raw = RawDiagnostic::new(severity);
        raw.add_message(message);

        Self {
            ctx,
            raw,
            phantom: PhantomData,
        }
    }

    pub fn new_empty(ctx: DiagnosticContextRef<'diag>, severity: Severity) -> Self {
        Self {
            ctx,
            raw: RawDiagnostic::new(severity),
            phantom: PhantomData,
        }
    }

    pub fn add_message(&mut self, message: impl Into<String>) {
        self.raw.add_message(message);
    }

    pub fn emit(self) -> G::Result {
        G::emit(self)
    }

    pub fn emit_with_guarantee(self) -> ErrorGuarantee {
        let diag = self.raw;

        self.ctx.emit_diagnostic(diag).unwrap()
    }

    pub fn span(&mut self, span: Span) {
        self.raw.span = span;
    }
}

impl<'diag, G> !Clone for RasmDiagnostic<'diag, G> where G: EmissionProof {}

pub struct RawDiagnostic {
    pub(crate) severity: Severity,
    pub messages: Vec<String>,
    pub span: Span,
}

impl RawDiagnostic {
    pub fn new(severity: Severity) -> Self {
        Self {
            severity,
            messages: vec![],
            span: Span::DUMMY_SPAN,
        }
    }

    pub fn add_message(&mut self, message: impl Into<String>) {
        self.messages.push(message.into());
    }
}
