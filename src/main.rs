fn main() {
    let mut string = String::from("  \t  ");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    let mut start_idx: Option<usize> = None;
    let mut end_idx: Option<usize> = None;

    for (i, symb) in _text.char_indices() {
        if !symb.is_whitespace() {
            if start_idx.is_none() {
                start_idx = Some(i);
            }
        } else {
            if start_idx.is_some() && end_idx.is_none() {
                end_idx = Some(i);
                break;
            }
        }
    }

    if start_idx.is_some() && end_idx.is_none() {
        end_idx = Some(_text.len());
    }

    _text.push('!');

    match (start_idx, end_idx) {
        (Some(start), Some(end)) => &_text[start..end],
        _ => "",
    }
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
