use rasm_parser::lexer::tokenize;
use rasm_span::sourcemap::SourceMap;

#[test]
fn lex_comment() {
    let mut source_map = SourceMap::new();
    let tokens =
        tokenize("<input>", "; this is a line comment", &mut source_map).collect::<Vec<_>>();
    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn lex_basic_instruction() {
    let mut source_map = SourceMap::new();
    let tokens =
        tokenize("<input>", "adcseq r0, r1, r2, lsl #1", &mut source_map).collect::<Vec<_>>();
    insta::assert_debug_snapshot!(tokens);
}
