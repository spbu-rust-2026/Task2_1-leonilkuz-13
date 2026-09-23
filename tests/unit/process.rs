use super::process;

fn check_process(input: &str, expected_text: &str, expected_word: &str) {
    let mut text = input.to_owned();

    let word = process(&mut text);

    assert_eq!(word, expected_word, "возвращённое первое слово");
    assert_eq!(text, expected_text, "изменённая строка");
}

#[test]
fn sample_sentence() {
    check_process("MATMEX LUCHSHE VSEH", "MATMEX LUCHSHE VSEH!", "MATMEX");
}

#[test]
fn single_word() {
    check_process("Привет", "Привет!", "Привет");
}

#[test]
fn several_words() {
    check_process("Rust спасает дедлайн", "Rust спасает дедлайн!", "Rust");
}

#[test]
fn empty_string() {
    check_process("", "!", "");
}

#[test]
fn whitespace_only_string() {
    check_process(" \t ", " \t !", "");
}

#[test]
fn leading_whitespace() {
    check_process("  Rust учит терпению", "  Rust учит терпению!", "Rust");
}

#[test]
fn trailing_whitespace() {
    check_process("Rust   ", "Rust   !", "Rust");
}

#[test]
fn mixed_whitespace_separators() {
    check_process(
        "первое\tвторое\nтретье",
        "первое\tвторое\nтретье!",
        "первое",
    );
}

#[test]
fn unicode_and_punctuation() {
    check_process("Эрмитаж, откройся", "Эрмитаж, откройся!", "Эрмитаж,");
}

#[test]
fn repeated_calls_append_one_mark_each_time() {
    let mut text = String::from("ещё один шанс");

    {
        let word = process(&mut text);
        assert_eq!(word, "ещё");
    }
    assert_eq!(text, "ещё один шанс!");

    let word = process(&mut text);
    assert_eq!(word, "ещё");
    assert_eq!(text, "ещё один шанс!!");
}
