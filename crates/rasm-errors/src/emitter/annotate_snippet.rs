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

use std::{io, io::Write};

use annotate_snippets::{Group, Level, Renderer, renderer::DecorStyle};
use anstream::AutoStream;
use colorchoice::ColorChoice;

use crate::{diagnostic::RawDiagnostic, emitter::Emitter, severity::Severity};

fn emit_to_destination(rendered: String, dst: &mut Destination) -> io::Result<()> {
    writeln!(dst, "{rendered}")?;
    dst.flush()?;

    Ok(())
}

pub fn stderr_destination() -> Destination {
    let buffer_writer = io::stderr();
    let buffer = Vec::new();
    AutoStream::new(
        Box::new(Buffy {
            buffer_writer,
            buffer,
        }),
        ColorChoice::AlwaysAnsi,
    )
}

struct Buffy {
    buffer_writer: io::Stderr,
    buffer: Vec<u8>,
}

impl Write for Buffy {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buffer_writer.write_all(&self.buffer)?;
        self.buffer.clear();
        Ok(())
    }
}

impl Drop for Buffy {
    fn drop(&mut self) {
        if !self.buffer.is_empty() {
            self.flush().unwrap();
            panic!("buffers need to be flushed in order to print their contents");
        }
    }
}

pub struct AnnotateSnippetEmitter {
    dest: Destination,
}

impl AnnotateSnippetEmitter {
    pub fn new(dest: Destination) -> Self {
        Self { dest }
    }

    // todo: take spans into account
    fn emit_message_default(&mut self, severity: Severity, messages: &[String]) {
        let renderer = self.renderer();
        let level = Level::from(severity);

        let title = level.primary_title(messages.iter().map(ToOwned::to_owned).collect::<String>());

        let mut report = vec![];
        let group = Group::with_title(title);

        report.push(group);

        let string = renderer.render(&report);

        emit_to_destination(string, &mut self.dest)
            .expect("failed to write diagnostic to destination");
    }

    fn renderer(&self) -> Renderer {
        Renderer::styled().decor_style(DecorStyle::Unicode)
    }
}

impl Emitter for AnnotateSnippetEmitter {
    fn emit_diagnostic(&mut self, diagnostic: RawDiagnostic) {
        self.emit_message_default(diagnostic.severity, &diagnostic.messages)
    }
}

pub type Destination = AutoStream<Box<dyn Write + Send>>;
