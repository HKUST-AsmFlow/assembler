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

#![feature(panic_update_hook)]

use std::{panic, panic::PanicHookInfo};

use rasm_errors::{
    ExplicitBug,
    context::DiagnosticContext,
    emitter::annotate_snippet::{AnnotateSnippetEmitter, stderr_destination},
};
use crate::session_diagnostics::InternalAssemblerError;

mod runner;
mod session_diagnostics;

// todo: issue template
pub static DEFAULT_BUG_REPORT_URL: &str = "https://github.com/HKUST-AsmFlow/assembler";

pub fn install_iae_hook(bug_report_url: &'static str, extra_info: fn(&DiagnosticContext)) {
    panic::update_hook(Box::new(
        move |_: &(dyn Fn(&PanicHookInfo) + Send + Sync + 'static), info: &PanicHookInfo| {
            let _ = anstream::stderr().lock();
            report_ice(info, bug_report_url, extra_info);
        },
    ));
}

fn report_ice(info: &PanicHookInfo, bug_report_url: &str, extra_info: fn(&DiagnosticContext)) {
    let dc = DiagnosticContext::new(Box::new(AnnotateSnippetEmitter::new(stderr_destination())));
    let dcr = dc.r#ref();

    if !info.payload().is::<ExplicitBug>() {
        dcr.emit_error(InternalAssemblerError);
    }

    extra_info(&dc);
}

pub fn main() {
    install_iae_hook(DEFAULT_BUG_REPORT_URL, |_| {});
    runner::run_assembler();
}
