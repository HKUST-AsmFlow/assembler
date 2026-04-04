use rasm_parser::lexer::tokenize;

#[test]
fn lex_comment() {
    let tokens = tokenize("; this is a line comment").collect::<Vec<_>>();
    insta::assert_debug_snapshot!(tokens);
}

#[test]
fn lex_basic_instruction() {
    let tokens = tokenize("adcseq r0, r1, r2, lsl #1").collect::<Vec<_>>();
    insta::assert_debug_snapshot!(tokens);
}
