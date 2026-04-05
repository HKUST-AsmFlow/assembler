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

use crate::{
    context::DiagnosticContextRef,
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
    pub fn emit(self) -> G::Result {
        G::emit(self)
    }

    pub fn emit_with_guarantee(mut self) -> ErrorGuarantee {
        let diag = self.raw;

        self.ctx.emit_diagnostic(diag).unwrap()
    }
}

impl<'diag, G> !Clone for RasmDiagnostic<'diag, G> where G: EmissionProof {}

pub struct RawDiagnostic;
