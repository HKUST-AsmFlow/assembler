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

use std::{env, panic, panic::PanicHookInfo};

use getopts::{Fail, Options};
use rasm_errors::{
    ExplicitBug,
    context::DiagnosticContext,
    emitter::annotate_snippet::{AnnotateSnippetEmitter, stderr_destination},
};
use rasm_session::early::EarlyDiagnosticContext;

use crate::{
    args::{StructuredOptions, rasm_optgroups},
    session_diagnostics::{IaeBugReportUrl, InternalAssemblerError},
};

mod args;
mod runner;
mod session_diagnostics;

// todo: issue template
pub static DEFAULT_BUG_REPORT_URL: &str = "https://github.com/HKUST-AsmFlow/assembler";

fn install_iae_hook(bug_report_url: &'static str, extra_info: fn(&DiagnosticContext)) {
    panic::update_hook(Box::new(
        move |default_hook: &(dyn Fn(&PanicHookInfo) + Send + Sync + 'static),
              info: &PanicHookInfo| {
            let _ = anstream::stderr().lock();

            default_hook(info);
            report_iae(info, bug_report_url, extra_info);
        },
    ));
}

fn handle_options(options: &Options, dcx: &EarlyDiagnosticContext) -> StructuredOptions {
    let matches = options.parse(env::args()).unwrap_or_else(|fail| {
        let msg = match fail {
            Fail::ArgumentMissing(arg) => format!("required argument `{arg}` is missing"),
            Fail::OptionMissing(arg) => format!("required option `{arg}` is migging"),
            Fail::OptionDuplicated(arg) => format!("found duplicate argument `{arg}`"),
            Fail::UnrecognizedOption(arg) => format!("found unrecognized option `{arg}`"),
            Fail::UnexpectedArgument(arg) => format!("unexpected argument for argument `{arg}`"),
        };

        dcx.fatal(msg);
    });

    StructuredOptions::from_matches(matches)
}

fn report_iae(info: &PanicHookInfo, bug_report_url: &str, extra_info: fn(&DiagnosticContext)) {
    let dc = DiagnosticContext::new(Box::new(AnnotateSnippetEmitter::new(stderr_destination())));
    let dcr = dc.r#ref();

    if !info.payload().is::<ExplicitBug>() {
        dcr.emit_error(InternalAssemblerError);
    }

    dcr.emit_note(IaeBugReportUrl {
        url: bug_report_url,
    });

    extra_info(&dc);
}

pub fn main() {
    let early_dcx = EarlyDiagnosticContext::new();

    let options = rasm_optgroups();
    let config = handle_options(&options, &early_dcx);

    if config.input_files().is_empty() {
        early_dcx.fatal("no input files");
    }

    install_iae_hook(DEFAULT_BUG_REPORT_URL, |_| {});
    runner::run_assembler(config, |assembler| {
        let session = &assembler.session;
        let _ = rasm_parser::parse(session);
    });
}
