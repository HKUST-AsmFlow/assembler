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

use std::panic;

use crate::{
    ExplicitBug, context::DiagnosticContextRef, diagnostic::RasmDiagnostic, severity::Severity,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Abort;

impl EmissionProof for Abort {
    type Result = !;

    fn emit(diagnostic: RasmDiagnostic<Self>) -> Self::Result {
        diagnostic.emit_with_guarantee();
        panic::panic_any(ExplicitBug)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorGuarantee(());

impl ErrorGuarantee {
    pub fn new_unchecked() -> Self {
        Self(())
    }
}

impl EmissionProof for ErrorGuarantee {
    fn emit(diagnostic: RasmDiagnostic<Self>) -> Self::Result {
        diagnostic.emit_with_guarantee()
    }
}

pub trait Diagnostic<'a, G: EmissionProof = ErrorGuarantee> {
    #[must_use]
    fn into_diagnostic(
        self,
        dcr: DiagnosticContextRef<'a>,
        severity: Severity,
    ) -> RasmDiagnostic<'a, G>;
}

pub trait EmissionProof: Sized {
    type Result = Self;

    fn emit(diagnostic: RasmDiagnostic<Self>) -> Self::Result;
}
