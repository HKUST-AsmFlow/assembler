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

#![feature(default_field_values)]
#![feature(path_absolute_method)]

use std::{path::PathBuf, sync::Arc};

use rasm_ast::nodes::Program;
use rasm_errors::diagnostic::RasmDiagnostic;
use rasm_session::{Session, parse::ParserSession};
use rasm_span::sourcemap::SourceFile;

use crate::{lexer::into_tokenstream, parser::Parser};

pub mod error;
pub mod lexer;
pub mod parser;

fn new_parser_from_file<'a>(
    psess: &'a ParserSession,
    path: &PathBuf,
) -> Result<Parser<'a>, Vec<RasmDiagnostic<'a>>> {
    let sm = psess.source_map();
    let source_file = sm
        .get_file(path.to_string_lossy().to_string())
        .unwrap_or_else(|| {
            let msg = format!("couldn't read `{}`", path.display());
            psess.diagnostic_context().fatal(msg);
        });

    new_parser_from_source_file(psess, source_file)
}

fn new_parser_from_source_file(
    psess: &ParserSession,
    source_file: Arc<SourceFile>,
) -> Result<Parser<'_>, Vec<RasmDiagnostic<'_>>> {
    let stream = into_tokenstream(
        psess,
        source_file.content.as_ref(),
        source_file.start_pos as u32,
    )?;
    Ok(Parser::new(psess, stream))
}

pub fn parse(session: &Session) -> Program {
    let mut parser =
        new_parser_from_file(&session.parser, &session.inputs[0]).unwrap_or_else(|diags| todo!());

    parser.parse_program().unwrap_or_else(|diagnostics| {
        let proof = diagnostics.emit_with_guarantee();
        proof.raise_fatal();
    })
}
