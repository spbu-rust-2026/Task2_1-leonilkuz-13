fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    todo!("исправьте заимствования и реализуйте process");
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
